/*
 * @Author: metehanpataci metehanpataci@users.noreply.github.com
 * @Date: 2022-06-05 22:25:43
 * @LastEditors: metehanpataci metehanpataci@users.noreply.github.com
 * @LastEditTime: 2022-06-07 01:12:08
 * @FilePath: \ev_low_emissions_charging\src\solar_yield.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::fs::File;
use std::io::Read;
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

use crate::{DataLoader,DEBUG_ENABLE};


#[derive(Serialize, Deserialize, Debug)]
pub struct SolarYield
{
    pub till: DateTime<Utc>,

    pub from: DateTime<Utc>,

    pub solarYieldWattHour:u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SolarYields
{
    pub yields : Vec<SolarYield>,
}

impl SolarYields
{
    pub fn new() -> Self {
        Self {
            yields: Vec::new(),
        }
    }
}

impl DataLoader for SolarYields
{
    fn load(&mut self,file_name:String){

        let mut file = File::open(file_name.to_string()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        if DEBUG_ENABLE == true {
            println!("{}",buff);
        }
      
        self.yields = serde_json::from_str(&buff).unwrap();
    }
}