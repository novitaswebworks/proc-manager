#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceItemType {
    Process,
    Service,
    Container,
}

impl WorkspaceItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceItemType::Process => "process",
            WorkspaceItemType::Service => "service",
            WorkspaceItemType::Container => "container",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "process" => Some(WorkspaceItemType::Process),
            "service" => Some(WorkspaceItemType::Service),
            "container" => Some(WorkspaceItemType::Container),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceItem {
    pub id: i64,
    pub item_type: WorkspaceItemType,
    pub item_name: String,
}

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<WorkspaceItem>,
}
