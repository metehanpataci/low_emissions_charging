use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};
use std::cmp::Ordering;

use crate::{DataLoader};

use crate::{DEBUG_ENABLE};

#[derive(Serialize, Deserialize, Debug)] //Eq
pub struct SpotPrice
{
    pub till: DateTime<Utc>,

    pub from: DateTime<Utc>,

    pub marketPrice: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotPrices
{
    pub prices : Vec<SpotPrice>,
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
/*
impl Ord for SpotPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}

impl PartialOrd for SpotPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SpotPrice {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
    }
}
*/
impl SpotPrices
{

    pub fn new() -> Self {
        Self {
            prices: Vec::new(),
        }
    }

  
}


impl DataLoader for SpotPrices
{
    /*
    * Definition:
    * Parameters:
    * Return:
    */
    fn load(&mut self,file_name:String){

        let mut file = File::open(file_name.to_string()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        if DEBUG_ENABLE == true {
         println!("{}",buff);
        }
      
        self.prices = serde_json::from_str(&buff).unwrap();

    }
}

/*
* Definition:
* Parameters:
* Return:
*/
pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}