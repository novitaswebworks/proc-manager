use crate::app::{App, ActiveScreen};
use crate::ui::screens::log_view::LogViewState;
use crate::domain::workspaces::models::WorkspaceItemType;
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) async fn handle_docker_list_events(&mut self, key: KeyEvent) {
        if self.docker_list_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => self.docker_list_state.is_searching = false,
                KeyCode::Char(c) => { self.docker_list_state.search_query.push(c); self.refresh_docker_list(); },
                KeyCode::Backspace => { self.docker_list_state.search_query.pop(); self.refresh_docker_list(); },
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.active_screen = ActiveScreen::ProcessList,
                KeyCode::Char('/') => self.docker_list_state.is_searching = true,
                KeyCode::Down | KeyCode::Char('j') => self.docker_list_state.next(self.containers.len()),
                KeyCode::Up | KeyCode::Char('k') => self.docker_list_state.previous(self.containers.len()),
                KeyCode::Char('S') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i) { let _ = self.docker_manager.start_container(&c.name).await; self.notify(format!("Starting container {}", c.name)); }
                    }
                }
                KeyCode::Char('K') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i) { let _ = self.docker_manager.stop_container(&c.name).await; self.notify(format!("Stopping container {}", c.name)); }
                    }
                }
                KeyCode::Char('R') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i) { let _ = self.docker_manager.restart_container(&c.name).await; self.notify(format!("Restarting container {}", c.name)); }
                    }
                }
                KeyCode::Char('L') | KeyCode::Char('l') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i).cloned() {
                            self.log_view_state = LogViewState::new();
                            self.log_view_state.container_id = Some(c.id.clone());
                            self.log_view_state.title = c.name.clone();
                            if let Ok(logs) = self.docker_manager.get_container_logs(&c.id, 100).await {
                                self.log_view_state.set_logs(logs);
                            } else {
                                self.log_view_state.set_logs(vec!["Could not fetch logs or not supported.".to_string()]);
                            }
                            self.previous_screen = self.active_screen;
                            self.active_screen = ActiveScreen::LogView;
                        }
                    }
                }
                KeyCode::Char('W') | KeyCode::Char('w') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i).cloned() {
                            if self.workspaces.is_empty() {
                                let _ = self.workspace_manager.create_workspace("Default", None).await;
                            }
                            if let Some(ws) = self.workspace_manager.get_workspaces().first() {
                                let _ = self.workspace_manager.add_item(ws.id, WorkspaceItemType::Container, &c.name).await;
                            }
                        }
                    }
                }
                KeyCode::Char('E') | KeyCode::Char('e') => {
                    if let Some(i) = self.docker_list_state.table_state.selected() {
                        if let Some(c) = self.containers.get(i).cloned() {
                            // Drop out of TUI temporarily
                            let _ = crossterm::terminal::disable_raw_mode();
                            let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture);
                            
                            // Spawn interactive shell
                            let _ = std::process::Command::new("docker")
                                .arg("exec")
                                .arg("-it")
                                .arg(&c.name)
                                .arg("/bin/sh")
                                .status();
                                
                            // Re-enter TUI
                            let _ = crossterm::terminal::enable_raw_mode();
                            let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
