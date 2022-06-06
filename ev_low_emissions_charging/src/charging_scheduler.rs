
use std::ops::Add;

use chrono::{DateTime, Duration, Utc};


use crate::{spot_price::SpotPrices};

use crate::{solar_yield::SolarYields};

use crate::{battery::Battery};

pub const MINUTES_IN_HOUR:u8 = 60;

fn get_best_time_for_charging_include_solar_power(vehicleBattery:Battery,currDate:DateTime<Utc>, leaveDate:DateTime<Utc>, desiredStateOfCharge:u8, spotPrices:SpotPrices, solarYields:SolarYields,estimatedTravelDurationInHours : f32)-> DateTime<Utc>
{

    let foundData : DateTime<Utc> = Utc::now();
    // get_best_to_time_start_charging

    return foundData;
}


pub fn get_best_to_time_start_charging(vehicleBattery:&Battery,currDate:DateTime<Utc>, travelDate:DateTime<Utc>, desiredStateOfCharge:u8, spotPrices:&SpotPrices) -> DateTime<Utc>
{

    let mut foundData : DateTime<Utc> = Utc::now();
    // sort the spot prices accroding to date.
    // The loaded data may not be sorted
    // Open this function call if the data is not sorted
    //spotPrices.prices.sort_by(|a,b| b.from.cmp(&a.from));

    // Calculate needed energy

    // If solar data enabled extract solar data

    // Find best time to charge
    if vehicleBattery.get_current_state() >= desiredStateOfCharge
    {
        return travelDate; 
    }

    let mut neededBatteryPercantage:u8 = desiredStateOfCharge - vehicleBattery.get_current_state() ;

    let mut vehicleChargingCapacity_kWh = ((vehicleBattery.get_phase_count() * vehicleBattery.get_volt() * vehicleBattery.get_amper())) as f32 / 1000.0_f32;

    // calculate needed KWH by including chargin efficiecy parameter
    let mut neededCapacitykWhWithEfficiencyParameter : f32 = (( vehicleBattery.capacity as f32 / 100.0_f32 )  * neededBatteryPercantage as f32) /  vehicleBattery.get_efficiency();

    // return date
    return get_best_charging_time_for_ground(&spotPrices,currDate,travelDate,neededCapacitykWhWithEfficiencyParameter,vehicleChargingCapacity_kWh);
    
}


fn get_best_charging_time_for_ground(spotPrices:&SpotPrices, startDate:DateTime<Utc>, travelDate:DateTime<Utc>,neededCapacity_kWh:f32,charginCapacity_Kwh:f32) -> DateTime<Utc>{

    let mut bestTime = startDate.clone();
    let mut currTime = startDate.clone();
    let mut bestPrice = f32::MAX;

    let totalChargingDurationInMinutes = ((neededCapacity_kWh / charginCapacity_Kwh) * MINUTES_IN_HOUR as f32) as i64;
    let mut endTime = currTime + Duration::minutes(totalChargingDurationInMinutes);

    // Find best total price

    // check best price according to precision
    while endTime < travelDate || currTime >= spotPrices.prices[spotPrices.prices.len()-1].till
    {
        let proposedPrice = calculate_market_price_of_a_time_period(&spotPrices,currTime,totalChargingDurationInMinutes);
        // check if duration not exceeds travelTİme
        if bestPrice > proposedPrice
        {
            bestPrice = proposedPrice;
            bestTime = currTime.clone();
        }

        currTime = currTime + Duration::minutes(1);
        endTime = endTime + Duration::minutes(1);
    }

    return bestTime;

}

/*
*
*/
fn calculate_market_price_of_a_time_period(spotPrices:&SpotPrices,startDate:DateTime<Utc>,durationInMinutes:i64) -> f32{
    let mut marketPriceInMin = 0.0;
    let mut endTime = startDate + Duration::minutes(durationInMinutes);
    let mut currTime = startDate.clone();

    if endTime > spotPrices.prices[spotPrices.prices.len() - 1].till{
        return -1.0 as f32;
    }
        

    for i in 0..durationInMinutes{

        // One minute market price
        marketPriceInMin += (get_market_price_per_hour(&spotPrices,currTime) / MINUTES_IN_HOUR as f32); 

        // increment minute by one
        currTime = currTime + Duration::minutes(1);
 
    }

    return marketPriceInMin;
}

/*
*
*
*/
fn get_market_price_per_hour(spotPrices:&SpotPrices,given_date:DateTime<Utc>) -> f32{

    let price = -1.0;

    for i in 0..spotPrices.prices.len()
    {
        if given_date >= spotPrices.prices[i].from && given_date < spotPrices.prices[i].till
        {
            return spotPrices.prices[i].marketPrice;
        }

    }

    return price;
}

/*
fn get_sorted_spot_prices(sp:SpotPrices) -> &'a SpotPrices
{

    let sorted_sp : SpotPrices = SpotPrices::new();

    for i in 0..sp.prices.len()
    {

        print!("{}",i);
    }

    sorted_sp

}
*/

