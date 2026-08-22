#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Failed,
    Restarting,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub status: ServiceStatus,
    pub is_enabled: bool,
}
