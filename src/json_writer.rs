use std::process::exit;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use json::Error;

#[derive(Serialize)] #[derive(Deserialize)]
pub struct Location {
    name : String,
    latitude : String,
    longitude : String
}

impl Location {
    pub fn new(name : String,lat : String, lon : String) -> Self {
        Self{
            name,
            latitude: lat,
            longitude: lon,
        }
    }
}

#[derive(Serialize)] #[derive(Deserialize)]
pub struct LocationJSON {
    locations : Vec<Location>
}

impl LocationJSON {
    pub fn new() -> Result<LocationJSON, std::io::Error>{
        let file = match File::open("json.txt") {
            Ok(x) => {x}
            Err(_) => {return Ok(LocationJSON {
                locations: vec![],
            })}
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let new: LocationJSON = match serde_json::from_str(&*contents) {
            Ok(v) => v,
            Err(_) => {
                Self {
                    locations: vec![]
                }
            }
        };
        Ok(new)
    }

    fn load_from_json(&mut self) -> std::io::Result<()> {
        let file = File::open("json.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let new: LocationJSON = match serde_json::from_str(&*contents) {
            Ok(v) => v,
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data");
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        Ok(())

    }

    pub fn add_location(&mut self, name : &str, lat : &str, lon : &str){
        self.locations.push(Location::new(name.to_string(), lat.to_string(), lon.to_string()))
    }

    pub fn get_locations(&self) {
        for item in self.locations.iter() {
            println!("name is {}, lat is {}, lon is {}", item.name, item.latitude, item.longitude)
        }
    }

    pub fn contains_location(&self, name : &str) -> bool {
        for item in self.locations.iter() {
            if item.name == name.to_string().trim() {
                return true
            }
        }
        false
    }

    pub fn get_lat(&self, name : &str) -> &str {
        for item in self.locations.iter() {
            if item.name == name.to_string().trim() {
                return &item.latitude
            }
        }
        ""
    }

    pub fn get_lon(&self, name : &str) -> &str {
        for item in self.locations.iter() {
            if item.name == name.to_string().trim() {
                return &item.longitude
            }
        }
        ""
    }

    pub fn save_to_json(&self) -> std::io::Result<()> {
        let json = match serde_json::to_string(self){
            Ok(v) => v,
            Err(_) => {
                println!("unable to read");
                exit(1);
            }
        };
        let mut file = File::create("json.txt")?;
        file.write_all(json.as_ref())?;
        println!("{}", json);
        Ok(())
    }
}