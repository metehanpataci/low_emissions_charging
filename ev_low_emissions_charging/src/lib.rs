
use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

pub mod charging_scheduler;

pub mod spot_price;

pub mod solar_yield;

pub const VEHICLE_SOLAR_YILED_PRED_PATH : &'static str = "src\\vehicle_solar_yield_prediction.json";

pub const SPOT_PRICE_PRED_PATH :  &str = "src\\spot_price_prediction.json";

pub const DEBUG_ENABLE : bool = false;

use reqwest; // 0.9.10
use serde_json::Value; // 1.0.38


use std::error::Error;


pub trait DataLoader
{
    //fn load(&mut self,file_name:String);
    fn load(&mut self,file_name:String);
}

