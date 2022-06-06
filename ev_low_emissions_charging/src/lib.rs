
use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

pub mod charging_scheduler;

pub mod spot_price;

pub mod solar_yield;

pub mod battery;

pub const VEHICLE_SOLAR_YILED_PRED_PATH : &'static str = "src\\vehicle_solar_yield_prediction.json";

pub const SPOT_PRICE_PRED_PATH :  &str = "src\\spot_price_prediction.json";

pub const CHARGING_TIME_PRECISION_IN_MINUTES : u32 = 60;

pub const DEBUG_ENABLE : bool = false;

pub const VEHICLE_BATTERY_CURRENT_STATE : u8 = 32;

pub const VEHICLE_BATTERY_DESIRED_CHARGE :u8 = 80;

pub const VEHICLE_BATTERY_CAPACITY_KWH:u32 = 60;

pub const VEHICLE_BATTERY_PHASE_COUNT:u16 = 3;

pub const VEHICLE_BATTERY_VOLT:u16 = 230;

pub const VEHICLE_BATTERY_AMPER:u16 = 16;

pub const VEHICLE_BATTERY_CHARGING_EFFICIENCY:f32 = 0.9;



use reqwest; // 0.9.10

use serde_json::Value; // 1.0.38


use std::error::Error;


pub trait DataLoader
{
    //fn load(&mut self,file_name:String);
    fn load(&mut self,file_name:String);
}

