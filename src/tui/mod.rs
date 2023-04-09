use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub use app::AppResult;

mod app;
mod event;
mod handler;
mod tui;
mod ui;

pub fn run() -> AppResult<()> {
    let mut app = app::App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = event::EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            event::Event::Tick => app.tick(),
            event::Event::Key(key_event) => handler::handle_key_events(key_event, &mut app)?,
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
