use crate::domain::processes::models::ProcessInfo;
use crate::domain::ports::models::PortInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};

pub fn render_process_details(f: &mut Frame, area: Rect, process: Option<&ProcessInfo>, ports: &[PortInfo]) {
    if let Some(p) = process {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(15),
                Constraint::Length(7),
                Constraint::Length(7),
            ])
            .split(area);

        let ports_str = if ports.is_empty() {
            "-".to_string()
        } else {
            ports.iter().map(|pt| format!("{}/{:?}", pt.local_port, pt.protocol)).collect::<Vec<_>>().join(", ")
        };

        let text = vec![
            Line::from(vec![Span::styled("Name: ", Style::default().fg(Color::Cyan)), Span::raw(&p.name)]),
            Line::from(vec![Span::styled("PID: ", Style::default().fg(Color::Cyan)), Span::raw(p.pid.to_string())]),
            Line::from(vec![Span::styled("Parent PID: ", Style::default().fg(Color::Cyan)), Span::raw(p.parent.map(|pid| pid.to_string()).unwrap_or_else(|| "-".to_string()))]),
            Line::from(vec![Span::styled("User: ", Style::default().fg(Color::Cyan)), Span::raw(p.user_id.as_deref().unwrap_or("-"))]),
            Line::from(vec![Span::styled("Executable: ", Style::default().fg(Color::Cyan)), Span::raw(&p.exe)]),
            Line::from(vec![Span::styled("Command: ", Style::default().fg(Color::Cyan)), Span::raw(p.cmd.join(" "))]),
            Line::from(vec![Span::styled("Status: ", Style::default().fg(Color::Cyan)), Span::raw(format!("{:?}", p.status))]),
            Line::from(vec![Span::styled("CPU Usage: ", Style::default().fg(Color::Cyan)), Span::raw(format!("{:.1}%", p.cpu_usage))]),
            Line::from(vec![Span::styled("Memory: ", Style::default().fg(Color::Cyan)), Span::raw(format!("{:.1} MB", p.memory as f64 / 1024.0 / 1024.0))]),
            Line::from(vec![Span::styled("Virtual Memory: ", Style::default().fg(Color::Cyan)), Span::raw(format!("{:.1} MB", p.virtual_memory as f64 / 1024.0 / 1024.0))]),
            Line::from(vec![Span::styled("Start Time: ", Style::default().fg(Color::Cyan)), Span::raw(p.start_time.to_string())]),
            Line::from(vec![Span::styled("Run Time: ", Style::default().fg(Color::Cyan)), Span::raw(format!("{}s", p.run_time))]),
            Line::from(""),
            Line::from(vec![Span::styled("Open Ports: ", Style::default().fg(Color::Magenta)), Span::raw(ports_str)]),
            Line::from(""),
            Line::styled("Actions: [K] Kill Process | [T] View Network Tabs", Style::default().add_modifier(Modifier::BOLD)),
        ];
        
        let block = Block::default()
            .title(" Process Details (Press ESC to go back) ")
            .borders(Borders::ALL);
        
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, chunks[0]);

        let cpu_data: Vec<(f64, f64)> = p.cpu_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64 / 100.0)).collect();
        let mem_data: Vec<(f64, f64)> = p.memory_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();

        let cpu_max = cpu_data.iter().map(|(_, v)| *v).fold(10.0f64, f64::max); 
        let mem_max = mem_data.iter().map(|(_, v)| *v).fold(100.0f64, f64::max);

        let cpu_dataset = ratatui::widgets::Dataset::default()
            .name("CPU %")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Yellow))
            .data(&cpu_data);

        let cpu_chart = ratatui::widgets::Chart::new(vec![cpu_dataset])
            .block(Block::default().title(" CPU Usage History ").borders(Borders::ALL))
            .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 60.0]))
            .y_axis(ratatui::widgets::Axis::default().bounds([0.0, cpu_max]).labels(vec![Span::raw("0%"), Span::raw(format!("{:.0}%", cpu_max))]));

        let mem_dataset = ratatui::widgets::Dataset::default()
            .name("Memory MB")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&mem_data);

        let mem_chart = ratatui::widgets::Chart::new(vec![mem_dataset])
            .block(Block::default().title(" Memory Usage History ").borders(Borders::ALL))
            .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 60.0]))
            .y_axis(ratatui::widgets::Axis::default().bounds([0.0, mem_max]).labels(vec![Span::raw("0"), Span::raw(format!("{:.0}", mem_max))]));

        f.render_widget(cpu_chart, chunks[1]);
        f.render_widget(mem_chart, chunks[2]);
    } else {
        let block = Block::default()
            .title(" Process Details (Press ESC to go back) ")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("No process selected.").block(block);
        f.render_widget(paragraph, area);
    }
}
