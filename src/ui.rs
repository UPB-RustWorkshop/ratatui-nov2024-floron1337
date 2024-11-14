use std::{rc::Rc};

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, symbols::line::TOP_RIGHT, text::{Line, Span}, widgets::{block::title, Block, BorderType, Borders, Paragraph}, Frame};
use crate::app::App;

fn generate_temp_string(temp: &i32){

}

fn render_app_title(frame: &mut Frame, chunk: Rect){
    let app_title_block = Paragraph::new("Rusty Weather")
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(app_title_block, chunk);
}
fn render_weather_info(frame: &mut Frame, cityName: &str, chunk: Rect){
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(10), // Each paragraph takes up 1/3 of the height
        Constraint::Percentage(70),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ])
    .split(chunk);

    let city_name_block = Paragraph::new(cityName)
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
        
    let city_temp_block = Paragraph::new("
    000000000               888888888                  CCCCCCCCCCCCC
   00:::::::::00           88:::::::::88             CCC::::::::::::C
 00:::::::::::::00       88:::::::::::::88         CC:::::::::::::::C
0:::::::000:::::::0     8::::::88888::::::8       C:::::CCCCCCCC::::C
0::::::0   0::::::0     8:::::8     8:::::8      C:::::C       CCCCCC
0:::::0     0:::::0     8:::::8     8:::::8     C:::::C              
0:::::0     0:::::0      8:::::88888:::::8      C:::::C              
0:::::0 000 0:::::0       8:::::::::::::8       C:::::C              
0:::::0 000 0:::::0      8:::::88888:::::8      C:::::C              
0:::::0     0:::::0     8:::::8     8:::::8     C:::::C              
0:::::0     0:::::0     8:::::8     8:::::8     C:::::C              
0::::::0   0::::::0     8:::::8     8:::::8      C:::::C       CCCCCC
0:::::::000:::::::0     8::::::88888::::::8       C:::::CCCCCCCC::::C
 00:::::::::::::00       88:::::::::::::88         CC:::::::::::::::C
   00:::::::::00           88:::::::::88             CCC::::::::::::C
     000000000               888888888                  CCCCCCCCCCCCC")
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    
    let city_temp_feels_block = Paragraph::new("\nFeels like 5 C")
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
fn render_user_input(frame: &mut Frame, chunk: Rect){
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

    let input_text = Paragraph::new("Bucharest")
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));

    frame.render_widget(input_header, chunks[0]);
    frame.render_widget(input_text, chunks[1]);
}
fn render_help(frame: &mut Frame, chunk: Rect){
    let help_block = Paragraph::new("HELP\nPress Space to change city\nPress Q to exit\n")
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
        Constraint::Percentage(10),
        Constraint::Percentage(65), // Top half of the screen
        Constraint::Percentage(15), // Bottom half of the screen
        Constraint::Percentage(10), // Bottom half of the screen
    ])
    .split(frame.area());
    
    let city_name = "Cogealac";
    let temp: i32 = 12;

    render_app_title(frame, chunks[0]);
    render_weather_info(frame, city_name, chunks[1]);
    render_user_input(frame, chunks[2]);
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
