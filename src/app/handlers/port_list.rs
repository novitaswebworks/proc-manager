use crate::app::{App, ActiveScreen};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) fn handle_port_list_events(&mut self, key: KeyEvent) {
        if self.port_list_state.is_searching {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => self.port_list_state.is_searching = false,
                KeyCode::Char(c) => { self.port_list_state.search_query.push(c); self.refresh_port_list(); },
                KeyCode::Backspace => { self.port_list_state.search_query.pop(); self.refresh_port_list(); },
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.active_screen = ActiveScreen::ProcessList,
                KeyCode::Char('/') => self.port_list_state.is_searching = true,
                KeyCode::Down | KeyCode::Char('j') => self.port_list_state.next(self.ports.len()),
                KeyCode::Up | KeyCode::Char('k') => self.port_list_state.previous(self.ports.len()),
                KeyCode::Enter => {
                    if let Some(i) = self.port_list_state.table_state.selected() {
                        if let Some(p) = self.ports.get(i) {
                            if let Some(pid) = p.pids.first() {
                                if let Some(sys_pid) = self.processes.iter().find(|pr| pr.pid.as_u32() == *pid).map(|pr| pr.pid) {
                                    self.selected_process_pid = Some(sys_pid);
                                    self.active_screen = ActiveScreen::ProcessDetails;
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('K') => {
                    if let Some(i) = self.port_list_state.table_state.selected() {
                        if let Some(p) = self.ports.get(i).cloned() {
                            for pid in p.pids {
                                if let Some(sys_pid) = self.processes.iter().find(|pr| pr.pid.as_u32() == pid).map(|pr| pr.pid) {
                                    let _ = self.process_manager.kill_process(sys_pid);
                                    self.notify(format!("Killed process {} holding port {}", pid, p.local_port));
                                }
                            }
                            self.refresh_port_list();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
