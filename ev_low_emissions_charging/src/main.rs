/*
 * @Author: metehanpataci metehanpataci@users.noreply.github.com
 * @Date: 2022-06-05 16:23:05
 * @LastEditors: metehanpataci metehanpataci@users.noreply.github.com
 * @LastEditTime: 2022-06-07 02:11:37
 * @FilePath: \ev_low_emissions_charging\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use ev_low_emissions_charging::battery::Battery;
use ev_low_emissions_charging::{
    spot_price::SpotPrices,
    solar_yield::SolarYields,
    charging_scheduler::get_best_to_time_start_for_grid_charging,
    charging_scheduler::get_best_time_for_charging_include_solar_power,
};

use chrono::{DateTime, Duration, Utc};

extern crate ev_low_emissions_charging;

use ev_low_emissions_charging::{
    VEHICLE_SOLAR_YILED_PRED_PATH, 
    VEHICLE_BATTERY_PHASE_COUNT, 
    VEHICLE_BATTERY_VOLT, 
    VEHICLE_BATTERY_CHARGING_EFFICIENCY,
    VEHICLE_BATTERY_CURRENT_STATE,
    VEHICLE_BATTERY_CAPACITY_KWH,
    VEHICLE_BATTERY_AMPER,
    VEHICLE_BATTERY_DESIRED_CHARGE,
    ESTIMATED_TRAVEL_DURATION_IN_MINUTES,
    IS_SOLAR_CHARGE_ESTIMATION_ENABLED,
};
use ev_low_emissions_charging::SPOT_PRICE_PRED_PATH;
use ev_low_emissions_charging::DataLoader;

/*
* Definition:
* Parameters:
* Return:
*/
fn main() {

    let mut battery = Battery{

        current_state : VEHICLE_BATTERY_CURRENT_STATE,

        capacity : VEHICLE_BATTERY_CAPACITY_KWH, // Battery Capacity of kWh

        phase_count : VEHICLE_BATTERY_PHASE_COUNT, // Phase count
    
        amper : VEHICLE_BATTERY_AMPER, // Amper value
    
        volt : VEHICLE_BATTERY_VOLT, // Volt
    
        efficiency : VEHICLE_BATTERY_CHARGING_EFFICIENCY, // Charging efficiency. Meaning that each kWh from the grid delivers. ex: 90% means 0.9kWh in the battery
    };

    let mut sp = SpotPrices::new();
    sp.load(SPOT_PRICE_PRED_PATH.to_string());

   
    let mut sy:SolarYields = SolarYields::new();
    sy.load(VEHICLE_SOLAR_YILED_PRED_PATH.to_string());


    // Set current time to first prediction data time
    let currDate = sp.prices[0].from.clone();

    let travelDate = DateTime::parse_from_rfc3339("2022-04-07T13:00:00.000Z").unwrap().with_timezone(&Utc);

    if IS_SOLAR_CHARGE_ESTIMATION_ENABLED == false{ 
        let mut res = get_best_to_time_start_for_grid_charging(&battery,currDate,travelDate,VEHICLE_BATTERY_DESIRED_CHARGE,&sp);
        //sp.prices.sort_by(|a,b| b.from.cmp(&a.from));
        println!("");
        println!("ONLY GRID CHARGING ESTIMATION");
        //println!("Earliest Charge start Date is {} Travel Date is: {}",currDate,travelDate);
        println!("Checking Best Time to charge...");
        println!("# # # # Report: # # # #");
        println!("Best Time To Charge : {}\nPrice : {} \nDuration (minutes) : {} \n", res.0,res.1,res.2);
    }else{
        let (grid_result,solar_result) = get_best_time_for_charging_include_solar_power(&battery,currDate,travelDate,VEHICLE_BATTERY_DESIRED_CHARGE,&sp,&sy,ESTIMATED_TRAVEL_DURATION_IN_MINUTES);

        println!("");
        println!("GRID AND SOLAR CHARGING ESTIMATION");
        //println!("Earliest Charge start Date is {} Travel Date is: {}",currDate,travelDate);
        println!("Checking Best Time to charge...");
        
        println!("# # # # Report: # # # #");
        println!("Best Time To Charge : {}\nPrice : {} \nDuration (minutes) : {} \nSolar Charge Contribution kwh {}\n", grid_result.0,grid_result.1,grid_result.2,solar_result);
    }

}
