
use std::ops::Add;

use chrono::{DateTime, Duration, Utc};


use crate::{spot_price::SpotPrices};

use crate::{solar_yield::SolarYields};

use crate::{battery::Battery};

pub const MINUTES_IN_HOUR:u8 = 60;

/*
* Definition: Calculate solar data battery contribution after leave than calculate charging. 
* Parameters:
* Return:
*/
pub fn get_best_time_for_charging_include_solar_power(vehicleBattery:&Battery,currDate:DateTime<Utc>, leaveDate:DateTime<Utc>, desiredStateOfCharge:u8, spotPrices:&SpotPrices, solarYields:&SolarYields,estimatedTravelDurationInMinues : i32)-> ((DateTime<Utc>,f32,i64),f32)
{
    let foundData : DateTime<Utc> = Utc::now();
    let mut startSolarChargeTime = leaveDate.clone();
    let mut currDesiredStateOfCharge = desiredStateOfCharge;


    // calculate solar charging during travel
    let estimatedSolarYield_kwh = calculate_solar_charging_during_travel_in_kwh(vehicleBattery,&solarYields,estimatedTravelDurationInMinues,leaveDate);

    // Update desired battey charge level. Because we will charge during travel
    let batteryPercentageContributionOfSolarCharging = (100.0 * estimatedSolarYield_kwh) / vehicleBattery.capacity as f32;

    if ((currDesiredStateOfCharge as f32) - batteryPercentageContributionOfSolarCharging) <= vehicleBattery.get_current_state() as f32{
        // I do not think low level charge status
        // If charging level is low we can add some grid charging
        currDesiredStateOfCharge = vehicleBattery.get_current_state();

    }else{

        currDesiredStateOfCharge = currDesiredStateOfCharge - batteryPercentageContributionOfSolarCharging as u8;
    }

    return (get_best_to_time_start_for_grid_charging(vehicleBattery,currDate,leaveDate, desiredStateOfCharge,&spotPrices),batteryPercentageContributionOfSolarCharging);
}

/*
* Definition: Return estimated solar charge inncluding charging efficiency
* Parameters:
* Return: price
*/
fn calculate_solar_charging_during_travel_in_kwh(vehicleBattery:&Battery,solarYields:&SolarYields,estimatedTravelDurationInMinues : i32,solarChargeStartDate:DateTime<Utc>) -> f32
{
  
    let mut totalSolarYieldInMinute_kwh:f32 = 0_f32;
    let mut currTime = solarChargeStartDate.clone();
/*
    if endTime > spotPrices.prices[spotPrices.prices.len() - 1].till{
        return -1.0 as f32;
    }
*/       
    for i in 0..estimatedTravelDurationInMinues{

        // One minute market price
        totalSolarYieldInMinute_kwh += (get_solar_charging_amount_according_to_exact_time(&solarYields,solarChargeStartDate) / MINUTES_IN_HOUR as f32); 

        // increment minute by one
        currTime = currTime + Duration::minutes(1);
 
    }

    // Add efficiency to calculation
    return totalSolarYieldInMinute_kwh * vehicleBattery.efficiency;
}

/*
* Definition: 
* Parameters:
* Return: price
*/
fn get_solar_charging_amount_according_to_exact_time(solarYields:&SolarYields,givenDate:DateTime<Utc>) -> f32{

    let solarYield = 0_f32;

    for i in 0..solarYields.yields.len()
    {
        if givenDate >= solarYields.yields[i].from && givenDate < solarYields.yields[i].till
        {
            return solarYields.yields[i].solarYieldWattHour as f32 / 1000.0 as f32;
        }

    }

    return solarYield;
}

/*
* Definition: Calculate best time to start charging and return
* Parameters:
*   vehicleBattery : Vehicle battery properties and status
*   currDate : Earliest time to start charging
*   travelDate : Car movement date. This is the last time for charging
*   desiredStateOfCharge : Desired chargin percentage
*   spotPrices : spot prices list
* Return: Best time start for charging
*/
pub fn get_best_to_time_start_for_grid_charging(vehicleBattery:&Battery,currDate:DateTime<Utc>, travelDate:DateTime<Utc>, desiredStateOfCharge:u8, spotPrices:&SpotPrices) -> (DateTime<Utc>,f32,i64)
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
        return (travelDate,0_f32,0); 
    }

    let mut neededBatteryPercantage:u8 = desiredStateOfCharge - vehicleBattery.get_current_state() ;

    let mut vehicleChargingCapacity_kWh = ((vehicleBattery.get_phase_count() * vehicleBattery.get_volt() * vehicleBattery.get_amper())) as f32 / 1000.0_f32;

    // calculate needed KWH by including chargin efficiecy parameter
    let mut neededCapacitykWhWithEfficiencyParameter : f32 = (( vehicleBattery.capacity as f32 / 100.0_f32 )  * neededBatteryPercantage as f32) /  vehicleBattery.get_efficiency();

    // return date
    return get_best_charging_time_for_ground(&spotPrices,currDate,travelDate,neededCapacitykWhWithEfficiencyParameter,vehicleChargingCapacity_kWh);
    
}

/*
* Definition: Helper function to calculate best continious charging start time
* Parameters:
*   spotPrices: spot price data list
*   startDate : earliest time to start charging
*   travelDate : the car movement time. this is the end of charging time
*   neededCapacity_kwh : needed kwh to reach desired charging level
*   charginCapacity_Kwh : 
* Return:
*/
fn get_best_charging_time_for_ground(spotPrices:&SpotPrices, startDate:DateTime<Utc>, travelDate:DateTime<Utc>,neededCapacity_kWh:f32,charginCapacityOfVehicle_Kwh:f32) -> (DateTime<Utc>,f32,i64){

    let mut bestTime = startDate.clone();
    let mut currTime = startDate.clone();
    let mut bestPrice = f32::MAX;

    let totalChargingDurationInMinutes = ((neededCapacity_kWh / charginCapacityOfVehicle_Kwh) * MINUTES_IN_HOUR as f32) as i64;
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

    return (bestTime,bestPrice,totalChargingDurationInMinutes);

}

/*
* Definition: This function calculates price time period in minutes precision
* Parameters:
*   spotPrices: Spor prices data
*   startDate : Start date of charging
*   durationInMinutes : Chargin duration in minutes
* Return : price of charging according to given data
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
        marketPriceInMin += (get_market_price_for_exact_date(&spotPrices,currTime) / MINUTES_IN_HOUR as f32); 

        // increment minute by one
        currTime = currTime + Duration::minutes(1);
 
    }

    return marketPriceInMin;
}

/*
* Definition: return market price of electricity according to given date time. Function find exact to range of of given time
* Parameters:
*   spotPrices : spotPrice list data
*   givenDate : Exact time to get price
* Return: price
*/
fn get_market_price_for_exact_date(spotPrices:&SpotPrices,givenDate:DateTime<Utc>) -> f32{

    let price = -1.0;

    for i in 0..spotPrices.prices.len()
    {
        if givenDate >= spotPrices.prices[i].from && givenDate < spotPrices.prices[i].till
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

