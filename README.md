# EV LOW EMISSION CHARGING

## PURPOSE
A function in Rust to return the best time to start charging. 

## Code
All the configarions are in the lib.rs file. Configurative things defines as const variabled.

The prgram calculates best time for grid charging in minutes precision. Start time of charging can be any minutes of available times and carging continues without interruption.

If user makes **IS_SOLAR_CHARGE_ESTIMATION_ENABLED** variable **true** at **lib.rs** file program calculates charging of car during travel according to **vehicle_solar_yield_prediction.json** file. Solar power contribution is extracted from **VEHICLE_BATTERY_DESIRED_CHARGE** (which is at lib.rs file) during program run time then grid charging was calculated.

## Data

All data was stored as json file under the same directory (src) of code files.
- spot_price_prediction.json
This file stores spot prices.
- vehicle_solar_yield_prediction.json
This file stores solar yield prediction data.

## Configuration

Most of the configuration parameters are at **lib.rs** file.

***lib.rs***
```
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
```
## FUNCTIONS

**get_best_to_time_start_for_grid_charging**

Calculates best charging time without taking account of solar power during travel.

**get_best_time_for_charging_include_solar_power**

Calculates best charging time with taking account of solar power during travel.

## EXECUTION

**main.rs** is starting point of program. Run main.rs or executable file under Debug folder.
