use crate::app::{App, ActiveScreen};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) async fn handle_workspace_list_events(&mut self, key: KeyEvent) {
        if self.workspace_list_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => self.workspace_list_state.is_searching = false,
                KeyCode::Char(c) => { self.workspace_list_state.search_query.push(c); self.refresh_workspace_list(); },
                KeyCode::Backspace => { self.workspace_list_state.search_query.pop(); self.refresh_workspace_list(); },
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.active_screen = ActiveScreen::ProcessList,
                KeyCode::Char('/') => self.workspace_list_state.is_searching = true,
                KeyCode::Char('N') | KeyCode::Char('n') => self.is_prompting_workspace = true,
                KeyCode::Tab => self.workspace_list_state.focus_items = !self.workspace_list_state.focus_items,
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.workspace_list_state.focus_items {
                        if let Some(i) = self.workspace_list_state.table_state.selected() {
                            if let Some(ws) = self.workspaces.get(i) { self.workspace_list_state.item_next(ws.items.len()); }
                        }
                    } else { self.workspace_list_state.next(self.workspaces.len()); }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.workspace_list_state.focus_items {
                        if let Some(i) = self.workspace_list_state.table_state.selected() {
                            if let Some(ws) = self.workspaces.get(i) { self.workspace_list_state.item_previous(ws.items.len()); }
                        }
                    } else { self.workspace_list_state.previous(self.workspaces.len()); }
                }
                KeyCode::Char('S') => self.handle_workspace_action('S').await,
                KeyCode::Char('K') => self.handle_workspace_action('K').await,
                KeyCode::Char('D') => {
                    if self.workspace_list_state.focus_items {
                        if let Some(ws_idx) = self.workspace_list_state.table_state.selected() {
                            if let Some(ws) = self.workspaces.get(ws_idx) {
                                if let Some(item_idx) = self.workspace_list_state.item_list_state.selected() {
                                    if let Some(item) = ws.items.get(item_idx) {
                                        let _ = self.workspace_manager.remove_item(item.id).await;
                                        self.notify(format!("Removed {} from workspace", item.item_name));
                                        self.workspace_list_state.item_list_state.select(None);
                                    }
                                }
                            }
                        }
                    } else {
                        if let Some(ws_idx) = self.workspace_list_state.table_state.selected() {
                            if let Some(ws) = self.workspaces.get(ws_idx).cloned() {
                                let _ = self.workspace_manager.delete_workspace(ws.id).await;
                                self.notify(format!("Deleted workspace '{}'", ws.name));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
