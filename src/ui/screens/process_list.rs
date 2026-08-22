use crate::domain::processes::models::ProcessInfo;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Frame,
};

pub struct ProcessListState {
    pub table_state: TableState,
    pub search_query: String,
    pub is_searching: bool,
    pub sort_by: SortColumn,
    pub sort_descending: bool,
}

#[derive(PartialEq)]
pub enum SortColumn {
    Pid,
    Name,
    Cpu,
    Memory,
}

impl ProcessListState {
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
            search_query: String::new(),
            is_searching: false,
            sort_by: SortColumn::Cpu,
            sort_descending: true,
        }
    }

    pub fn next(&mut self, item_count: usize) {
        if item_count == 0 {
            self.table_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= item_count - 1 {
                    0
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
                    item_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

pub fn render_process_list(f: &mut Frame, area: Rect, processes: &[ProcessInfo], state: &mut ProcessListState) {
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
    let header_cells = ["PID", "Name", "CPU %", "Mem (MB)", "User"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1).bottom_margin(0);

    let rows = processes.iter().map(|p| {
        let pid = p.pid.to_string();
        let name = p.name.clone();
        let cpu = format!("{:.1}", p.cpu_usage);
        let mem = format!("{:.1}", p.memory as f64 / 1024.0 / 1024.0);
        let user = p.user_id.clone().unwrap_or_else(|| "-".to_string());

        Row::new(vec![pid, name, cpu, mem, user])
    });

    let table = Table::new(rows, [
        Constraint::Length(8),
        Constraint::Min(20),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(15),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Processes "))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);
}
