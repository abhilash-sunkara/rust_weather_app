use reqwest::Client;

pub struct WeatherRequest {
    grid_x: String,
    grid_y: String,
    grid_id: String,
    client: Client,
    time_period : usize,
}

impl WeatherRequest{

    pub fn new() -> Self{
        Self {
            grid_x : "".parse().unwrap(),
            grid_y : "".parse().unwrap(),
            grid_id : "".parse().unwrap(),
            client : Client::new(),
            time_period : 0
        }
    }

    pub async fn update_location(&mut self, lat : &str, lon : &str){
        let grid_string = self.client.get("https://api.weather.gov/points/".to_owned()+lat+"," + lon).header("User-Agent", "request").send().await.expect("panic").text().await.unwrap();
        let grid = (&json::parse(&*grid_string).unwrap()["properties"]["gridX"], &json::parse(&*grid_string).unwrap()["properties"]["gridY"], &json::parse(&*grid_string).unwrap()["properties"]["gridId"]);
        self.grid_x = grid.0.to_string();
        self.grid_y = grid.1.to_string();
        self.grid_id = grid.2.to_string();

    }

    pub async fn get_location(&self) {
        println!("grid X : {}", self.grid_x);
        println!("grid Y : {}", self.grid_y);
        println!("grid id : {}", self.grid_id);
    }

    pub fn get_grid_x(&self) -> &str {
        self.grid_x.as_str()
    }

    pub fn get_grid_y(&self) -> &str {
        self.grid_y.as_str()
    }

    pub fn get_grid_id(&self) -> &str {
        self.grid_id.as_str()
    }


    pub async fn get_is_day(&self){
        let response = self.client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        println!("{}", response);
        println!("{}", response_json["properties"]["periods"][self.time_period]["isDaytime"]);
    }

    pub async fn get_temperature(&self) -> String{
        let temp_time_period : usize = self.time_period.clone();
        let response = self.client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let mut response_json = json::parse(&*response).unwrap();
        //println!("{}", response_json["properties"]["periods"][0]["temperature"]);
        //println!("{}", response_json["properties"]["periods"][self.time_period]["temperature"].to_string());
        return response_json["properties"]["periods"][temp_time_period]["temperature"].to_string()
    }



    pub async fn get_json(&self){
        let response = self.client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        //println!("{}", response_json["properties"]["periods"]);
    }

    pub async fn get_short_forecast(&self) -> String{
        let temp_time_period : usize = self.time_period.clone();
        let response = self.client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let response_json = json::parse(&*response).unwrap();
        //println!("{}", response_json["properties"]["periods"][self.time_period]["shortForecast"]);
        return response_json["properties"]["periods"][temp_time_period]["shortForecast"].to_string()
    }

    pub async fn get_detailed_forecast(&self) -> String{
        let temp_time_period : usize = self.time_period.clone();
        let response = self.client.get("https://api.weather.gov/gridpoints/".to_owned()+ &*self.grid_id +"/"+&*self.grid_x+","+&*self.grid_y+"/forecast").header("User-Agent", "reqwest").send().await.unwrap().text().await.unwrap();
        let mut response_json = json::parse(&*response).unwrap();
        //println!("{}", response_json["properties"]["periods"][self.time_period]["detailedForecast"]);
        return response_json["properties"]["periods"][temp_time_period]["detailedForecast"].to_string()
    }

    pub fn increment_time_period(&mut self){
        if self.time_period < 13 {
            self.time_period += 1;
            //println!("Time period is now at {}", self.time_period);
        } else {
            //println!("Time period unable to be incremented");
        }

    }

    pub fn decrement_time_period(&mut self){
        if self.time_period > 0 {
            self.time_period -= 1;
            //println!("Time period is now at {}", self.time_period);
        } else {
            //println!("Time period unable to be decremented");
        }
    }
}