#![allow(dead_code)]

pub mod linux;
pub mod macos;

use crate::errors::Result;
use crate::domain::services::ServiceEngine;

pub trait PlatformAbstraction: Send + Sync {
    // Phase 0: Just a placeholder for platform-specific methods
    fn get_platform_name(&self) -> String;
}

pub struct PlatformManager {
    // inner platform implementation
}

impl PlatformManager {
    pub fn new() -> Result<Self> {
        // Here we would conditionally compile or detect the platform
        Ok(Self {})
    }

    pub fn create_service_engine(&self) -> Box<dyn ServiceEngine> {
        #[cfg(target_os = "macos")]
        return Box::new(macos::services::LaunchdEngine::new());
        
        #[cfg(target_os = "linux")]
        return Box::new(linux::services::SystemdEngine::new());
        
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        panic!("Unsupported platform for services");
    }
}

impl PlatformAbstraction for PlatformManager {
    fn get_platform_name(&self) -> String {
        #[cfg(target_os = "macos")]
        return "macOS".to_string();

        #[cfg(target_os = "linux")]
        return "Linux".to_string();

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return "Unsupported".to_string();
    }
}
