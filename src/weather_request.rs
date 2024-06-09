use reqwest::Client;

pub struct WeatherRequest {
    pub(crate) grid_x: String,
    pub(crate) grid_y: String,
    pub(crate) grid_id: String,
}

impl WeatherRequest{
    pub(crate) async fn update_location(&mut self, lat : &str, lon : &str) -> bool{
        let client = Client::new();
        let grid_string = client.get("https://api.weather.gov/points/".to_owned()+lat+"," + lon).header("User-Agent", "request").send().await.expect("panic").text().await.unwrap();
        let grid = (&json::parse(&*grid_string).unwrap()["properties"]["gridX"], &json::parse(&*grid_string).unwrap()["properties"]["gridY"], &json::parse(&*grid_string).unwrap()["properties"]["gridId"]);
        println!("grid X : {}", grid.0);
        println!("grid Y : {}", grid.1);
        println!("grid id : {}", grid.2);
        self.grid_x = grid.0.to_string();
        self.grid_y = grid.1.to_string();
        self.grid_id = grid.2.to_string();
        true
    }



    pub(crate) async fn get_is_day(&self){
        let client = Client::new();
        let response = client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        println!("{}", response_json["properties"]["periods"][0]["isDaytime"]);
    }

    pub(crate) async fn get_temperature(&self) -> String{
        let client = Client::new();
        let response = client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        //println!("{}", response_json["properties"]["periods"][0]["temperature"]);
        response_json["properties"]["periods"][0]["temperature"].to_string()
    }

    pub(crate) async fn get_json(&self){
        let client = Client::new();
        let response = client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        println!("{}", response_json["properties"]["periods"]);
    }
}