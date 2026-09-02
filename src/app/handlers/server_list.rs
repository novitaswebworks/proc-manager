use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub fn handle_server_list_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let len = self.servers.len();
                self.server_list_state.next(len);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let len = self.servers.len();
                self.server_list_state.previous(len);
            }
            KeyCode::Enter => {
                if let Some(i) = self.server_list_state.state.selected() {
                    if let Some(server) = self.servers.get_mut(i) {
                        server.status = "Connecting...".to_string();
                        // Get the corresponding ServerConfig
                        if let Some(ref srv_configs) = self.config.servers {
                            if let Some(config) = srv_configs.iter().find(|c| c.name == server.name) {
                                match self.ssh_manager.connect(config) {
                                    Ok(_) => {
                                        server.status = "Connected".to_string();
                                        self.notification = Some(format!("Connected to {}", config.name));
                                        self.notification_expiry = Some(std::time::Instant::now() + std::time::Duration::from_secs(5));
                                        // Switch to ProcessList view
                                        self.active_screen = crate::app::ActiveScreen::ProcessList;
                                    },
                                    Err(e) => {
                                        server.status = "Error".to_string();
                                        self.notification = Some(format!("SSH Error: {}", e));
                                        self.notification_expiry = Some(std::time::Instant::now() + std::time::Duration::from_secs(5));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
