/*
 * @Author: metehanpataci metehanpataci@users.noreply.github.com
 * @Date: 2022-06-05 16:35:31
 * @LastEditors: metehanpataci metehanpataci@users.noreply.github.com
 * @LastEditTime: 2022-06-07 02:16:18
 * @FilePath: \ev_low_emissions_charging\src\lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

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

pub const ESTIMATED_TRAVEL_DURATION_IN_MINUTES:i32 = 90;

pub const IS_SOLAR_CHARGE_ESTIMATION_ENABLED : bool = false;

use reqwest; // 0.9.10

use serde_json::Value; // 1.0.38


use std::error::Error;


pub trait DataLoader
{
    //fn load(&mut self,file_name:String);
    fn load(&mut self,file_name:String);
}

