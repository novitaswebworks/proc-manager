use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use crate::errors::{Result, AppError};
use crate::configuration::ServerConfig;

pub struct SshManager {
    session: Option<Session>,
}

impl SshManager {
    pub fn new() -> Self {
        Self { session: None }
    }

    pub fn connect(&mut self, config: &ServerConfig) -> Result<()> {
        let addr = if config.address.contains(':') {
            config.address.clone()
        } else {
            format!("{}:22", config.address)
        };

        let tcp = TcpStream::connect(&addr)
            .map_err(|e| anyhow::anyhow!("Failed to connect to TCP stream ({}): {}", addr, e))?;
        
        let mut sess = Session::new()
            .map_err(|e| anyhow::anyhow!("Failed to initialize SSH session: {}", e))?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| anyhow::anyhow!("SSH Handshake failed: {}", e))?;

        if let Some(key_path) = &config.ssh_key {
            let expanded_path = if key_path.starts_with("~/") {
                if let Some(base_dirs) = directories::BaseDirs::new() {
                    base_dirs.home_dir().join(&key_path[2..])
                } else {
                    std::path::PathBuf::from(key_path)
                }
            } else if key_path.starts_with("/Downloads/") {
                // Handle common root-relative typo
                if let Some(base_dirs) = directories::BaseDirs::new() {
                    base_dirs.home_dir().join(&key_path[1..])
                } else {
                    std::path::PathBuf::from(key_path)
                }
            } else {
                std::path::PathBuf::from(key_path)
            };

            if !expanded_path.exists() {
                return Err(anyhow::anyhow!("SSH key file not found at: {}", expanded_path.display()).into());
            }

            sess.userauth_pubkey_file(
                &config.user,
                None,
                &expanded_path,
                None,
            ).map_err(|e| anyhow::anyhow!("Public key auth failed for {}: {}", expanded_path.display(), e))?;
        } else {
            // Try to use the ssh-agent if no explicit key is provided
            sess.userauth_agent(&config.user).map_err(|e| anyhow::anyhow!("Agent authentication failed: {}", e))?;
        }

        if !sess.authenticated() {
            return Err(anyhow::anyhow!("Authentication failed for server {}", config.name).into());
        }

        self.session = Some(sess);
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    pub fn execute_command(&self, command: &str) -> Result<String> {
        let sess = self.session.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No active SSH session"))?;

        let cmd = command.to_string();
        tokio::task::block_in_place(move || {
            let mut channel = sess.channel_session()
                .map_err(|e| anyhow::anyhow!("Failed to open channel: {}", e))?;
            channel.exec(&cmd)
                .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

            let mut s = String::new();
            use std::io::Read;
            channel.read_to_string(&mut s)
                .map_err(|e| anyhow::anyhow!("Failed to read command output: {}", e))?;
            channel.wait_close()
                .map_err(|e| anyhow::anyhow!("Failed to close channel: {}", e))?;
            
            if let Ok(exit_status) = channel.exit_status() {
                if exit_status != 0 {
                    return Err(anyhow::anyhow!("Command exited with status {}: {}", exit_status, s).into());
                }
            }
            Ok(s)
        })
    }

    pub fn disconnect(&mut self) {
        self.session = None;
    }
}

impl Drop for SshManager {
    fn drop(&mut self) {
        if let Some(mut sess) = self.session.take() {
            let _ = sess.disconnect(None, "Application closing", None);
        }
    }
}
