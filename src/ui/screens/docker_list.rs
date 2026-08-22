use crate::domain::docker::models::{ContainerInfo, ContainerStatus};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Frame,
};

pub struct DockerListState {
    pub table_state: TableState,
    pub search_query: String,
    pub is_searching: bool,
}

impl DockerListState {
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

pub fn render_docker_list(
    f: &mut Frame,
    area: Rect,
    containers: &[ContainerInfo],
    state: &mut DockerListState,
) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Table
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Search bar
    let search_title = if state.is_searching {
        " Search Containers (Active - Press Enter to finish) "
    } else {
        " Search Containers (Press '/' to search) "
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .style(if state.is_searching { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let search_text = Paragraph::new(state.search_query.as_str()).block(search_block);
    f.render_widget(search_text, chunks[0]);

    // Table
    let header_cells = ["ID", "Name", "Image", "Status", "Ports"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1).bottom_margin(0);

    let rows = containers.iter().map(|c| {
        let status_color = match c.status {
            ContainerStatus::Running => Color::Green,
            ContainerStatus::Exited => Color::Red,
            ContainerStatus::Paused => Color::Yellow,
            ContainerStatus::Restarting => Color::LightYellow,
            ContainerStatus::Dead => Color::DarkGray,
            ContainerStatus::Created => Color::Cyan,
            ContainerStatus::Removing => Color::Gray,
            ContainerStatus::Stopped => Color::Gray,
            ContainerStatus::Unknown => Color::Gray,
        };
        let status_cell = Cell::from(c.state_string.clone()).style(Style::default().fg(status_color));

        Row::new(vec![
            Cell::from(c.id.clone()),
            Cell::from(c.name.clone()),
            Cell::from(c.image.clone()),
            status_cell,
            Cell::from(c.ports.clone()),
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(14), // ID
        Constraint::Length(25), // Name
        Constraint::Length(30), // Image
        Constraint::Length(25), // Status
        Constraint::Min(20),    // Ports
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Docker Containers (S: Start, K: Stop, R: Restart, W: Add to Workspace, L: Logs, E: Shell) "))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);

    // Help Bar
    let help_text = Paragraph::new("Actions: [S] Start | [K] Stop | [R] Restart")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help_text, chunks[2]);
}
