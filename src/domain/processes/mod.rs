pub mod models;
pub mod system_metrics;

use models::ProcessInfo;
use sysinfo::{Pid, System, Users};
use crate::errors::Result;

pub struct ProcessManager {
    system: System,
    users: Users,
    cpu_history: std::collections::HashMap<Pid, std::collections::VecDeque<u64>>,
    memory_history: std::collections::HashMap<Pid, std::collections::VecDeque<u64>>,
    max_history: usize,
    remote_processes: Option<Vec<ProcessInfo>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            users: Users::new_with_refreshed_list(),
            cpu_history: std::collections::HashMap::new(),
            memory_history: std::collections::HashMap::new(),
            max_history: 100, // store up to 100 ticks for sparklines
            remote_processes: None,
        }
    }

    pub fn refresh(&mut self, ssh_manager: Option<&crate::infrastructure::ssh_manager::SshManager>) {
        if let Some(ssh) = ssh_manager {
            if ssh.is_connected() {
                self.refresh_remote(ssh);
                return;
            }
        }
        self.remote_processes = None;
        self.system.refresh_all();
        // Update history
        let mut active_pids = std::collections::HashSet::new();
        for (pid, proc) in self.system.processes() {
            active_pids.insert(*pid);
            
            let cpu = self.cpu_history.entry(*pid).or_default();
            cpu.push_back((proc.cpu_usage() ) as u64); // scale up for sparkline resolution
            if cpu.len() > self.max_history {
                cpu.pop_front();
            }

            let mem = self.memory_history.entry(*pid).or_default();
            // Store in MB for sparkline
            mem.push_back(proc.memory() / 1024 / 1024);
            if mem.len() > self.max_history {
                mem.pop_front();
            }
        }
        
        // Cleanup dead processes
        self.cpu_history.retain(|pid, _| active_pids.contains(pid));
        self.memory_history.retain(|pid, _| active_pids.contains(pid));
    }

    fn refresh_remote(&mut self, ssh: &crate::infrastructure::ssh_manager::SshManager) {
        if let Ok(output) = ssh.execute_command("ps axo pid,ppid,pcpu,rss,user,comm") {
            let mut processes = Vec::new();
            let mut active_pids = std::collections::HashSet::new();
            
            for line in output.lines().skip(1) { // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    if let Ok(pid_val) = parts[0].parse::<usize>() {
                        let pid = Pid::from(pid_val);
                        active_pids.insert(pid);
                        let parent = parts[1].parse::<usize>().ok().map(Pid::from);
                        let pcpu = parts[2].parse::<f32>().unwrap_or(0.0);
                        let rss = parts[3].parse::<u64>().unwrap_or(0) * 1024; // KB to Bytes
                        let user = parts[4].to_string();
                        let comm = parts[5..].join(" ");
                        
                        let cpu = self.cpu_history.entry(pid).or_default();
                        cpu.push_back((pcpu ) as u64);
                        if cpu.len() > self.max_history { cpu.pop_front(); }
                        
                        let mem = self.memory_history.entry(pid).or_default();
                        mem.push_back(rss / 1024 / 1024);
                        if mem.len() > self.max_history { mem.pop_front(); }
                        
                        processes.push(ProcessInfo {
                            pid,
                            parent,
                            name: comm.clone(),
                            exe: comm,
                            cmd: vec![],
                            cpu_usage: pcpu,
                            memory: rss,
                            virtual_memory: rss,
                            status: sysinfo::ProcessStatus::Run,
                            start_time: 0,
                            run_time: 0,
                            user_id: Some(user),
                            cpu_history: cpu.iter().copied().collect(),
                            memory_history: mem.iter().copied().collect(),
                            tree_depth: 0,
                        });
                    }
                }
            }
            
            self.cpu_history.retain(|pid, _| active_pids.contains(pid));
            self.memory_history.retain(|pid, _| active_pids.contains(pid));
            self.remote_processes = Some(processes);
        } else {
            self.remote_processes = None;
        }
    }

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        if let Some(remote) = &self.remote_processes {
            return remote.clone();
        }

        self.system
            .processes()
            .iter()
            .map(|(pid, proc)| {
                let user_id = proc.user_id().map(|uid| {
                    self.users.get_user_by_id(uid)
                        .map(|u| u.name().to_string())
                        .unwrap_or_else(|| uid.to_string())
                });

                let cpu_hist = self.cpu_history.get(pid).map(|d| d.iter().copied().collect()).unwrap_or_default();
                let mem_hist = self.memory_history.get(pid).map(|d| d.iter().copied().collect()).unwrap_or_default();

                ProcessInfo {
                    pid: *pid,
                    parent: proc.parent(),
                    name: proc.name().to_string_lossy().to_string(),
                    exe: proc.exe().map(|p| p.to_string_lossy().to_string()).unwrap_or_default(),
                    cmd: proc.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                    cpu_usage: proc.cpu_usage(),
                    memory: proc.memory(),
                    virtual_memory: proc.virtual_memory(),
                    status: proc.status(),
                    start_time: proc.start_time(),
                    run_time: proc.run_time(),
                    user_id,
                    cpu_history: cpu_hist,
                    memory_history: mem_hist,
                    tree_depth: 0,
                }
            })
            .collect()
    }

    pub fn get_system_metrics(&self) -> system_metrics::SystemMetrics {
        system_metrics::SystemMetrics {
            cpu_usage: self.system.global_cpu_usage(),
            used_memory: self.system.used_memory(),
            total_memory: self.system.total_memory(),
            used_swap: self.system.used_swap(),
            total_swap: self.system.total_swap(),
        }
    }

    pub fn kill_process(&self, pid: Pid, ssh_manager: Option<&crate::infrastructure::ssh_manager::SshManager>) -> Result<()> {
        if let Some(ssh) = ssh_manager {
            if ssh.is_connected() {
                ssh.execute_command(&format!("kill -9 '{}'", pid.as_u32()))
                    .map_err(|e| crate::errors::AppError::PlatformError(format!("Remote kill failed: {}", e)))?;
                return Ok(());
            }
        }
        
        if let Some(proc) = self.system.process(pid) {
            proc.kill();
            Ok(())
        } else {
            Err(crate::errors::AppError::PlatformError(format!("Process {} not found", pid)))
        }
    }
}
