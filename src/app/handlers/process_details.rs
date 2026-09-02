use crate::app::{App, ActiveScreen};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub(crate) fn handle_process_details_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.active_screen = ActiveScreen::ProcessList,
            KeyCode::Char('K') => {
                if let Some(pid) = self.selected_process_pid {
                    let _ = self.process_manager.kill_process(pid, Some(&self.ssh_manager));
                    self.notify(format!("Sent Kill to process {}", pid));
                    self.active_screen = ActiveScreen::ProcessList;
                    self.refresh_process_list();
                }
            }
            KeyCode::Char('T') | KeyCode::Char('t') => self.active_screen = ActiveScreen::PortList,
            _ => {}
        }
    }
}
