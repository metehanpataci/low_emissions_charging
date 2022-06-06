
use ev_low_emissions_charging::battery::Battery;
use ev_low_emissions_charging::{
    spot_price::SpotPrices,
    solar_yield::SolarYields,
    charging_scheduler::get_best_to_time_start_charging,
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
};
use ev_low_emissions_charging::SPOT_PRICE_PRED_PATH;
use ev_low_emissions_charging::DataLoader;

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

    println!("Output : {} {} {}",sp.prices[0].from, sp.prices[0].till, sp.prices[0].marketPrice );

    let mut sy:SolarYields = SolarYields::new();
    sy.load(VEHICLE_SOLAR_YILED_PRED_PATH.to_string());



    //let date_str = "2022-04-06T14:00:00.000Z";
    //let currDate = DateTime::parse_from_rfc3339(date_str);

    // Set current time to first prediction data time
    let currDate = sp.prices[0].from.clone();

    let travelDate = DateTime::parse_from_rfc3339("2022-04-07T13:00:00.000Z").unwrap().with_timezone(&Utc);

    println!("Earliest Charge start Date is {} Travel Date is: {}",currDate,travelDate);

    let mut bestTimeToCharge = get_best_to_time_start_charging(&battery,currDate,travelDate,VEHICLE_BATTERY_DESIRED_CHARGE,&sp);
    //sp.prices.sort_by(|a,b| b.from.cmp(&a.from));

    
    println!("Checking Best Time to charge...");
    println!("Best Time To Charge is {}", bestTimeToCharge);

}
