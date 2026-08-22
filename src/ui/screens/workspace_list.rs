use crate::domain::workspaces::models::WorkspaceInfo;
use ratatui::{
    layout::{Constraint, Layout, Rect, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph, List, ListItem},
    Frame,
};

pub struct WorkspaceListState {
    pub table_state: TableState,
    pub item_list_state: ratatui::widgets::ListState,
    pub search_query: String,
    pub is_searching: bool,
    pub focus_items: bool,
}

impl WorkspaceListState {
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
            item_list_state: ratatui::widgets::ListState::default(),
            search_query: String::new(),
            is_searching: false,
            focus_items: false,
        }
    }

    pub fn next(&mut self, item_count: usize) {
        if item_count == 0 {
            self.table_state.select(None);
            self.item_list_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= item_count.saturating_sub(1) { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.item_list_state.select(None);
    }

    pub fn previous(&mut self, item_count: usize) {
        if item_count == 0 {
            self.table_state.select(None);
            self.item_list_state.select(None);
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 { item_count - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.item_list_state.select(None);
    }

    pub fn item_next(&mut self, item_count: usize) {
        if item_count == 0 {
            self.item_list_state.select(None);
            return;
        }
        let i = match self.item_list_state.selected() {
            Some(i) => {
                if i >= item_count.saturating_sub(1) { item_count.saturating_sub(1) } else { i + 1 }
            }
            None => 0,
        };
        self.item_list_state.select(Some(i));
    }

    pub fn item_previous(&mut self, item_count: usize) {
        if item_count == 0 {
            self.item_list_state.select(None);
            return;
        }
        let i = match self.item_list_state.selected() {
            Some(i) => {
                if i == 0 { 0 } else { i - 1 }
            }
            None => 0,
        };
        self.item_list_state.select(Some(i));
    }
}

pub fn render_workspace_list(
    f: &mut Frame,
    area: Rect,
    workspaces: &[WorkspaceInfo],
    state: &mut WorkspaceListState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search
            Constraint::Percentage(50), // Workspace List
            Constraint::Percentage(50), // Items in Workspace
        ])
        .split(area);

    // Search bar
    let search_title = if state.is_searching {
        " Search Workspaces (Active - Press Enter to finish) "
    } else {
        " Search Workspaces (Press '/' to search, 'N' to Create New) "
    };
    let search_block = Block::default().title(search_title).borders(Borders::ALL).style(if state.is_searching { Style::default().fg(Color::Yellow) } else { Style::default() });
    f.render_widget(Paragraph::new(state.search_query.as_str()).block(search_block), chunks[0]);

    // Table
    let header = Row::new(vec!["ID", "Name", "Description", "Items Count"])
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .height(1).bottom_margin(0);

    let rows = workspaces.iter().map(|w| {
        Row::new(vec![
            Cell::from(w.id.to_string()),
            Cell::from(w.name.clone()),
            Cell::from(w.description.clone().unwrap_or_default()),
            Cell::from(w.items.len().to_string()),
        ])
    });

    let table_block = Block::default()
        .borders(Borders::ALL)
        .title(" Workspaces ")
        .style(if state.focus_items { Style::default().fg(Color::DarkGray) } else { Style::default().fg(Color::Yellow) });

    let table = Table::new(rows, [
        Constraint::Length(5),
        Constraint::Length(30),
        Constraint::Min(20),
        Constraint::Length(15),
    ])
    .header(header)
    .block(table_block)
    .row_highlight_style(if !state.focus_items { Style::default().add_modifier(Modifier::REVERSED) } else { Style::default() })
    .highlight_symbol(if !state.focus_items { ">> " } else { "   " });

    f.render_stateful_widget(table, chunks[1], &mut state.table_state);

    // Items Panel
    let items_block = Block::default()
        .borders(Borders::ALL)
        .title(" Workspace Items (Tab: Switch Focus, D: Delete/Remove) ")
        .style(if state.focus_items { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::DarkGray) });
    
    if let Some(selected) = state.table_state.selected() {
        if let Some(ws) = workspaces.get(selected) {
            let items: Vec<ListItem> = ws.items.iter().map(|item| {
                let text = format!("[{}] {}", item.item_type.as_str().to_uppercase(), item.item_name);
                let color = match item.item_type {
                    crate::domain::workspaces::models::WorkspaceItemType::Process => Color::Cyan,
                    crate::domain::workspaces::models::WorkspaceItemType::Service => Color::Green,
                    crate::domain::workspaces::models::WorkspaceItemType::Container => Color::Blue,
                };
                ListItem::new(text).style(Style::default().fg(color))
            }).collect();
            
            let list = List::new(items)
                .block(items_block)
                .highlight_style(if state.focus_items { Style::default().add_modifier(Modifier::REVERSED) } else { Style::default() })
                .highlight_symbol(if state.focus_items { ">> " } else { "   " });
            f.render_stateful_widget(list, chunks[2], &mut state.item_list_state);
        } else {
            f.render_widget(Paragraph::new("No workspace selected").block(items_block), chunks[2]);
        }
    } else {
        f.render_widget(Paragraph::new("Select a workspace").block(items_block), chunks[2]);
    }
}
