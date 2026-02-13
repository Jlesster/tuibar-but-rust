use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use tokio::time::{Duration, interval};

mod app;
mod config;
mod hyprland;
mod modules;
mod styles;
mod system;
mod ui;
use app::App;
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //setup term
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = config::Config::load()?;

    //app init
    let mut app = App::new();

    let mut tick_interval = interval(Duration::from_millis(config.refresh_interval_ms));

    loop {
        //draw handle here VV
        terminal.draw(|f| render_ui(f, &app))?;

        //Handle events with timeout VV
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        tokio::select! {
            _ = tick_interval.tick() => {
                app.update()?;
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
