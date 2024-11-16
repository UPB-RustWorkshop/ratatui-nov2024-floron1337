use std::any;

use crate::{app::{App, AppResult}, connection::get_data};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use tokio::task;
/// Handles the key events and updates the state of [`App`].
/// 
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.running = false,
        KeyCode::Tab => {
            app.selected_city = "".to_string();
            app.user_input = "".to_string();
        }
        KeyCode::Char(c) => {
            if(app.selected_city == "".to_string()){
                app.user_input.push(c);
            }
        }
        KeyCode::Enter => {
            if(app.selected_city == "".to_string()){
                app.selected_city = app.user_input.clone();
                //app.user_input = String::from("");
                let city = app.selected_city.clone();

                let result = task::block_in_place(|| get_data(&city));
                // Spawn a blocking task to fetch data
                app.user_input = result;
            }
        }
        KeyCode::Backspace => {
            if(app.selected_city == "".to_string() && app.user_input.len() > 0){
                app.user_input.pop();
            }
        }
        // TODO: define actions for apps functionalities
        _ => {}
    }
    Ok(())
}