
use ev_low_emissions_charging::{
    spot_price::SpotPrices,
    solar_yield::SolarYields,
    
};

extern crate ev_low_emissions_charging;
use ev_low_emissions_charging::VEHICLE_SOLAR_YILED_PRED_PATH;
use ev_low_emissions_charging::SPOT_PRICE_PRED_PATH;
use ev_low_emissions_charging::DataLoader;

fn main() {
    let mut sp = SpotPrices::new();
    sp.load(SPOT_PRICE_PRED_PATH.to_string());

    let mut sy:SolarYields = SolarYields::new();
    sy.load(VEHICLE_SOLAR_YILED_PRED_PATH.to_string());

}
