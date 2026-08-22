use crate::domain::services::models::{ServiceInfo, ServiceStatus};
use crate::domain::services::ServiceEngine;
use crate::errors::{AppError, Result};
use std::process::Command;

pub struct LaunchdEngine {
    services: Vec<ServiceInfo>,
}

impl LaunchdEngine {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
}

impl ServiceEngine for LaunchdEngine {
    fn refresh(&mut self) -> Result<()> {
        let output = Command::new("launchctl")
            .arg("list")
            .output()
            .map_err(|e| AppError::PlatformError(format!("Failed to execute launchctl: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut new_services = Vec::new();

        for line in stdout.lines().skip(1) { // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let pid = parts[0];
                let status = parts[1];
                let name = parts[2];

                let service_status = if pid != "-" {
                    ServiceStatus::Running
                } else if status != "0" {
                    ServiceStatus::Failed
                } else {
                    ServiceStatus::Stopped
                };

                new_services.push(ServiceInfo {
                    name: name.to_string(),
                    description: format!("launchd service: {}", name),
                    status: service_status,
                    is_enabled: true, // launchd doesn't easily expose this in `list`
                });
            }
        }

        self.services = new_services;
        Ok(())
    }

    fn get_services(&self) -> Vec<ServiceInfo> {
        self.services.clone()
    }

    fn start_service(&self, name: &str) -> Result<()> {
        Command::new("launchctl")
            .args(["start", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to start service: {}", e)))?;
        Ok(())
    }

    fn stop_service(&self, name: &str) -> Result<()> {
        Command::new("launchctl")
            .args(["stop", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to stop service: {}", e)))?;
        Ok(())
    }

    fn restart_service(&self, name: &str) -> Result<()> {
        self.stop_service(name)?;
        self.start_service(name)
    }

    fn enable_service(&self, _name: &str) -> Result<()> {
        Err(AppError::PlatformError("Enable not supported for launchctl via simple name".to_string()))
    }

    fn disable_service(&self, _name: &str) -> Result<()> {
        Err(AppError::PlatformError("Disable not supported for launchctl via simple name".to_string()))
    }

    fn get_service_logs(&self, _name: &str, _lines: usize) -> Result<Vec<String>> {
        Ok(vec!["Log viewing for launchd services is not supported yet.".to_string()])
    }
}
