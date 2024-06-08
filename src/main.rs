use reqwest::Client;
use tokio;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut prompt: bool = true;
    let mut input:String = String::new();
    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();
    let mut grid_x = "".to_string();
    let mut grid_y = "".to_string();
    let mut grid_id = "".to_string();
    while prompt {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        split_words = input.trim().split_whitespace().collect();
        if split_words.len() < 1 { continue }
        match split_words[0] {
            "location" => {(grid_x, grid_y, grid_id) = get_location(&split_words[1], &split_words[2]).await}
            "day" => {get_is_day(&grid_x, &grid_y, &grid_id).await;}
            "exit" => {prompt =false}
            &_ => {}
        }
    }
}

async fn get_location(lat : &str, lon : &str) -> (String, String, String){
    let client = Client::new();
    let grid_string = client.get("https://api.weather.gov/points/".to_owned()+lat+"," + lon).header("User-Agent", "request").send().await.expect("panic").text().await.unwrap();
    let grid = (&json::parse(&*grid_string).unwrap()["properties"]["gridX"], &json::parse(&*grid_string).unwrap()["properties"]["gridY"], &json::parse(&*grid_string).unwrap()["properties"]["gridId"]);
    println!("grid X : {}", grid.0);
    println!("grid Y : {}", grid.1);
    println!("grid id : {}", grid.2);
    (grid.0.to_string(), grid.1.to_string(), grid.2.to_string())
}

async fn get_is_day(grid_x:&str, grid_y:&str, grid_id:&str){
    let client = Client::new();
    let response = client.get("https://api.weather.gov/gridpoints/".to_owned()+grid_id+"/"+grid_x+","+grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
    let response_json = json::parse(&*response).unwrap();
    println!("{}", response_json["properties"]["periods"][0]["isDaytime"]);
}