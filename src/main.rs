
use reqwest::Client;
use tokio;
use std::io::{self, Write};
use weather_forecasting::weather_request::WeatherRequest;


#[tokio::main]
async fn main() {

    let mut prompt: bool = true;
    let mut input:String = String::new();
    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();
    let mut forecaster = WeatherRequest::new();
    while prompt {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        split_words = input.trim().split_whitespace().collect();
        if split_words.len() < 1 { continue }
        match split_words[0] {
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
}




