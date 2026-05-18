mod app;
mod help;
mod model;
mod parser;
mod ui;
mod writer;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser as ClapParser;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;

#[derive(ClapParser)]
#[command(
    name = "kfs-squid-editor",
    version,
    about = "TUI editor for Squid proxy configuration"
)]
struct Cli {
    /// Path to squid.conf file
    #[arg(default_value = "/etc/squid/squid.conf")]
    config: PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let config = if cli.config.exists() {
        let content = std::fs::read_to_string(&cli.config)?;
        parser::parse(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
    } else {
        model::SquidConfig::default()
    };

    let mut app = app::App::new(config, cli.config);

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<impl Backend>, app: &mut app::App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            if app.tab == app::Tab::Auth && app.screen == app::Screen::List {
                app.handle_auth_key(key);
            } else {
                app.handle_key(key);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
