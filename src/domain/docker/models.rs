#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Paused,
    Restarting,
    Dead,
    Created,
    Exited,
    Removing,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub state_string: String,
    pub ports: String,
}
