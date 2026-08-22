use crate::app::{App, ActiveScreen};
use crate::ui::screens::process_list::SortColumn;
use crate::domain::workspaces::models::WorkspaceItemType;
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) async fn handle_process_list_events(&mut self, key: KeyEvent) {
        if self.process_list_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => self.process_list_state.is_searching = false,
                KeyCode::Char(c) => { self.process_list_state.search_query.push(c); self.refresh_process_list(); },
                KeyCode::Backspace => { self.process_list_state.search_query.pop(); self.refresh_process_list(); },
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('/') => self.process_list_state.is_searching = true,
                KeyCode::Down | KeyCode::Char('j') => self.process_list_state.next(self.processes.len()),
                KeyCode::Up | KeyCode::Char('k') => self.process_list_state.previous(self.processes.len()),
                KeyCode::Enter => {
                    if let Some(i) = self.process_list_state.table_state.selected() {
                        if let Some(p) = self.processes.get(i) {
                            self.selected_process_pid = Some(p.pid);
                            self.active_screen = ActiveScreen::ProcessDetails;
                        }
                    }
                }
                KeyCode::Char('T') | KeyCode::Char('t') => {
                    self.process_list_state.is_tree_view = !self.process_list_state.is_tree_view;
                    self.refresh_process_list();
                    let msg = if self.process_list_state.is_tree_view { "Tree view enabled" } else { "Tree view disabled" };
                    self.notify(msg);
                }
                KeyCode::Char('O') | KeyCode::Char('o') => self.active_screen = ActiveScreen::PortList,
                KeyCode::Char('W') | KeyCode::Char('w') => {
                    if let Some(i) = self.process_list_state.table_state.selected() {
                        if let Some(p) = self.processes.get(i).cloned() {
                            if self.workspaces.is_empty() {
                                let _ = self.workspace_manager.create_workspace("Default", None).await;
                            }
                            if let Some(ws) = self.workspace_manager.get_workspaces().first() {
                                let _ = self.workspace_manager.add_item(ws.id, WorkspaceItemType::Process, &p.name).await;
                            }
                        }
                    }
                }
                KeyCode::Char('p') => { self.process_list_state.sort_by = SortColumn::Pid; self.sort_processes(); }
                KeyCode::Char('n') => { self.process_list_state.sort_by = SortColumn::Name; self.sort_processes(); }
                KeyCode::Char('c') => { self.process_list_state.sort_by = SortColumn::Cpu; self.sort_processes(); }
                KeyCode::Char('m') => { self.process_list_state.sort_by = SortColumn::Memory; self.sort_processes(); }
                KeyCode::Char('r') => { self.process_list_state.sort_descending = !self.process_list_state.sort_descending; self.sort_processes(); }
                _ => {}
            }
        }
    }
}
