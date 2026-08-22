use crate::domain::services::models::{ServiceInfo, ServiceStatus};
use crate::domain::services::ServiceEngine;
use crate::errors::{AppError, Result};
use std::process::Command;

pub struct SystemdEngine {
    services: Vec<ServiceInfo>,
}

impl SystemdEngine {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
}

impl ServiceEngine for SystemdEngine {
    fn refresh(&mut self) -> Result<()> {
        let output = Command::new("systemctl")
            .args(["list-units", "--type=service", "--all", "--no-pager", "--no-legend"])
            .output()
            .map_err(|e| AppError::PlatformError(format!("Failed to execute systemctl: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut new_services = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let name = parts[0].replace(".service", "");
                let load = parts[1]; // loaded
                let active = parts[2]; // active, inactive, failed
                let sub = parts[3]; // running, exited, dead
                let desc = parts[4..].join(" ");

                let status = match (active, sub) {
                    ("active", "running") => ServiceStatus::Running,
                    ("active", _) => ServiceStatus::Running,
                    ("inactive", _) => ServiceStatus::Stopped,
                    ("failed", _) => ServiceStatus::Failed,
                    ("reloading", _) => ServiceStatus::Restarting,
                    _ => ServiceStatus::Unknown,
                };

                new_services.push(ServiceInfo {
                    name,
                    description: desc,
                    status,
                    is_enabled: load == "loaded", // Basic approximation
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
        Command::new("sudo")
            .args(["systemctl", "start", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to start service: {}", e)))?;
        Ok(())
    }

    fn stop_service(&self, name: &str) -> Result<()> {
        Command::new("sudo")
            .args(["systemctl", "stop", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to stop service: {}", e)))?;
        Ok(())
    }

    fn restart_service(&self, name: &str) -> Result<()> {
        Command::new("sudo")
            .args(["systemctl", "restart", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to restart service: {}", e)))?;
        Ok(())
    }

    fn enable_service(&self, name: &str) -> Result<()> {
        Command::new("sudo")
            .args(["systemctl", "enable", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to enable service: {}", e)))?;
        Ok(())
    }

    fn disable_service(&self, name: &str) -> Result<()> {
        Command::new("systemctl")
            .args(["disable", name])
            .status()
            .map_err(|e| AppError::PlatformError(format!("Failed to disable service: {}", e)))?;
        Ok(())
    }

    fn get_service_logs(&self, name: &str, lines: usize) -> Result<Vec<String>> {
        let output = Command::new("journalctl")
            .args(["-u", name, "-n", &lines.to_string(), "--no-pager"])
            .output()
            .map_err(|e| AppError::PlatformError(format!("Failed to get logs: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|s| s.to_string()).collect())
    }
}
