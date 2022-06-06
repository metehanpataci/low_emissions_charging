# EV LOW EMISSION CHARGING

## PURPOSE
Write a function in Rust to return the best time to start charging. To do so first calculate the amount of time needed to charge up to the desired level taking the following into account:
- Current state of charge 32%
- Desired state of charge 80%
- Battery capacity of 60kWh
- 3-phase charger, 16A on each phase at 230V
- Charging efficiency of 90% (meaning that each kWh from the grid delivers 0.9kWh in the battery)
- Preferred to charge in a single time block without interruptions for simplicity 
Once we know home much time is needed we need to find the optimal continuous time block to charge taking into account predictions for the electricity spot prices. You can find the predictions in the appendix.

Return the best starting time for charging taking the following into account

- She wants to leave at 13:00 on 7 April
- The location is Zurich, Switzerland
- Current date is 6th of April
- The spot prices are shown in the appendix

While doing all of the above please make sure to
- Write code in Rust (rust-lang.org)
- Write tests
- Add a README.md file with explanation of what the repository does
- Zip the code repository and send it to Lightyear
- Tell us how long it took and whether Rust is new to you
- Don’t make the solution publicly available

If you love this assignment you can earn some extra bonus points by doing one or
both of the following

- Wrapping the function in a REST endpoint
- Taking the solar yield predictions of the vehicle itself into account, they can be
found in the appendix

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


