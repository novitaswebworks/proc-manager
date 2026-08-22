use sysinfo::{Pid, ProcessStatus};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub name: String,
    pub exe: String,
    pub cmd: Vec<String>,
    pub cpu_usage: f32,
    pub memory: u64,
    pub virtual_memory: u64,
    pub status: ProcessStatus,
    pub start_time: u64,
    pub run_time: u64,
    pub user_id: Option<String>,
    pub cpu_history: Vec<u64>,
    pub memory_history: Vec<u64>,
}
