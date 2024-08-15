
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
use std::time::{Duration, Instant};

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
    let mut last_key_time: Option<Instant> = None;
    let mut last_key_code: Option<KeyCode> = None;

    let mut forecaster = WeatherRequest::new();
    forecaster.update_location("29.551900", "-95.098080").await;
    let mut database = LocationJSON::new().unwrap();
    //forecaster.get_location().await;

    let mut input = String::new();

    let mut should_quit = false;
    while !should_quit {
        let weather_data = WeatherData{
            grid_x: forecaster.get_grid_x().to_string(),
            grid_y: forecaster.get_grid_y().to_string(),
            grid_id: forecaster.get_grid_id().to_string(),
            temperature : forecaster.get_temperature().await,
            short_forecast : forecaster.get_short_forecast().await,
            detailed_forecast : forecaster.get_detailed_forecast().await
        };
        terminal.draw(|frame| ui(frame, &weather_data, input.as_str()))?;
        should_quit = handle_events(&mut input, &mut last_key_time, &mut last_key_code, &mut forecaster, &mut database).await?;
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

async fn handle_events(input: &mut String, last_key_time: &mut Option<Instant>, last_key_code: &mut Option<KeyCode>, forecaster: &mut WeatherRequest, database: &mut LocationJSON) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(1))? {
        if let Event::Key(key) = event::read()? {
            let now = Instant::now();
            let debounce_time = Duration::from_millis(200);
            if Some(key.code) != *last_key_code || last_key_time.map_or(true, |t| now.duration_since(t) > debounce_time) {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        // Process the input when Enter is pressed
                        process_command(&input, forecaster, database).await;
                        input.clear(); // Clear the input buffer
                    }
                    KeyCode::Esc => {
                        return Ok(true);// Exit on Esc key press
                    }
                    _ => {}
                }

                *last_key_code = Some(key.code.clone());
                *last_key_time = Some(now);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, weather_data: &WeatherData, input : &str){

    let areas = Layout::default().direction(Direction::Vertical).margin(1).constraints([Constraint::Fill(1), Constraint::Length(5)].as_ref()).split(frame.area());
    let forecast_split = Layout::default().direction(Direction::Horizontal).margin(1).constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref()).split(areas[0]);
    let loc_split = Layout::default().direction(Direction::Vertical).margin(1).constraints([Constraint::Fill(1), Constraint::Length(5)].as_ref()).split(forecast_split[0]);

    let forecast_area = loc_split[0];
    let location_area = loc_split[1];
    let terminal_area = areas[1];
    let location_json_area = forecast_split[1];


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

    let terminal = Block::default().borders(Borders::ALL).title("Terminal");
    let terminal_widget = Paragraph::new(Text::from(input))
        .block(terminal)
        .style(Style::default().fg(Color::Yellow));

    let forecast = Paragraph::new(Text::from(forecast_text)).wrap(Wrap{trim : true}).block(Block::default().title("Forecast").borders(Borders::ALL));
    frame.render_widget(forecast, forecast_area);

    let location_json = Paragraph::new("Hello World").block(Block::default().title("Greeting").borders(Borders::ALL));
    frame.render_widget(location_json, location_json_area);

    let location = Paragraph::new(Text::from(grid_location_text)).wrap(Wrap{trim : false}).block(Block::default().title("Location").borders(Borders::ALL));
    frame.render_widget(location, location_area);


    frame.render_widget(terminal_widget, terminal_area)


}

async fn process_command(input: &str, forecaster: &mut WeatherRequest, mut database: &mut LocationJSON) {
    // Parse the input using clap
    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();

    if split_words.len() < 1 { } else {
        match split_words[0] {
            "add_location" => {database.add_location(split_words[1], split_words[2], split_words[3]); let _ = database.save_to_json();}
            "switch_location" => {
                if database.contains_location(split_words[1]) {
                    forecaster.update_location(database.get_lat(split_words[1]), database.get_lon(split_words[1])).await
                }
            }
            "update" => {forecaster.update_location(split_words[1], split_words[2]).await}
            "exit" => {}
            "increment" => {forecaster.increment_time_period()}
            "decrement" => {forecaster.decrement_time_period()}
            &_ => {}
        }
    }

}



