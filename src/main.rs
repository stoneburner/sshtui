//! SSH config TUI: list hosts from ~/.ssh/config and connect with Enter.

mod config;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::process::Command;

use config::{parse_config, HostEntry};

#[derive(Parser, Debug)]
#[command(name = "sshtui")]
#[command(about = "TUI to pick an SSH host from config and connect")]
struct Args {
    /// Path to SSH config file (default: ~/.ssh/config)
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn default_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".ssh").join("config")
}

fn run_ssh(host: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new("ssh").arg(host).status()
}

struct App {
    hosts: Vec<HostEntry>,
    list_state: ListState,
    quit: bool,
    connect: Option<String>,
}

impl App {
    fn new(hosts: Vec<HostEntry>) -> Self {
        let mut list_state = ListState::default();
        if !hosts.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            hosts,
            list_state,
            quit: false,
            connect: None,
        }
    }

    fn next(&mut self) {
        let i = self
            .list_state
            .selected()
            .map(|i| (i + 1).min(self.hosts.len().saturating_sub(1)))
            .or(Some(0));
        self.list_state.select(i);
    }

    fn previous(&mut self) {
        let i = self
            .list_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .or(Some(0));
        self.list_state.select(i);
    }

    fn connect_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.hosts.get(i) {
                self.connect = Some(entry.ssh_name().to_string());
                self.quit = true;
            }
        }
    }

    fn selected_entry(&self) -> Option<&HostEntry> {
        self.list_state.selected().and_then(|i| self.hosts.get(i))
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[0]);

    let items: Vec<ListItem> = app
        .hosts
        .iter()
        .map(|e| ListItem::new(Line::from(Span::raw(e.display_name()))))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Hosts ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(" > ");

    f.render_stateful_widget(list, main_chunks[0], &mut app.list_state);

    let details_text = if let Some(entry) = app.selected_entry() {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Host: ", Style::default().fg(Color::Yellow)),
                Span::raw(entry.display_name()),
            ]),
            Line::from(vec![
                Span::styled("Hostname: ", Style::default().fg(Color::Yellow)),
                Span::raw(entry.hostname.as_deref().unwrap_or("-")),
            ]),
            Line::from(vec![
                Span::styled("User: ", Style::default().fg(Color::Yellow)),
                Span::raw(entry.user.as_deref().unwrap_or("-")),
            ]),
            Line::from(vec![
                Span::styled("Port: ", Style::default().fg(Color::Yellow)),
                Span::raw(
                    entry
                        .port
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "22 (default)".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("IdentityFile: ", Style::default().fg(Color::Yellow)),
                Span::raw(entry.identity_file.as_deref().unwrap_or("-")),
            ]),
        ];
        for (k, v) in &entry.extra {
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", k), Style::default().fg(Color::DarkGray)),
                Span::raw(v.as_str()),
            ]));
        }
        lines
    } else {
        vec![Line::from(Span::raw("No host selected"))]
    };

    let details = Paragraph::new(details_text)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(details, main_chunks[1]);

    let help = Paragraph::new(Line::from(vec![
        Span::styled("j/Down: ", Style::default().fg(Color::DarkGray)),
        Span::raw("down  "),
        Span::styled("k/Up: ", Style::default().fg(Color::DarkGray)),
        Span::raw("up  "),
        Span::styled("Enter: ", Style::default().fg(Color::DarkGray)),
        Span::raw("connect  "),
        Span::styled("q/Esc: ", Style::default().fg(Color::DarkGray)),
        Span::raw("quit"),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Keys "));

    f.render_widget(help, chunks[1]);
}

fn run_tui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.quit = true;
                    break;
                }
                KeyCode::Char('j') | KeyCode::Down => app.next(),
                KeyCode::Char('k') | KeyCode::Up => app.previous(),
                KeyCode::Enter => {
                    app.connect_selected();
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let config_path = args.config.unwrap_or_else(default_config_path);

    let hosts = parse_config(&config_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read config {:?}: {}", config_path, e),
        )
    })?;

    if hosts.is_empty() {
        eprintln!("No connectable hosts found in {:?}", config_path);
        eprintln!("(Wildcard-only Host blocks like 'Host *' are skipped.)");
        std::process::exit(1);
    }

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout))?;

    let mut app = App::new(hosts);
    let result = run_tui(&mut terminal, &mut app);

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        return Err(e);
    }

    if let Some(host) = app.connect {
        drop(terminal);
        run_ssh(&host)?;
    }

    Ok(())
}
