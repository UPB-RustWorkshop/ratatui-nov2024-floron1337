use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{BarChart, Block, Borders, Paragraph}, Frame};
use text_to_ascii_art::to_art;
use crate::{app::App};

fn generate_ascii_art(string: String) -> String{
    match to_art(string.to_string(), "", 1, 1, 1) {
        Ok(string) => return string,
        Err(err) => return "ERROR".to_string(),
    }
}

fn render_app_title(frame: &mut Frame, chunk: Rect){
    let app_title_block = Paragraph::new(generate_ascii_art("Rusty Weather".to_string()))
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(app_title_block, chunk);
}

fn render_weather_info(frame: &mut Frame, city_name: &String, city_temp: &i32, city_feels_like_temp: &i32, city_weather: &String, chunk: Rect){
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(20), 
        Constraint::Percentage(50),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
    ])
    .split(chunk);

    let city_name_block = Paragraph::new(format!("\n{}", city_name))
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    
    let city_temp_string = if (*city_temp < 10 && *city_temp > 0) { format!("0{}*C", city_temp.to_string())} else { format!("{}*C", city_temp.to_string())};
    let city_temp_block = Paragraph::new(generate_ascii_art(city_temp_string))
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    
    let city_temp_feels_block = Paragraph::new(format!("\nFeels like {}*C", city_feels_like_temp))
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    let city_weather_block = Paragraph::new(format!("{}", city_weather))
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    frame.render_widget(city_name_block, chunks[0]);
    frame.render_widget(city_temp_block, chunks[1]);
    frame.render_widget(city_temp_feels_block, chunks[2]);
    frame.render_widget(city_weather_block, chunks[3]);
}

fn render_weather_graph(app: &mut App, frame: &mut Frame, chunk : Rect){
    let data: Vec<(&str, f32)> = app.city_info.weather_list
                .iter()
                .enumerate()
                .map(|(index, weather)| {
                    let day = match index {
                        0 => "D+0",
                        1 => "D+1",
                        2 => "D+2",
                        3 => "D+3",
                        4 => "D+4",
                        5 => "D+5",
                        6 => "D+6",
                        _ => "Unknown",
                    };
                    (day, weather.temp) // Tuple with day and temperature
                })
                .collect();

    let int_data: Vec<(&str, u64)> = data
        .iter()
        .map(|(day, value)| (*day, *value as u64)) // Convert f32 to u64
        .collect();

    let barchart = BarChart::default()
        .block(Block::default().title("This Week's Temperatures").borders(Borders::NONE))
        .data(&int_data)
        .bar_width(5)
        .bar_gap(2)
        .value_style(Style::default().fg(Color::White))
        .label_style(Style::default().fg(Color::White))
        .bar_style(Style::default().fg(Color::Cyan));
    frame.render_widget(barchart, chunk);
}

fn render_user_input(frame: &mut Frame, input: String,  chunk: Rect){
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(50), // Each paragraph takes up 1/3 of the height
        Constraint::Percentage(50),
    ])
    .split(chunk);

    let input_header = Paragraph::new("Enter your city: ")
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::TOP));

    let input_text = Paragraph::new(input)
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));

    frame.render_widget(input_header, chunks[0]);
    frame.render_widget(input_text, chunks[1]);
}

fn render_help(frame: &mut Frame, chunk: Rect){
    let help_block = Paragraph::new("HELP\nPress TAB to change city\nPress ESC to exit\n")
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));
    frame.render_widget(help_block, chunk);
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/main/ratatui/examples

    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(25),
        Constraint::Percentage(50), 
        Constraint::Percentage(15),
        Constraint::Percentage(10), 
    ])
    .split(frame.area());
    
    let weather_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

    let city_name = &app.selected_city;
    let city_temp: i32 = app.city_info.weather_list[0].temp.round() as i32;
    let city_feels_like_temp: i32 = app.city_info.weather_list[0].feels_like.round() as i32;
    let city_weather = app.city_info.weather_list[0].weather.clone();
    
    render_app_title(frame, chunks[0]);
    if app.selected_city != "" {
        render_weather_info(frame, &city_name, &city_temp, &city_feels_like_temp, &city_weather, weather_chunks[0]);
        render_weather_graph(app, frame, weather_chunks[1]);
    }
    render_user_input(frame, app.user_input.clone(), chunks[2]);
    render_help(frame, chunks[3]);
}
