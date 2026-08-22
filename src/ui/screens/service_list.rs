use crate::domain::services::models::{ServiceInfo, ServiceStatus};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Frame,
};

pub struct ServiceListState {
    pub table_state: TableState,
    pub search_query: String,
    pub is_searching: bool,
}

impl ServiceListState {
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

pub fn render_service_list(
    f: &mut Frame,
    area: Rect,
    services: &[ServiceInfo],
    state: &mut ServiceListState,
) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Table
            Constraint::Length(3), // Help / Instructions
        ])
        .split(area);

    // Search bar
    let search_title = if state.is_searching {
        " Search Services (Active - Press Enter to finish) "
    } else {
        " Search Services (Press '/' to search) "
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .style(if state.is_searching { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let search_text = Paragraph::new(state.search_query.as_str()).block(search_block);
    f.render_widget(search_text, chunks[0]);

    // Table
    let header_cells = ["Status", "Name", "Enabled", "Description"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1).bottom_margin(0);

    let rows = services.iter().map(|s| {
        let status = format!("{:?}", s.status);
        let status_color = match s.status {
            ServiceStatus::Running => Color::Green,
            ServiceStatus::Failed => Color::Red,
            ServiceStatus::Stopped => Color::Gray,
            ServiceStatus::Restarting => Color::Yellow,
            ServiceStatus::Unknown => Color::DarkGray,
        };
        let status_cell = Cell::from(status).style(Style::default().fg(status_color));
        
        let name = s.name.clone();
        let enabled = if s.is_enabled { "Yes".to_string() } else { "No".to_string() };
        let desc = s.description.clone();

        Row::new(vec![status_cell, Cell::from(name), Cell::from(enabled), Cell::from(desc)])
    });

    let table = Table::new(rows, [
        Constraint::Length(12), // Status
        Constraint::Length(40), // Name
        Constraint::Length(8),  // Enabled
        Constraint::Min(20),    // Description
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Services (S: Start, K: Stop, R: Restart, E/D: Enable/Disable, W: Add to Workspace, L: Logs) "))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);

    // Help Bar
    let help_text = Paragraph::new("Actions: [S] Start | [K] Stop | [R] Restart | [E] Enable | [D] Disable")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help_text, chunks[2]);
}
