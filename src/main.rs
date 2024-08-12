
use reqwest::Client;
use tokio;
use std::io::{self, stdout, Write};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use weather_forecasting::weather_request::WeatherRequest;
use weather_forecasting::json_writer::LocationJSON;

struct WeatherData {
    grid_x : String,
    grid_y : String,
    grid_id : String,
    temperature : String,
    short_forecast : String,
    detailed_forecast : String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut forecaster = WeatherRequest::new();
    forecaster.update_location("29.551900", "-95.098080").await;
    //forecaster.get_location().await;
    let weather_data = WeatherData{
        grid_x: forecaster.get_grid_x().to_string(),
        grid_y: forecaster.get_grid_y().to_string(),
        grid_id: forecaster.get_grid_id().to_string(),
        temperature : forecaster.get_temperature().await,
        short_forecast : forecaster.get_short_forecast().await,
        detailed_forecast : forecaster.get_detailed_forecast().await
    };


    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &weather_data))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())

        /*
    let mut prompt: bool = true;
    let mut input:String = String::new();
    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();
    let mut forecaster = WeatherRequest::new();
    let mut database = LocationJSON::new().unwrap();
    while prompt {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        split_words = input.trim().split_whitespace().collect();
        if split_words.len() < 1 { continue }
        match split_words[0] {
            "add_location" => {database.add_location(split_words[1], split_words[2], split_words[3]); let _ = database.save_to_json();}
            "get_location" => {database.get_locations()}
            "switch_location" => {
                if database.contains_location(split_words[1]) {
                    forecaster.update_location(database.get_lat(split_words[1]), database.get_lon(split_words[1])).await
                }
            }
            "update" => {forecaster.update_location(split_words[1], split_words[2]).await}
            "location" => {forecaster.get_location().await}
            "day" => {forecaster.get_is_day().await}
            "temperature" => {forecaster.get_temperature().await}
            "json" => {forecaster.get_json().await}
            "exit" => {prompt = false}
            "short_forecast" => {forecaster.get_short_forecast().await}
            "detailed_forecast" => {forecaster.get_detailed_forecast().await}
            "increment" => {forecaster.increment_time_period()}
            "decrement" => {forecaster.decrement_time_period()}
            &_ => {}
        }
    }
    */

}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, weather_data: &WeatherData){

    let areas = Layout::default().direction(Direction::Vertical).margin(1).constraints([Constraint::Fill(1), Constraint::Length(5)].as_ref()).split(frame.area());
    let forecast_split = Layout::default().direction(Direction::Horizontal).margin(1).constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref()).split(areas[0]);

    let forecast_text = vec![
        Line::from(Span::styled(
            format!("Temperature : {}", weather_data.temperature),
            Style::default().fg(Color::LightRed),
        )),
        Line::from(Span::styled(
            format!("Short Forecast : {}", weather_data.short_forecast),
            Style::default().fg(Color::LightRed),
        )),
        Line::from(Span::styled(
            format!("Detailed Forecast : {}", weather_data.detailed_forecast),
            Style::default().fg(Color::LightRed),
        )),
    ];


    let grid_location_text = vec![
        Line::from(Span::styled(
            format!("Grid X: {}", weather_data.grid_x),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("Grid Y: {}", weather_data.grid_y),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("Grid ID: {}", weather_data.grid_id),
            Style::default().fg(Color::Green),
        )),
    ];

    let text1 = Paragraph::new(Text::from(forecast_text)).wrap(Wrap{trim : true}).block(Block::default().title("Forecast").borders(Borders::ALL));
    frame.render_widget(text1, forecast_split[0]);

    let text2 = Paragraph::new("Hello World").block(Block::default().title("Greeting").borders(Borders::ALL));
    frame.render_widget(text2, forecast_split[1]);

    let location = Paragraph::new(Text::from(grid_location_text)).wrap(Wrap{trim : false}).block(Block::default().title("Location").borders(Borders::ALL));
    frame.render_widget(location, areas[1]);




}



