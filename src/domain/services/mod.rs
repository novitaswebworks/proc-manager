pub mod models;

use crate::errors::Result;
use models::ServiceInfo;

pub trait ServiceEngine: Send + Sync {
    fn refresh(&mut self) -> Result<()>;
    fn get_services(&self) -> Vec<ServiceInfo>;
    fn start_service(&self, name: &str) -> Result<()>;
    fn stop_service(&self, name: &str) -> Result<()>;
    fn restart_service(&self, name: &str) -> Result<()>;
    fn enable_service(&self, name: &str) -> Result<()>;
    fn disable_service(&self, name: &str) -> Result<()>;
    fn get_service_logs(&self, name: &str, lines: usize) -> Result<Vec<String>>;
}

pub struct ServiceManager {
    engine: Box<dyn ServiceEngine>,
}

impl ServiceManager {
    pub fn new(engine: Box<dyn ServiceEngine>) -> Self {
        Self { engine }
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.engine.refresh()
    }

    pub fn get_services(&self) -> Vec<ServiceInfo> {
        self.engine.get_services()
    }

    pub fn start_service(&self, name: &str) -> Result<()> {
        self.engine.start_service(name)
    }

    pub fn stop_service(&self, name: &str) -> Result<()> {
        self.engine.stop_service(name)
    }

    pub fn restart_service(&self, name: &str) -> Result<()> {
        self.engine.restart_service(name)
    }

    pub fn enable_service(&self, name: &str) -> Result<()> {
        self.engine.enable_service(name)
    }

    pub fn disable_service(&self, name: &str) -> Result<()> {
        self.engine.disable_service(name)
    }

    pub fn get_service_logs(&self, name: &str, lines: usize) -> Result<Vec<String>> {
        self.engine.get_service_logs(name, lines)
    }
}
