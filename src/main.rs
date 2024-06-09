mod weather_request;

use reqwest::Client;
use tokio;
use std::io::{self, Write};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use eframe::egui::{CentralPanel, CtxRef};
use eframe::epi::{App, Frame};
use eframe::{egui, NativeOptions, run_native};
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use crate::egui::mutex::MutexGuard;
use crate::weather_request::WeatherRequest;


#[tokio::main]
async fn main() {
    /*
    let mut prompt: bool = true;
    let mut input:String = String::new();
    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();
    let mut forecaster = WeatherRequest {
        grid_x : "".parse().unwrap(),
        grid_y : "".parse().unwrap(),
        grid_id : "".parse().unwrap()
    };
    while prompt {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        split_words = input.trim().split_whitespace().collect();
        if split_words.len() < 1 { continue }
        match split_words[0] {
            "location" => {forecaster.get_location(split_words[1], split_words[2]).await}
            "day" => {forecaster.get_is_day().await}
            "temperature" => {forecaster.get_temperature().await}
            "json" => {forecaster.get_json().await}
            "exit" => {prompt =false}
            &_ => {}
        }
    }
    */


    let app = Weather::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}

async fn get_text() -> String{
    "check".to_string()
}

async fn async_operation() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "Async operation result".to_string()
}

async fn get_temperature(lat : &str, lon : &str) -> String {
    println!("lat : {}, lon : {}", lat, lon);
    let mut temp = WeatherRequest {
        grid_x : "".to_string(),
        grid_y : "".to_string(),
        grid_id : "".to_string()
    };
    temp.update_location(lat, lon).await;
    temp.get_temperature().await
}

struct Weather {
    async_runtime: Runtime,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
    message: Arc<Mutex<String>>,
    temperature: Arc<Mutex<String>>,
    latitude_input: String,
    longitude_input : String
}

impl Weather{
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let message = Arc::new(Mutex::new(String::new()));
        let temperature = Arc::new(Mutex::new(String::new()));

        let runtime = Runtime::new().unwrap();

        Self {
            async_runtime: runtime,
            sender,
            receiver,
            message,
            temperature,
            latitude_input : "".to_string(),
            longitude_input : "".to_string()
        }
    }


}


impl App for Weather{
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        if let Ok(result) = self.receiver.try_recv() {
            let mut msg = self.temperature.lock().unwrap();
            *msg = result;
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enter latitude and longitude to get temperature");
            //ui.label("This is a simple eframe application.");
            let msg = self.temperature.lock().unwrap();
            ui.label("Temperature : ".to_owned() + msg.as_str()); // Display the message
            ui.text_edit_singleline(&mut self.latitude_input);
            ui.text_edit_singleline(&mut self.longitude_input);
            if ui.button("Update Message").clicked() {
                let sender = self.sender.clone();
                let message_clone = Arc::clone(&self.temperature);
                let longitude_input = self.longitude_input.clone();
                let latitude_input = self.latitude_input.clone();
                self.async_runtime.spawn(async move {
                    let result = get_temperature(&latitude_input, &longitude_input).await;
                    sender.send(result).await.unwrap();
                    let mut msg = message_clone.lock().unwrap();
                    *msg = "Async operation completed!".to_string();
                });
            }
        });

        //println!("{}", self.message.lock().unwrap());
    }

    fn name(&self) -> &str {
        "Weather"
    }
}
