/*
 * @Author: metehanpataci metehanpataci@users.noreply.github.com
 * @Date: 2022-06-06 12:45:46
 * @LastEditors: metehanpataci metehanpataci@users.noreply.github.com
 * @LastEditTime: 2022-06-07 12:31:59
 * @FilePath: \ev_low_emissions_charging\src\test\solar_yield_test.rs
 */


#[test]
fn read_solar_yield_data_test()
{
    let sy : SolarYields = SolarYields::new();

    assert_eq!( sy.load(VEHICLE_SOLAR_YILED_PRED_PATH),true);
}

#[test]
fn read_solar_gird_data_test()
{
    let sp : SpotPrices = SpotPrices::new();

    assert_eq!( sp.load(SPOT_PRICE_PRED_PATH),true);
}



