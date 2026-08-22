pub mod models;

use models::{ContainerInfo, ContainerStatus};
use bollard::Docker;
use crate::errors::{AppError, Result};
use std::sync::Arc;

#[derive(Clone)]
pub struct DockerManager {
    docker: Option<Arc<Docker>>,
    containers: Vec<ContainerInfo>,
}

impl DockerManager {
    pub fn new() -> Self {
        let docker = match Docker::connect_with_local_defaults() {
            Ok(d) => Some(Arc::new(d)),
            Err(e) => {
                tracing::warn!("Failed to connect to docker daemon: {}", e);
                None
            }
        };

        Self {
            docker,
            containers: Vec::new(),
        }
    }

    pub async fn refresh(&mut self) {
        if let Some(docker) = &self.docker {
            let options = Some(bollard::query_parameters::ListContainersOptions {
                all: true,
                ..Default::default()
            });

            if let Ok(containers) = docker.list_containers(options).await {
                self.containers = containers.into_iter().map(|c| {
                    let name = c.names.unwrap_or_default().join(", ").replace("/", "");
                    
                    let state_str = c.state.map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default();
                    let status = if state_str.contains("running") {
                        ContainerStatus::Running
                    } else if state_str.contains("exited") {
                        ContainerStatus::Exited
                    } else if state_str.contains("paused") {
                        ContainerStatus::Paused
                    } else if state_str.contains("restarting") {
                        ContainerStatus::Restarting
                    } else if state_str.contains("dead") {
                        ContainerStatus::Dead
                    } else if state_str.contains("created") {
                        ContainerStatus::Created
                    } else if state_str.contains("removing") {
                        ContainerStatus::Removing
                    } else {
                        ContainerStatus::Unknown
                    };

                    let ports = c.ports.unwrap_or_default().iter().map(|p| {
                        let ip = p.ip.clone().unwrap_or_default();
                        let public = p.public_port.map(|port| port.to_string()).unwrap_or_default();
                        let private = p.private_port;
                        let typ_str = p.typ.as_ref().map(|t| format!("{:?}", t).to_lowercase()).unwrap_or_default();
                        if public.is_empty() {
                            format!("{}/{}", private, typ_str)
                        } else {
                            format!("{}:{}->{}/{}", ip, public, private, typ_str)
                        }
                    }).collect::<Vec<_>>().join(", ");

                    ContainerInfo {
                        id: c.id.unwrap_or_default().chars().take(12).collect(),
                        name,
                        image: c.image.unwrap_or_default(),
                        status,
                        state_string: c.status.unwrap_or_default(),
                        ports,
                    }
                }).collect();
            }
        }
    }

    pub fn get_containers(&self) -> Vec<ContainerInfo> {
        self.containers.clone()
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        if let Some(docker) = &self.docker {
            docker.start_container(id, None)
                .await
                .map_err(|e| AppError::PlatformError(format!("Failed to start container: {}", e)))?;
        }
        Ok(())
    }

    pub async fn stop_container(&self, id: &str) -> Result<()> {
        if let Some(docker) = &self.docker {
            docker.stop_container(id, None)
                .await
                .map_err(|e| AppError::PlatformError(format!("Failed to stop container: {}", e)))?;
        }
        Ok(())
    }

    pub async fn restart_container(&self, id: &str) -> Result<()> {
        if let Some(docker) = &self.docker {
            docker.restart_container(id, None)
                .await
                .map_err(|e| AppError::PlatformError(format!("Failed to restart container: {}", e)))?;
        }
        Ok(())
    }

    pub async fn get_container_logs(&self, id: &str, lines: usize) -> Result<Vec<String>> {
        if let Some(docker) = &self.docker {
            use bollard::query_parameters::LogsOptions;
            use futures_util::StreamExt;
            
            let options = Some(LogsOptions {
                stdout: true,
                stderr: true,
                tail: lines.to_string(),
                ..Default::default()
            });

            let mut logs_stream = docker.logs(id, options);
            let mut logs = Vec::new();

            while let Some(log_result) = logs_stream.next().await {
                match log_result {
                    Ok(log) => {
                        let log_str = log.to_string();
                        // Split by newlines just in case a chunk has multiple
                        for line in log_str.lines() {
                            logs.push(line.to_string());
                        }
                    }
                    Err(e) => {
                        logs.push(format!("Error reading log: {}", e));
                    }
                }
            }
            Ok(logs)
        } else {
            Ok(vec!["Docker is not available".to_string()])
        }
    }
}
