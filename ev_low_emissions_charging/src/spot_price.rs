
use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

use crate::{DataLoader};

use crate::{DEBUG_ENABLE};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotPrice
{
    till: DateTime<Utc>,

    from: DateTime<Utc>,

    marketPrice:f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotPrices
{
    prices : Vec<SpotPrice>,
}

impl SpotPrice{
    pub fn new() -> Self
    {
        Self{
            till : chrono::offset::Utc::now(),
            from : chrono::offset::Utc::now(), 
            marketPrice : 0.0,
        }
    }
}


impl SpotPrices
{

    pub fn new() -> Self {
        Self {
            prices: Vec::new(),
        }
    }

/*
    pub fn load(&mut self,file_name:String)
    {

        let mut file = File::open(file_name.to_string()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        println!("{}",buff);
      
        self.prices = serde_json::from_str(&buff).unwrap();

        println!("Buffer deserialized to struct");
        //println!("Name: {}", foo.name);

    }
*/
  
}


impl DataLoader for SpotPrices
{

    fn load(&mut self,file_name:String){

        let mut file = File::open(file_name.to_string()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        if DEBUG_ENABLE == true {
         println!("{}",buff);
        }
      
        self.prices = serde_json::from_str(&buff).unwrap();

        println!("Spot prices data was loaded..");
        //println!("Name: {}", foo.name);

    }
}


pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}