use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

use crate::{DataLoader,DEBUG_ENABLE};


#[derive(Serialize, Deserialize, Debug)]
pub struct SolarYield
{
    till: DateTime<Utc>,

    from: DateTime<Utc>,

    solarYieldWattHour:u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SolarYields
{
    yields : Vec<SolarYield>,
}

impl SolarYields
{
    pub fn new() -> Self {
        Self {
            yields: Vec::new(),
        }
    }
}

impl DataLoader for SolarYields
{
    fn load(&mut self,file_name:String){

        let mut file = File::open(file_name.to_string()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        if DEBUG_ENABLE == true {
            println!("{}",buff);
        }
      
        self.yields = serde_json::from_str(&buff).unwrap();

        println!("Solar yields data was loaded..");
        //println!("Name: {}", foo.name);

    }
}