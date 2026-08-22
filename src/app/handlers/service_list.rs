use crate::app::{App, ActiveScreen};
use crate::ui::screens::log_view::LogViewState;
use crate::domain::workspaces::models::WorkspaceItemType;
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) async fn handle_service_list_events(&mut self, key: KeyEvent) {
        if self.service_list_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => self.service_list_state.is_searching = false,
                KeyCode::Char(c) => { self.service_list_state.search_query.push(c); self.refresh_service_list(); },
                KeyCode::Backspace => { self.service_list_state.search_query.pop(); self.refresh_service_list(); },
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.active_screen = ActiveScreen::ProcessList,
                KeyCode::Char('/') => self.service_list_state.is_searching = true,
                KeyCode::Down | KeyCode::Char('j') => self.service_list_state.next(self.services.len()),
                KeyCode::Up | KeyCode::Char('k') => self.service_list_state.previous(self.services.len()),
                KeyCode::Char('S') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i) { let _ = self.service_manager.start_service(&s.name); self.notify(format!("Starting service {}", s.name)); }
                    }
                }
                KeyCode::Char('K') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i) { let _ = self.service_manager.stop_service(&s.name); self.notify(format!("Stopping service {}", s.name)); }
                    }
                }
                KeyCode::Char('R') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i) { let _ = self.service_manager.restart_service(&s.name); self.notify(format!("Restarting service {}", s.name)); }
                    }
                }
                KeyCode::Char('E') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i) { let _ = self.service_manager.enable_service(&s.name); self.notify(format!("Enabled service {}", s.name)); }
                    }
                }
                KeyCode::Char('D') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i) { let _ = self.service_manager.disable_service(&s.name); self.notify(format!("Disabled service {}", s.name)); }
                    }
                }
                KeyCode::Char('L') | KeyCode::Char('l') => {
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i).cloned() {
                            self.log_view_state = LogViewState::new();
                            self.log_view_state.service_name = Some(s.name.clone());
                            self.log_view_state.title = s.name.clone();
                            if let Ok(logs) = self.service_manager.get_service_logs(&s.name, 100) {
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
                    if let Some(i) = self.service_list_state.table_state.selected() {
                        if let Some(s) = self.services.get(i).cloned() {
                            if self.workspaces.is_empty() {
                                let _ = self.workspace_manager.create_workspace("Default", None).await;
                            }
                            if let Some(ws) = self.workspace_manager.get_workspaces().first() {
                                let _ = self.workspace_manager.add_item(ws.id, WorkspaceItemType::Service, &s.name).await;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
