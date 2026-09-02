use super::{App, ActiveScreen};
use crate::domain::workspaces::models::WorkspaceItemType;
use crate::ui::screens::log_view::LogViewState;

impl App {
    pub(super) async fn handle_workspace_action(&mut self, action: char) {
        if let Some(i) = self.workspace_list_state.table_state.selected() {
            if let Some(ws) = self.workspaces.get(i).cloned() {
                if action == 'S' || action == 'K' {
                    let mut action_count = 0;
                    for item in ws.items {
                        action_count += 1;
                        match item.item_type {
                            WorkspaceItemType::Process => {
                                if action == 'K' {
                                    let mut pids_to_kill = Vec::new();
                                    for p in self.process_manager.get_processes() {
                                        if p.name == item.item_name {
                                            pids_to_kill.push(p.pid);
                                        }
                                    }
                                    for pid in pids_to_kill {
                                        let _ = self.process_manager.kill_process(pid, Some(&self.ssh_manager));
                                    }
                                }
                            }
                            WorkspaceItemType::Service => {
                                if action == 'S' {
                                    let _ = self.service_manager.start_service(&item.item_name);
                                } else {
                                    let _ = self.service_manager.stop_service(&item.item_name);
                                }
                            }
                            WorkspaceItemType::Container => {
                                if action == 'S' {
                                    let _ = self.docker_manager.start_container(&item.item_name).await;
                                } else {
                                    let _ = self.docker_manager.stop_container(&item.item_name).await;
                                }
                            }
                        }
                    }
                    if action == 'S' {
                        self.notify(format!("Sent Start to {} items", action_count));
                    } else {
                        self.notify(format!("Sent Kill/Stop to {} items", action_count));
                    }
                }
            }
        }
    }

    pub(super) async fn execute_palette_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() { return; }

        let cmd = parts[0].to_lowercase();
        let target = if parts.len() > 1 { parts[1..].join(" ").to_lowercase() } else { String::new() };

        match cmd.as_str() {
            "kill" => {
                if let Some(p) = self.processes.iter().find(|p| p.name.to_lowercase().contains(&target)) {
                    let _ = self.process_manager.kill_process(p.pid, Some(&self.ssh_manager));
                    self.notify(format!("Killed process {} ({})", p.name, p.pid));
                } else {
                    self.notify(format!("Process '{}' not found", target));
                }
            }
            "restart" => {
                if let Some(c) = self.containers.iter().find(|c| c.name.to_lowercase().contains(&target)) {
                    let _ = self.docker_manager.restart_container(&c.name).await;
                    self.notify(format!("Restarted container {}", c.name));
                } else if let Some(s) = self.services.iter().find(|s| s.name.to_lowercase().contains(&target)) {
                    let _ = self.service_manager.restart_service(&s.name);
                    self.notify(format!("Restarted service {}", s.name));
                } else {
                    self.notify(format!("Target '{}' not found in containers or services", target));
                }
            }
            "logs" => {
                if let Some(c) = self.containers.iter().find(|c| c.name.to_lowercase().contains(&target)) {
                    self.log_view_state = LogViewState::new();
                    self.log_view_state.container_id = Some(c.id.clone());
                    self.log_view_state.title = c.name.clone();
                    if let Ok(logs) = self.docker_manager.get_container_logs(&c.id, 100).await {
                        self.log_view_state.set_logs(logs);
                    }
                    self.previous_screen = self.active_screen;
                    self.active_screen = ActiveScreen::LogView;
                } else if let Some(s) = self.services.iter().find(|s| s.name.to_lowercase().contains(&target)) {
                    self.log_view_state = LogViewState::new();
                    self.log_view_state.service_name = Some(s.name.clone());
                    self.log_view_state.title = s.name.clone();
                    if let Ok(logs) = self.service_manager.get_service_logs(&s.name, 100) {
                        self.log_view_state.set_logs(logs);
                    }
                    self.previous_screen = self.active_screen;
                    self.active_screen = ActiveScreen::LogView;
                } else {
                    self.notify(format!("Logs for '{}' not found", target));
                }
            }
            "quit" | "q" => self.should_quit = true,
            _ => {
                self.notify(format!("Unknown command: {}", cmd));
            }
        }
    }
}
