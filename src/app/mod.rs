#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
pub mod handlers;

use crate::configuration::Config;
use crate::domain::processes::{ProcessManager, models::ProcessInfo};
use crate::domain::ports::{PortManager, models::PortInfo};
use crate::domain::services::{ServiceManager, models::ServiceInfo};
use crate::domain::docker::{DockerManager, models::ContainerInfo};
use crate::domain::workspaces::{WorkspaceManager, models::{WorkspaceInfo, WorkspaceItemType}};
use crate::events::EventBus;
use crate::infrastructure::platform::PlatformManager;
use crate::infrastructure::storage::database::Database;
use crate::ui::screens::process_list::{ProcessListState, render_process_list, SortColumn};
use crate::ui::screens::process_details::render_process_details;
use crate::ui::screens::port_list::{PortListState, render_port_list};
use crate::ui::screens::service_list::{ServiceListState, render_service_list};
use crate::ui::screens::docker_list::{DockerListState, render_docker_list};
use crate::ui::screens::workspace_list::{WorkspaceListState, render_workspace_list};
use crate::ui::screens::log_view::{LogViewState, render_log_view};
use crate::ui::screens::server_list::{ServerListState, ServerInfo, render_server_list};
use crate::ui::{init_tui, restore_tui, Tui};
use crate::errors::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tokio::time;
use sysinfo::Pid;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveScreen {
    ProcessList,
    ProcessDetails,
    PortList,
    ServiceList,
    DockerList,
    WorkspaceList,
    LogView,
    ServerList,
}

pub struct App {
    config: Config,
    database: Database,
    platform: PlatformManager,
    event_bus: EventBus,
    should_quit: bool,
    process_manager: ProcessManager,
    processes: Vec<ProcessInfo>,
    process_list_state: ProcessListState,
    port_manager: PortManager,
    ports: Vec<PortInfo>,
    port_list_state: PortListState,
    service_manager: ServiceManager,
    services: Vec<ServiceInfo>,
    service_list_state: ServiceListState,
    docker_manager: DockerManager,
    containers: Vec<ContainerInfo>,
    docker_list_state: DockerListState,
    workspace_manager: WorkspaceManager,
    workspaces: Vec<WorkspaceInfo>,
    workspace_list_state: WorkspaceListState,
    servers: Vec<ServerInfo>,
    server_list_state: ServerListState,
    ssh_manager: crate::infrastructure::ssh_manager::SshManager,
    log_view_state: LogViewState,
    active_screen: ActiveScreen,
    previous_screen: ActiveScreen,
    selected_process_pid: Option<Pid>,
    is_prompting_workspace: bool,
    workspace_prompt_text: String,
    is_command_palette: bool,
    command_prompt_text: String,
    notification: Option<String>,
    notification_expiry: Option<std::time::Instant>,
}

impl App {
    pub async fn new(config: Config, database: Database, platform: PlatformManager, event_bus: EventBus) -> Self {
        let mut process_manager = ProcessManager::new();
        process_manager.refresh(None);
        let processes = process_manager.get_processes();
        
        let mut port_manager = PortManager::new();
        port_manager.refresh();
        let ports = port_manager.get_ports();
        
        let mut service_manager = ServiceManager::new(platform.create_service_engine());
        let _ = service_manager.refresh();
        let services = service_manager.get_services();
        
        let mut docker_manager = DockerManager::new();
        let _ = docker_manager.refresh().await;
        let containers = docker_manager.get_containers();

        let mut workspace_manager = WorkspaceManager::new(database.clone());
        let _ = workspace_manager.refresh().await;
        if let Some(ws_conf) = &config.workspaces {
            workspace_manager.load_config_workspaces(ws_conf);
        }
        let workspaces = workspace_manager.get_workspaces();
        
        let mut active_screen = ActiveScreen::ProcessList;
        if let Some(theme) = &config.theme {
            if let Some(view) = &theme.default_view {
                match view.as_str() {
                    "DockerList" => active_screen = ActiveScreen::DockerList,
                    "ServiceList" => active_screen = ActiveScreen::ServiceList,
                    "WorkspaceList" => active_screen = ActiveScreen::WorkspaceList,
                    "PortList" => active_screen = ActiveScreen::PortList,
                    _ => {}
                }
            }
        }

        let mut initial_servers = Vec::new();
        if let Some(srv_conf) = &config.servers {
            for sc in srv_conf {
                initial_servers.push(ServerInfo {
                    name: sc.name.clone(),
                    address: sc.address.clone(),
                    status: "Disconnected".to_string(),
                });
            }
        }

        Self {
            config,
            database,
            platform,
            event_bus,
            should_quit: false,
            process_manager,
            processes,
            process_list_state: ProcessListState::new(),
            port_manager,
            ports,
            port_list_state: PortListState::new(),
            service_manager,
            services,
            service_list_state: ServiceListState::new(),
            docker_manager,
            containers,
            docker_list_state: DockerListState::new(),
            workspace_manager,
            workspaces,
            workspace_list_state: WorkspaceListState::new(),
            servers: initial_servers,
            server_list_state: ServerListState::new(),
            ssh_manager: crate::infrastructure::ssh_manager::SshManager::new(),
            log_view_state: LogViewState::new(),
            active_screen,
            previous_screen: active_screen,
            selected_process_pid: None,
            is_prompting_workspace: false,
            workspace_prompt_text: String::new(),
            is_command_palette: false,
            command_prompt_text: String::new(),
            notification: None,
            notification_expiry: None,
        }
    }

    pub fn notify(&mut self, msg: impl Into<String>) {
        self.notification = Some(msg.into());
        self.notification_expiry = Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = init_tui()?;
        let res = self.run_app(&mut tui).await;
        restore_tui(tui)?;
        res
    }

    async fn run_app(&mut self, tui: &mut Tui) -> Result<()> {
        let mut interval = time::interval(Duration::from_millis(500));
        self.sort_processes();

        loop {
            if self.should_quit {
                break;
            }

            tui.draw(|f| {
                let size = f.area();
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        ratatui::layout::Constraint::Min(5),
                        ratatui::layout::Constraint::Length(1),
                    ])
                    .split(size);
                
                let main_area = chunks[0];
                let footer_area = chunks[1];
                
                // Command Palette check
                if self.is_command_palette {
                    let block = ratatui::widgets::Block::default()
                        .title(" Command Palette (kill <name>, restart <name>, logs <name>) - Enter to run, Esc to cancel ")
                        .borders(ratatui::widgets::Borders::ALL)
                        .style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray).fg(ratatui::style::Color::White));
                    let p = ratatui::widgets::Paragraph::new(self.command_prompt_text.as_str()).block(block);
                    let area = ratatui::layout::Rect::new(size.width / 4, size.height / 2 - 2, size.width / 2, 3);
                    f.render_widget(ratatui::widgets::Clear, area);
                    f.render_widget(p, area);
                    return;
                }

                // Prompt overlay check
                if self.is_prompting_workspace {
                    let block = ratatui::widgets::Block::default()
                        .title(" Create Workspace (Type name and press Enter) ")
                        .borders(ratatui::widgets::Borders::ALL)
                        .style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
                    let p = ratatui::widgets::Paragraph::new(self.workspace_prompt_text.as_str()).block(block);
                    let area = ratatui::layout::Rect::new(size.width / 4, size.height / 2 - 2, size.width / 2, 3);
                    f.render_widget(ratatui::widgets::Clear, area);
                    f.render_widget(p, area);
                    return;
                }

                match self.active_screen {
                    ActiveScreen::ProcessList => {
                        let metrics = self.process_manager.get_system_metrics();
                        render_process_list(f, main_area, &self.processes, &metrics, &mut self.process_list_state);
                    }
                    ActiveScreen::ProcessDetails => {
                        let selected_process = self.selected_process_pid.and_then(|pid| self.processes.iter().find(|p| p.pid == pid));
                        let process_ports = self.selected_process_pid.map(|pid| self.port_manager.get_ports_for_pid(pid.as_u32())).unwrap_or_default();
                        render_process_details(f, main_area, selected_process, &process_ports);
                    }
                    ActiveScreen::PortList => {
                        render_port_list(f, main_area, &self.ports, &self.processes, &mut self.port_list_state);
                    }
                    ActiveScreen::ServiceList => {
                        render_service_list(f, main_area, &self.services, &mut self.service_list_state);
                    }
                    ActiveScreen::DockerList => {
                        render_docker_list(f, main_area, &self.containers, &mut self.docker_list_state);
                    }
                    ActiveScreen::WorkspaceList => {
                        render_workspace_list(f, main_area, &self.workspaces, &mut self.workspace_list_state);
                    }
                    ActiveScreen::LogView => {
                        render_log_view(f, main_area, &mut self.log_view_state);
                    }
                    ActiveScreen::ServerList => {
                        render_server_list(f, main_area, &self.servers, &mut self.server_list_state);
                    }
                }

                // Render Footer
                let footer_text = " NovaTask | [V] Cycle Views | [/] Search | [W] Add Workspace | [:] Command | [Q] Quit ";
                let footer = ratatui::widgets::Paragraph::new(footer_text)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White).bg(ratatui::style::Color::Blue));
                f.render_widget(footer, footer_area);

                if let Some(msg) = &self.notification {
                    if let Some(expiry) = self.notification_expiry {
                        if std::time::Instant::now() > expiry {
                            self.notification = None;
                            self.notification_expiry = None;
                        } else {
                            let block = ratatui::widgets::Block::default()
                                .borders(ratatui::widgets::Borders::ALL)
                                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).bg(ratatui::style::Color::DarkGray));
                            let p = ratatui::widgets::Paragraph::new(msg.as_str()).block(block);
                            let width = (msg.len() as u16 + 4).min(size.width);
                            let x = size.width.saturating_sub(width + 2);
                            let y = size.height.saturating_sub(4);
                            let area = ratatui::layout::Rect::new(x, y, width, 3);
                            f.render_widget(ratatui::widgets::Clear, area);
                            f.render_widget(p, area);
                        }
                    }
                }
            })?;

            tokio::select! {
                _ = interval.tick() => {
                    self.process_manager.refresh(Some(&self.ssh_manager));
                    self.refresh_process_list();
                    self.port_manager.refresh();
                    self.refresh_port_list();
                    let _ = self.service_manager.refresh();
                    self.refresh_service_list();
                    self.docker_manager.refresh().await;
                    self.refresh_docker_list();
                    let _ = self.workspace_manager.refresh().await;
                    self.refresh_workspace_list();
                }
                event_result = self.handle_crossterm_events() => {
                    event_result?;
                }
            }
        }
        Ok(())
    }
    
    // ... refreshing lists ...
    fn refresh_process_list(&mut self) {
        let all_processes = self.process_manager.get_processes();
        let query = self.process_list_state.search_query.to_lowercase();
        self.processes = all_processes.into_iter().filter(|p| {
            if query.is_empty() { true } else { p.name.to_lowercase().contains(&query) || p.pid.to_string().contains(&query) }
        }).collect();
        self.sort_processes();
    }
    
    fn refresh_port_list(&mut self) {
        let all_ports = self.port_manager.get_ports();
        let query = self.port_list_state.search_query.to_lowercase();
        self.ports = all_ports.into_iter().filter(|p| {
            if query.is_empty() { true } else { p.local_port.to_string().contains(&query) || format!("{:?}", p.protocol).to_lowercase().contains(&query) }
        }).collect();
    }

    fn refresh_service_list(&mut self) {
        let all_services = self.service_manager.get_services();
        let query = self.service_list_state.search_query.to_lowercase();
        self.services = all_services.into_iter().filter(|s| {
            if query.is_empty() { true } else { s.name.to_lowercase().contains(&query) || s.description.to_lowercase().contains(&query) }
        }).collect();
    }

    fn refresh_docker_list(&mut self) {
        let all_containers = self.docker_manager.get_containers();
        let query = self.docker_list_state.search_query.to_lowercase();
        self.containers = all_containers.into_iter().filter(|c| {
            if query.is_empty() { true } else { c.name.to_lowercase().contains(&query) || c.image.to_lowercase().contains(&query) }
        }).collect();
    }

    fn refresh_workspace_list(&mut self) {
        let all_workspaces = self.workspace_manager.get_workspaces();
        let query = self.workspace_list_state.search_query.to_lowercase();
        self.workspaces = all_workspaces.into_iter().filter(|w| {
            if query.is_empty() { true } else { w.name.to_lowercase().contains(&query) || w.description.clone().unwrap_or_default().to_lowercase().contains(&query) }
        }).collect();
    }
    
    fn sort_processes(&mut self) {
        let state = &self.process_list_state;
        
        // Reset depth
        for p in &mut self.processes {
            p.tree_depth = 0;
        }

        if state.is_tree_view {
            // Build parent-child map
            let mut children_map: std::collections::HashMap<Option<sysinfo::Pid>, Vec<ProcessInfo>> = std::collections::HashMap::new();
            for p in self.processes.drain(..) {
                children_map.entry(p.parent).or_default().push(p);
            }
            
            // Reconstruct flattened
            let mut flattened = Vec::new();
            
            // Helper function to recursively add children
            fn add_children(
                pid: Option<sysinfo::Pid>, 
                depth: u16, 
                map: &mut std::collections::HashMap<Option<sysinfo::Pid>, Vec<ProcessInfo>>,
                flat: &mut Vec<ProcessInfo>,
                sort_col: &SortColumn,
                desc: bool
            ) {
                if let Some(mut children) = map.remove(&pid) {
                    children.sort_by(|a, b| {
                        let cmp = match sort_col {
                            SortColumn::Pid => a.pid.as_u32().cmp(&b.pid.as_u32()),
                            SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                            SortColumn::Cpu => a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
                            SortColumn::Memory => a.memory.cmp(&b.memory),
                        };
                        if desc { cmp.reverse() } else { cmp }
                    });
                    for mut child in children {
                        child.tree_depth = depth;
                        let child_pid = child.pid;
                        flat.push(child);
                        add_children(Some(child_pid), depth + 1, map, flat, sort_col, desc);
                    }
                }
            }
            
            // Start with orphans (no parent or parent not in map)
            // Wait, we need to find root processes (parent is None, or parent is not in the full list)
            // A simple way: find all keys in map that don't exist in any values
            let all_pids: std::collections::HashSet<_> = children_map.values().flat_map(|v| v.iter().map(|p| p.pid)).collect();
            let mut roots = Vec::new();
            let keys: Vec<_> = children_map.keys().copied().collect();
            for k in keys {
                if k.is_none() || !all_pids.contains(&k.unwrap()) {
                    roots.push(k);
                }
            }
            
            for root in roots {
                add_children(root, 0, &mut children_map, &mut flattened, &state.sort_by, state.sort_descending);
            }
            
            self.processes = flattened;
        } else {
            self.processes.sort_by(|a, b| {
                let cmp = match state.sort_by {
                    SortColumn::Pid => a.pid.as_u32().cmp(&b.pid.as_u32()),
                    SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    SortColumn::Cpu => a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
                    SortColumn::Memory => a.memory.cmp(&b.memory),
                };
                if state.sort_descending { cmp.reverse() } else { cmp }
            });
        }
    }

    async fn handle_workspace_action(&mut self, action: char) {
        if let Some(i) = self.workspace_list_state.table_state.selected() {
            if let Some(ws) = self.workspaces.get(i).cloned() {
                if action == 'S' || action == 'K' {
                    let mut action_count = 0;
                    for item in ws.items {
                        action_count += 1;
                        match item.item_type {
                            WorkspaceItemType::Process => {
                                if action == 'K' {
                                    // fuzzy match name to kill
                                    let mut pids_to_kill = Vec::new();
                                    for p in self.process_manager.get_processes() {
                                        if p.name == item.item_name {
                                            pids_to_kill.push(p.pid);
                                        }
                                    }
                                    for pid in pids_to_kill {
                                        let _ = self.process_manager.kill_process(pid, Some(&self.ssh_manager));
                                    }
                                }
                            }
                            WorkspaceItemType::Service => {
                                if action == 'S' {
                                    let _ = self.service_manager.start_service(&item.item_name);
                                } else {
                                    let _ = self.service_manager.stop_service(&item.item_name);
                                }
                            }
                            WorkspaceItemType::Container => {
                                if action == 'S' {
                                    let _ = self.docker_manager.start_container(&item.item_name).await;
                                } else {
                                    let _ = self.docker_manager.stop_container(&item.item_name).await;
                                }
                            }
                        }
                    }
                    if action == 'S' {
                        self.notify(format!("Sent Start to {} items", action_count));
                    } else {
                        self.notify(format!("Sent Kill/Stop to {} items", action_count));
                    }
                }
            }
        }
    }

    async fn handle_crossterm_events(&mut self) -> Result<()> {
        let has_event = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(50)))
            .await
            .map_err(|e| crate::errors::AppError::Unknown(e.into()))??;

        if has_event {
            let event = event::read()?;
            
            if let Event::Mouse(mouse) = event {
                use crossterm::event::MouseEventKind;
                match mouse.kind {
                    MouseEventKind::ScrollDown => {
                        match self.active_screen {
                            ActiveScreen::ProcessList => self.process_list_state.next(self.processes.len()),
                            ActiveScreen::ServiceList => self.service_list_state.next(self.services.len()),
                            ActiveScreen::DockerList => self.docker_list_state.next(self.containers.len()),
                            ActiveScreen::WorkspaceList => {
                                if self.workspace_list_state.focus_items {
                                    if let Some(i) = self.workspace_list_state.table_state.selected() {
                                        if let Some(ws) = self.workspaces.get(i) { self.workspace_list_state.item_next(ws.items.len()); }
                                    }
                                } else { self.workspace_list_state.next(self.workspaces.len()); }
                            },
                            ActiveScreen::PortList => self.port_list_state.next(self.ports.len()),
                            ActiveScreen::LogView => { let max = self.log_view_state.logs.len() as u16; self.log_view_state.scroll_down(max); },
                            ActiveScreen::ServerList => self.server_list_state.next(self.servers.len()),
                            _ => {}
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        match self.active_screen {
                            ActiveScreen::ProcessList => self.process_list_state.previous(self.processes.len()),
                            ActiveScreen::ServiceList => self.service_list_state.previous(self.services.len()),
                            ActiveScreen::DockerList => self.docker_list_state.previous(self.containers.len()),
                            ActiveScreen::WorkspaceList => {
                                if self.workspace_list_state.focus_items {
                                    if let Some(i) = self.workspace_list_state.table_state.selected() {
                                        if let Some(ws) = self.workspaces.get(i) { self.workspace_list_state.item_previous(ws.items.len()); }
                                    }
                                } else { self.workspace_list_state.previous(self.workspaces.len()); }
                            },
                            ActiveScreen::PortList => self.port_list_state.previous(self.ports.len()),
                            ActiveScreen::LogView => self.log_view_state.scroll_up(),
                            ActiveScreen::ServerList => self.server_list_state.previous(self.servers.len()),
                            _ => {}
                        }
                    }
                    _ => {}
                }
                return Ok(());
            }

            if let Event::Key(key) = event {
                // Command palette overlay
                if self.is_command_palette {
                    match key.code {
                        KeyCode::Esc => {
                            self.is_command_palette = false;
                            self.command_prompt_text.clear();
                        }
                        KeyCode::Enter => {
                            let text = self.command_prompt_text.clone();
                            self.is_command_palette = false;
                            self.command_prompt_text.clear();
                            self.execute_palette_command(&text).await;
                        }
                        KeyCode::Backspace => {
                            self.command_prompt_text.pop();
                        }
                        KeyCode::Char(c) => {
                            self.command_prompt_text.push(c);
                        }
                        _ => {}
                    }
                    return Ok(());
                }

                // Workspace naming prompt overlay
                if self.is_prompting_workspace {
                    match key.code {
                        KeyCode::Esc => {
                            self.is_prompting_workspace = false;
                            self.workspace_prompt_text.clear();
                        }
                        KeyCode::Enter => {
                            let name = self.workspace_prompt_text.clone();
                            if !name.is_empty() {
                                let _ = self.workspace_manager.create_workspace(&name, None).await;
                            }
                            self.is_prompting_workspace = false;
                            self.workspace_prompt_text.clear();
                        }
                        KeyCode::Backspace => {
                            self.workspace_prompt_text.pop();
                        }
                        KeyCode::Char(c) => {
                            self.workspace_prompt_text.push(c);
                        }
                        _ => {}
                    }
                    return Ok(());
                }

                // Global view switching
                if !self.process_list_state.is_searching && !self.port_list_state.is_searching && !self.service_list_state.is_searching && !self.docker_list_state.is_searching && !self.workspace_list_state.is_searching {
                    if key.code == KeyCode::Char('V') || key.code == KeyCode::Char('v') {
                        // Cycle views
                        self.active_screen = match self.active_screen {
                            ActiveScreen::ProcessList => ActiveScreen::ServiceList,
                            ActiveScreen::ProcessDetails => ActiveScreen::ServiceList,
                            ActiveScreen::ServiceList => ActiveScreen::DockerList,
                            ActiveScreen::DockerList => ActiveScreen::WorkspaceList,
                            ActiveScreen::WorkspaceList => ActiveScreen::PortList,
                            ActiveScreen::PortList => ActiveScreen::ServerList,
                            ActiveScreen::ServerList => ActiveScreen::ProcessList,
                            ActiveScreen::LogView => self.previous_screen,
                        };
                        return Ok(());
                    }
                    if key.code == KeyCode::Char(':') {
                        self.is_command_palette = true;
                        self.command_prompt_text.clear();
                        return Ok(());
                    }
                }

                match self.active_screen {
                    ActiveScreen::ProcessList => self.handle_process_list_events(key).await,
                    ActiveScreen::ProcessDetails => self.handle_process_details_events(key),
                    ActiveScreen::PortList => self.handle_port_list_events(key),
                    ActiveScreen::ServiceList => self.handle_service_list_events(key).await,
                    ActiveScreen::DockerList => self.handle_docker_list_events(key).await,
                    ActiveScreen::WorkspaceList => self.handle_workspace_list_events(key).await,
                    ActiveScreen::LogView => self.handle_log_view_events(key).await,
                    ActiveScreen::ServerList => self.handle_server_list_events(key),
                }
            }
        }
        Ok(())
    }

    async fn execute_palette_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() { return; }

        let cmd = parts[0].to_lowercase();
        let target = if parts.len() > 1 { parts[1..].join(" ").to_lowercase() } else { String::new() };

        match cmd.as_str() {
            "kill" => {
                if let Some(p) = self.processes.iter().find(|p| p.name.to_lowercase().contains(&target)) {
                    let _ = self.process_manager.kill_process(p.pid, Some(&self.ssh_manager));
                    self.notify(format!("Killed process {} ({})", p.name, p.pid));
                } else {
                    self.notify(format!("Process '{}' not found", target));
                }
            }
            "restart" => {
                // Check docker first, then services
                if let Some(c) = self.containers.iter().find(|c| c.name.to_lowercase().contains(&target)) {
                    let _ = self.docker_manager.restart_container(&c.name).await;
                    self.notify(format!("Restarted container {}", c.name));
                } else if let Some(s) = self.services.iter().find(|s| s.name.to_lowercase().contains(&target)) {
                    let _ = self.service_manager.restart_service(&s.name);
                    self.notify(format!("Restarted service {}", s.name));
                } else {
                    self.notify(format!("Target '{}' not found in containers or services", target));
                }
            }
            "logs" => {
                if let Some(c) = self.containers.iter().find(|c| c.name.to_lowercase().contains(&target)) {
                    self.log_view_state = LogViewState::new();
                    self.log_view_state.container_id = Some(c.id.clone());
                    self.log_view_state.title = c.name.clone();
                    if let Ok(logs) = self.docker_manager.get_container_logs(&c.id, 100).await {
                        self.log_view_state.set_logs(logs);
                    }
                    self.previous_screen = self.active_screen;
                    self.active_screen = ActiveScreen::LogView;
                } else if let Some(s) = self.services.iter().find(|s| s.name.to_lowercase().contains(&target)) {
                    self.log_view_state = LogViewState::new();
                    self.log_view_state.service_name = Some(s.name.clone());
                    self.log_view_state.title = s.name.clone();
                    if let Ok(logs) = self.service_manager.get_service_logs(&s.name, 100) {
                        self.log_view_state.set_logs(logs);
                    }
                    self.previous_screen = self.active_screen;
                    self.active_screen = ActiveScreen::LogView;
                } else {
                    self.notify(format!("Logs for '{}' not found", target));
                }
            }
            "quit" | "q" => self.should_quit = true,
            _ => {
                self.notify(format!("Unknown command: {}", cmd));
            }
        }
    }
}
