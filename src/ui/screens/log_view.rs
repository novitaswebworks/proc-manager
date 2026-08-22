use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

pub struct LogViewState {
    pub logs: Vec<String>,
    pub title: String,
    pub scroll: u16,
    pub auto_scroll: bool,
    pub service_name: Option<String>,
    pub container_id: Option<String>,
    pub is_searching: bool,
    pub search_query: String,
}

impl LogViewState {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            title: String::new(),
            scroll: 0,
            auto_scroll: true,
            service_name: None,
            container_id: None,
            is_searching: false,
            search_query: String::new(),
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
            self.auto_scroll = false;
        }
    }

    pub fn scroll_down(&mut self, max: u16) {
        if self.scroll < max {
            self.scroll += 1;
            if self.scroll == max {
                self.auto_scroll = true;
            }
        }
    }

    pub fn set_logs(&mut self, logs: Vec<String>) {
        self.logs = logs;
    }
}

pub fn render_log_view(f: &mut Frame, area: Rect, state: &mut LogViewState) {
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3), // Search bar
            ratatui::layout::Constraint::Min(0),    // Logs
        ])
        .split(area);

    let search_title = if state.is_searching {
        " Search Logs (Active - Press Enter to finish) "
    } else {
        " Search Logs (Press '/' to search) "
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .style(if state.is_searching { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let search_text = Paragraph::new(state.search_query.as_str()).block(search_block);
    f.render_widget(search_text, chunks[0]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} Logs (L/R: Refresh, Esc: Back, Up/Down: Scroll) ", state.title));

    let query = state.search_query.to_lowercase();
    let filtered_logs: Vec<String> = if query.is_empty() {
        state.logs.clone()
    } else {
        state.logs.iter().filter(|l| l.to_lowercase().contains(&query)).cloned().collect()
    };

    let text: String = filtered_logs.join("\n");
    let line_count = filtered_logs.len() as u16;
    let height = chunks[1].height.saturating_sub(2); // borders

    if state.auto_scroll {
        state.scroll = line_count.saturating_sub(height);
    } else {
        state.scroll = state.scroll.min(line_count.saturating_sub(height));
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .scroll((state.scroll, 0))
        .style(Style::default().fg(Color::LightCyan));

    f.render_widget(paragraph, chunks[1]);
    
    // Render Scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
        
    let mut scrollbar_state = ScrollbarState::new(line_count as usize).position(state.scroll as usize);
    
    let scroll_area = Rect {
        x: chunks[1].x + chunks[1].width.saturating_sub(1),
        y: chunks[1].y + 1,
        width: 1,
        height: chunks[1].height.saturating_sub(2),
    };
    
    f.render_stateful_widget(scrollbar, scroll_area, &mut scrollbar_state);
}
