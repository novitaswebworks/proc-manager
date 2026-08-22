#![allow(dead_code)]

use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum AppEvent {
    // Process Events
    ProcessStarted(u32),
    ProcessStopped(u32),
    ProcessKilled(u32),

    // UI Events
    Tick,
    Quit,
    // Future events
    // ContainerStarted(String),
    // ServiceStarted(String),
    // WorkspaceDetected(String),
    // SnapshotCreated(String),
    // HealthIssueDetected(String),
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }

    pub fn send(&self, event: AppEvent) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        self.sender.send(event)
    }
}
