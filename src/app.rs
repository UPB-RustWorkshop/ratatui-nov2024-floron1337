use std::error;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub tick_rate: u64,
    pub selected_city: String,
    pub user_input: String
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tick_rate: u64) -> Self {
        Self {
            running: true,
            tick_rate,
            selected_city : "Bucharest".to_string(),
            user_input: "".to_string()
        }
    }
}
