use std::{fmt::format, rc::Rc};
use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, symbols::line::TOP_RIGHT, text::{Line, Span}, widgets::{block::title, BarChart, Block, BorderType, Borders, Paragraph}, Frame};
use text_to_ascii_art::to_art;
use crate::app::App;

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

fn render_weather_info(frame: &mut Frame, city_name: &String, city_temp: i32, city_feels_like_temp: i32, city_weather: String, chunk: Rect){
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
    
    let city_temp_string = if city_temp < 10 { format!("0{}*C", city_temp.to_string())} else { format!("{}*C", city_temp.to_string())};
    let city_temp_block = Paragraph::new(generate_ascii_art(city_temp_string))
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    
    let city_temp_feels_block = Paragraph::new(format!("\nFeels like {}*C", city_feels_like_temp))
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    let city_weather_block = Paragraph::new("Rain")
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

    frame.render_widget(city_name_block, chunks[0]);
    frame.render_widget(city_temp_block, chunks[1]);
    frame.render_widget(city_temp_feels_block, chunks[2]);
    frame.render_widget(city_weather_block, chunks[3]);
}

fn render_weather_graph(frame: &mut Frame, chunk : Rect){
    let data = [
                ("Mon", 5),
                ("Tue", 3),
                ("Wed", 8),
                ("Thu", 2),
                ("Fri", 6),
                ("Sat", 7),
                ("Sun", 4),
            ];
    let barchart = BarChart::default()
        .block(Block::default().title("This Week's Temperatures").borders(Borders::NONE))
        .data(&data)
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
    let city_temp: i32 = 12;
    let city_feels_like_temp: i32 = 12;
    let city_weather = "Muing".to_string();

    render_app_title(frame, chunks[0]);
    render_weather_info(frame, city_name, city_temp, city_feels_like_temp, city_weather, weather_chunks[0]);
    render_weather_graph(frame, weather_chunks[1]);
    render_user_input(frame, app.user_input.clone(), chunks[2]);
    render_help(frame, chunks[3]);
    // TODO: Split the layout
    // let [area1, area2, area3 ...] =

    // TODO: get the list of cities
    // let cities: Vec<ListItem> =
    // let list_component =

    // TODO: render the list of cities
    // frame.render_widget(list_component, area);


    // TODO: Create the weather info component
    // let weather_info =

    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);


}
