use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use crossterm::event::{self};
use std::io::{self};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    
    // Create an application.
    const APP_TICK_RATE: u64 = 200;
    let mut app: App = App::new(APP_TICK_RATE);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;


    // TODO:  the terminal user interface
    let handler:EventHandler = EventHandler::new(app.tick_rate);
    let mut tui = Tui::new(terminal, handler);

    // TODO: init the terminal
    Tui::init(&mut tui)?;

    // Start the main loop.
    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);

        // TODO: Handle events.
        let event = event::read()?;
        if let crossterm::event::Event::Key(key) = event {
            handle_key_events(key, &mut app);            
        }
    }
    // TODO: Reset the terminal if the app has been terminated
    tui.exit();
    Ok(())
}
