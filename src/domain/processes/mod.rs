pub mod models;

use models::ProcessInfo;
use sysinfo::{Pid, System, Users};
use crate::errors::Result;

pub struct ProcessManager {
    system: System,
    users: Users,
    cpu_history: std::collections::HashMap<Pid, std::collections::VecDeque<u64>>,
    memory_history: std::collections::HashMap<Pid, std::collections::VecDeque<u64>>,
    max_history: usize,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            users: Users::new_with_refreshed_list(),
            cpu_history: std::collections::HashMap::new(),
            memory_history: std::collections::HashMap::new(),
            max_history: 100, // store up to 100 ticks for sparklines
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
        // Update history
        let mut active_pids = std::collections::HashSet::new();
        for (pid, proc) in self.system.processes() {
            active_pids.insert(*pid);
            
            let cpu = self.cpu_history.entry(*pid).or_default();
            cpu.push_back((proc.cpu_usage() * 100.0) as u64); // scale up for sparkline resolution
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

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
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
                }
            })
            .collect()
    }

    pub fn kill_process(&self, pid: Pid) -> Result<()> {
        if let Some(proc) = self.system.process(pid) {
            proc.kill();
            Ok(())
        } else {
            Err(crate::errors::AppError::PlatformError(format!("Process {} not found", pid)))
        }
    }
}
