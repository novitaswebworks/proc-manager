use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) async fn handle_log_view_events(&mut self, key: KeyEvent) {
        if self.log_view_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.log_view_state.is_searching = false;
                }
                KeyCode::Char(c) => {
                    self.log_view_state.search_query.push(c);
                }
                KeyCode::Backspace => {
                    self.log_view_state.search_query.pop();
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('/') => {
                    self.log_view_state.is_searching = true;
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace => {
                    self.active_screen = self.previous_screen;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.log_view_state.scroll_up();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let max = self.log_view_state.logs.len() as u16;
                    self.log_view_state.scroll_down(max);
                }
                KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Char('r') => {
                    if let Some(container_id) = self.log_view_state.container_id.clone() {
                        if let Ok(logs) = self.docker_manager.get_container_logs(&container_id, 100).await {
                            self.log_view_state.set_logs(logs);
                            self.notify("Refreshed container logs");
                        }
                    } else if let Some(service_name) = self.log_view_state.service_name.clone() {
                        if let Ok(logs) = self.service_manager.get_service_logs(&service_name, 100) {
                            self.log_view_state.set_logs(logs);
                            self.notify("Refreshed service logs");
                        }
                    }
                }
                KeyCode::Char('F') | KeyCode::Char('f') => {
                    self.log_view_state.auto_scroll = !self.log_view_state.auto_scroll;
                    let msg = if self.log_view_state.auto_scroll { "Auto-scroll: ON" } else { "Auto-scroll: OFF" };
                    self.notify(msg);
                }
                KeyCode::Char('C') | KeyCode::Char('c') => {
                    self.log_view_state.logs.clear();
                    self.notify("Log buffer cleared");
                }
                KeyCode::Char('E') | KeyCode::Char('e') => {
                    let filename = format!("{}.log", self.log_view_state.title.replace(" ", "_"));
                    let path = if let Some(user_dirs) = directories::UserDirs::new() {
                        if let Some(desktop) = user_dirs.desktop_dir() {
                            desktop.join(&filename)
                        } else {
                            std::path::PathBuf::from(&filename)
                        }
                    } else {
                        std::path::PathBuf::from(&filename)
                    };
                    
                    if let Ok(mut file) = std::fs::File::create(&path) {
                        use std::io::Write;
                        let content = self.log_view_state.logs.join("\n");
                        if let Ok(_) = file.write_all(content.as_bytes()) {
                            self.notify(format!("Exported logs to {}", path.display()));
                        } else {
                            self.notify("Failed to write log file");
                        }
                    } else {
                        self.notify("Failed to create log file");
                    }
                }
                _ => {}
            }
        }
    }
}
