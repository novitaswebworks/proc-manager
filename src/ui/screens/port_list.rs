use crate::domain::ports::models::PortInfo;
use crate::domain::processes::models::ProcessInfo;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Frame,
};

pub struct PortListState {
    pub table_state: TableState,
    pub search_query: String,
    pub is_searching: bool,
}

impl PortListState {
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
            search_query: String::new(),
            is_searching: false,
        }
    }

    pub fn next(&mut self, item_count: usize) {
        if item_count == 0 {
            self.table_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= item_count.saturating_sub(1) {
                    item_count.saturating_sub(1)
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self, item_count: usize) {
        if item_count == 0 {
            self.table_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

pub fn render_port_list(
    f: &mut Frame,
    area: Rect,
    ports: &[PortInfo],
    processes: &[ProcessInfo],
    state: &mut PortListState,
) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Table
        ])
        .split(area);

    // Search bar
    let search_title = if state.is_searching {
        " Search (Active - Press Enter to finish) "
    } else {
        " Search (Press '/' to search) "
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .style(if state.is_searching { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let search_text = Paragraph::new(state.search_query.as_str()).block(search_block);
    f.render_widget(search_text, chunks[0]);

    // Table
    let header_cells = ["Local Address", "Protocol", "State", "PID", "Process Name"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1).bottom_margin(0);

    let rows = ports.iter().map(|p| {
        let address_str = format!("{}:{}", p.local_ip, p.local_port);
        
        let protocol_cell = match p.protocol {
            crate::domain::ports::models::Protocol::Tcp => Cell::from("TCP").style(Style::default().fg(Color::Cyan)),
            crate::domain::ports::models::Protocol::Udp => Cell::from("UDP").style(Style::default().fg(Color::Magenta)),
        };

        let state_str = p.state.clone();
        let state_cell = match state_str.as_str() {
            "LISTEN" => Cell::from(state_str).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            "ESTABLISHED" => Cell::from(state_str).style(Style::default().fg(Color::Yellow)),
            "TIME_WAIT" | "CLOSE_WAIT" => Cell::from(state_str).style(Style::default().fg(Color::DarkGray)),
            _ => Cell::from(state_str),
        };
        
        let pid_str = p.pids.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(", ");
        
        // Find process names
        let process_names = p.pids.iter().map(|pid| {
            if let Some(proc) = processes.iter().find(|proc| proc.pid.as_u32() == *pid) {
                proc.name.clone()
            } else {
                "-".to_string()
            }
        }).collect::<Vec<_>>().join(", ");

        Row::new(vec![
            Cell::from(address_str),
            protocol_cell,
            state_cell,
            Cell::from(pid_str),
            Cell::from(process_names)
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(25), // Address
        Constraint::Length(10), // Protocol
        Constraint::Length(15), // State
        Constraint::Length(15), // PID
        Constraint::Min(20),    // Process Name
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Listening Ports (Actions: [Enter] Inspect Process | [K] Kill Process | [Esc] Back) "))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);
}
