use crate::{app::{App, AppResult}, connection::{get_data, CityInfo, Weather}};
use crossterm::event::{KeyEvent, KeyCode};
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
                let city = app.selected_city.clone();

                let result = task::block_in_place(|| get_data(&city));
               
                // Spawn a blocking task to fetch data
                match result {
                    Ok(city_info) => {
                        // Successfully fetched and parsed data
                        app.city_info = city_info;
                    }
                    Err(_) => {
                        // Handle the error (e.g., log or set default value)
                        app.city_info = CityInfo {
                                            weather_list: vec![Weather {
                                                temp: 1337.0,
                                                feels_like: 0.0,
                                                weather: "Invalid City".to_string(),
                                            }],
                                        }
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if(app.selected_city == "".to_string() && app.user_input.len() > 0){
                app.user_input.pop();
            }
        }
        _ => {}
    }
    Ok(())
}