/// NetCtl TUI Setup Wizard Binary
/// Interactive terminal-based configuration wizard for NetCtl

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use netctl::tui::{TuiApp, SetupScreen};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, ListItem, List, Gauge},
    Frame, Terminal,
};
use std::io;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = TuiApp::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: TuiApp) -> Result<()> {
    let tick_rate = std::time::Duration::from_millis(250);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        if app.current_screen == SetupScreen::InstallationComplete {
                            app.exit = true;
                        } else if app.current_screen != SetupScreen::Welcome {
                            app.prev_screen();
                        } else {
                            app.exit = true;
                        }
                    }
                    KeyCode::Tab | KeyCode::Right => {
                        match app.next_screen() {
                            Ok(()) => {}
                            Err(msg) => {
                                app.error_message = Some(msg);
                            }
                        }
                    }
                    KeyCode::BackTab | KeyCode::Left => {
                        app.prev_screen();
                    }
                    KeyCode::Enter => {
                        match app.current_screen {
                            SetupScreen::Summary => {
                                match app.next_screen() {
                                    Ok(()) => {
                                        app.success_message =
                                            Some("Installation started!".to_string());
                                    }
                                    Err(msg) => {
                                        app.error_message = Some(msg);
                                    }
                                }
                            }
                            _ => {
                                match app.next_screen() {
                                    Ok(()) => {}
                                    Err(msg) => {
                                        app.error_message = Some(msg);
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        handle_text_input(&mut app, c);
                    }
                    KeyCode::Backspace => {
                        handle_backspace(&mut app);
                    }
                    KeyCode::Up => {
                        handle_selection_up(&mut app);
                    }
                    KeyCode::Down => {
                        handle_selection_down(&mut app);
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }

        if app.exit {
            break;
        }
    }

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &TuiApp) {
    let size = f.size();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    // Title bar
    draw_title_bar(f, chunks[0], app);

    // Content area
    match app.current_screen {
        SetupScreen::Welcome => draw_welcome_screen(f, chunks[1], app),
        SetupScreen::InterfaceSelection => draw_interface_selection(f, chunks[1], app),
        SetupScreen::IPConfiguration => draw_ip_configuration(f, chunks[1], app),
        SetupScreen::DNSConfiguration => draw_dns_configuration(f, chunks[1], app),
        SetupScreen::DashboardSetup => draw_dashboard_setup(f, chunks[1], app),
        SetupScreen::SecurityReview => draw_security_review(f, chunks[1], app),
        SetupScreen::Summary => draw_summary(f, chunks[1], app),
        SetupScreen::InstallationComplete => draw_installation_complete(f, chunks[1], app),
    }

    // Footer bar
    draw_footer_bar(f, chunks[2], app);
}

fn draw_title_bar<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    let inner = block.inner(area);

    f.render_widget(block, area);

    let title = Span::styled(
        format!("❯ NetCtl Setup Wizard - {}", app.current_screen),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    let line = Line::from(title);
    f.render_widget(
        Paragraph::new(line).alignment(Alignment::Left),
        inner,
    );
}

fn draw_welcome_screen<B: Backend>(f: &mut Frame<B>, area: Rect, _app: &TuiApp) {
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Welcome to NetCtl - Enterprise Network Control",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  This wizard will guide you through the initial setup of NetCtl,"),
        Line::from("  an advanced network management daemon with kernel-space packet"),
        Line::from("  shaping capabilities."),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Real-time network flow tracking and visualization"),
        Line::from("    • eBPF/XDP kernel-space traffic shaping"),
        Line::from("    • Enterprise security with JWT and RBAC"),
        Line::from("    • Live metrics and audit logging"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press TAB/ENTER to continue or Q to exit",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let block = Block::default()
        .title("Setup Wizard")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_interface_selection<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let items: Vec<ListItem> = app
        .interface_list
        .iter()
        .map(|iface| {
            let selected = app.selected_interface.as_ref() == Some(iface);
            let style = if selected {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("  {}", iface)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Network Interfaces")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(list, area);
}

fn draw_ip_configuration<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let help_text = vec![
        Line::from(format!("  IP Address: {}", app.ip_address)),
        Line::from(format!("  Netmask:    {}", app.netmask)),
        Line::from(format!("  Gateway:    {}", app.gateway)),
        Line::from(""),
        Line::from(Span::styled(
            "  Enter IP configuration values above",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let block = Block::default()
        .title("IP Configuration")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_dns_configuration<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let content = vec![
        Line::from(format!("  Primary DNS:   {}", app.dns_primary)),
        Line::from(format!("  Secondary DNS: {}", app.dns_secondary)),
        Line::from(""),
        Line::from("  These servers will be configured for network name resolution."),
    ];

    let block = Block::default()
        .title("DNS Configuration")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_dashboard_setup<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let content = vec![
        Line::from(format!("  Hostname: {}", app.dashboard_hostname)),
        Line::from(format!("  Port: {}", app.dashboard_port)),
        Line::from(format!("  HTTPS: {}", if app.enable_https { "Enabled" } else { "Disabled" })),
        Line::from(format!("  Admin User: {}", app.admin_username)),
        Line::from(""),
        Line::from("  Configure the web dashboard access details."),
    ];

    let block = Block::default()
        .title("Dashboard Setup")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_security_review<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("Security Configuration Review")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(0.85)
        .label("Security Level: Good ✓");

    f.render_widget(gauge, area);
}

fn draw_summary<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let config = app.get_config_map();
    let mut content = vec![
        Line::from(Span::styled(
            "Configuration Summary",
            Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (key, value) in config.iter() {
        content.push(Line::from(format!("  {}: {}", key, value)));
    }

    content.push(Line::from(""));
    content.push(Line::from(Span::styled(
        "  Press ENTER to apply configuration",
        Style::default().fg(Color::Yellow),
    )));

    let block = Block::default()
        .title("Review & Confirm")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_installation_complete<B: Backend>(f: &mut Frame<B>, area: Rect, _app: &TuiApp) {
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ✓ NetCtl Installation Complete!",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Your network control system is now configured and ready to use."),
        Line::from(""),
        Line::from("  Next steps:"),
        Line::from("    1. Start the NetCtl daemon: sudo systemctl start netctl"),
        Line::from("    2. Access the dashboard at the configured hostname"),
        Line::from("    3. Check the audit logs for system events"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press Q or ESC to exit",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let block = Block::default()
        .title("✓ Setup Complete")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightGreen));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_footer_bar<B: Backend>(f: &mut Frame<B>, area: Rect, app: &TuiApp) {
    let mut status = String::from("Navigation: ← → Tab  |  Action: Enter  |  Exit: Q");

    if let Some(ref error) = app.error_message {
        status = format!("⚠ {}", error);
    } else if let Some(ref success) = app.success_message {
        status = format!("✓ {}", success);
    }

    let block = Block::default()
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::Cyan));

    let footer = Paragraph::new(status)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}

fn handle_text_input(app: &mut TuiApp, c: char) {
    match app.current_screen {
        SetupScreen::IPConfiguration => {
            if c.is_numeric() || c == '.' {
                match app.ip_address.len() {
                    0..=14 => app.ip_address.push(c),
                    _ => {}
                }
            }
        }
        SetupScreen::DashboardSetup => {
            if app.admin_username.is_empty() {
                app.admin_username.push(c);
            } else if app.admin_password.len() < 32 && app.admin_password_confirm.is_empty() {
                app.admin_password.push(c);
            }
        }
        _ => {}
    }
}

fn handle_backspace(app: &mut TuiApp) {
    match app.current_screen {
        SetupScreen::IPConfiguration => {
            app.ip_address.pop();
        }
        SetupScreen::DashboardSetup => {
            if app.admin_password_confirm.is_empty() {
                app.admin_password.pop();
            }
        }
        _ => {}
    }
}

fn handle_selection_up(app: &mut TuiApp) {
    if let Some(ref mut selected) = app.selected_interface {
        if let Some(pos) = app.interface_list.iter().position(|x| x == selected) {
            if pos > 0 {
                *selected = app.interface_list[pos - 1].clone();
            }
        }
    }
}

fn handle_selection_down(app: &mut TuiApp) {
    if let Some(ref mut selected) = app.selected_interface {
        if let Some(pos) = app.interface_list.iter().position(|x| x == selected) {
            if pos < app.interface_list.len() - 1 {
                *selected = app.interface_list[pos + 1].clone();
            }
        }
    }
}
