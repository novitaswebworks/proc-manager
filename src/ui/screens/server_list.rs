use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

pub struct ServerInfo {
    pub name: String,
    pub address: String,
    pub status: String,
}

pub struct ServerListState {
    pub state: TableState,
}

impl ServerListState {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
        }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn render_server_list(
    f: &mut Frame,
    area: Rect,
    servers: &[ServerInfo],
    state: &mut ServerListState,
) {
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(Color::Cyan);
    let normal_style = Style::default().fg(Color::White);

    let header_cells = ["Name", "Address", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1)
        .bottom_margin(1);

    let rows = servers.iter().map(|item| {
        let status_color = match item.status.as_str() {
            "Connected" => Color::Green,
            "Disconnected" => Color::Red,
            _ => Color::Gray,
        };
        
        let cells = vec![
            Cell::from(item.name.clone()),
            Cell::from(item.address.clone()),
            Cell::from(item.status.clone()).style(Style::default().fg(status_color)),
        ];
        Row::new(cells).style(normal_style).height(1)
    });

    let t = Table::new(rows, [
        Constraint::Percentage(30),
        Constraint::Percentage(50),
        Constraint::Percentage(20),
    ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Remote Servers ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .row_highlight_style(selected_style);

    f.render_stateful_widget(t, area, &mut state.state);
}
