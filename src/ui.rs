use ratatui::{style::Stylize, widgets::Paragraph, Frame};
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/main/ratatui/examples
    let greeting = Paragraph::new("hello")
                .white()
                .on_blue();
    frame.render_widget(greeting, frame.size());

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
