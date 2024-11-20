use std::error;
use tokio::task;
use crate::connection::{get_data, CityInfo, Weather};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub tick_rate: u64,
    pub selected_city: String,
    pub user_input: String,
    pub city_info: CityInfo
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tick_rate: u64) -> Self {
        Self {
            running: true,
            tick_rate,
            selected_city : "Bucharest".to_string(),
            user_input: "".to_string(),
            city_info: match task::block_in_place(|| get_data(&String::from("Bucharest"))){
                Ok(info) => info,
                Err(_) => {
                    CityInfo {
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
}
