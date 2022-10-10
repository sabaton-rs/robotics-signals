use serde_derive::{Deserialize, Serialize};
use cdds_derive::*;
use cyclonedds_rs::*;

use crate::standard::Header;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum PowerSupplyStatus {
    Unknown=0,
    Charging=1,
    Discharging=2,
    NotCharging=3,
    Full=4,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum PowerSupplyHealth {
    Unknown = 0,
    Good = 1,
    Overheat = 2,
    Dead = 3,
    Overvoltage = 4,
    UnspecifiedFailure = 5,
    Cold = 6,
    WatchdogTimerExpired = 7,
    SafetyTimerExpired = 8,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum BatteryChemistry {
    Unknown  = 0,
    NiMh = 1,
    LIon = 2,
    LiPo = 3,
    LiFe = 4,
    NiCd = 5,
    LiMn = 6,
}

#[repr(C)]
#[derive(Serialize, Deserialize,Topic)]
pub struct BatteryState {
    pub header : Header,
    /// Voltage in Volts (Mandatory)
    pub voltage : f32,
    /// Temperature in Degrees Celsius (If unmeasured NaN)
    pub temperature : f32,
    /// Negative when discharging (A)  (If unmeasured NaN)
    pub current : f32,
    /// Current charge in Ah  (If unmeasured NaN)
    pub charge : f32,
    /// Capacity in Ah (last full capacity)  (If unmeasured NaN)
    pub capacity : f32,
    /// Capacity in Ah (design capacity)  (If unmeasured NaN)
    pub design_capacity : f32,
    ///  Charge percentage on 0 to 1 range  (If unmeasured NaN)
    pub percentage : f32,

    pub status : PowerSupplyStatus,
    pub health :PowerSupplyHealth,
    pub technology : BatteryChemistry,
    /// True if the battery is present
    pub present : bool,

    /// An array of individual cell voltages for each cell in the pack
    // If individual voltages unknown but number of cells known set each to NaN
    pub cell_voltage : Vec<f32>,
    /// # An array of individual cell temperatures for each cell in the pack
    /// If individual temperatures unknown but number of cells known set each to NaN
    pub cell_temperature : Vec<f32>,

    /// The location into which the battery is inserted. (slot number or plug)
    #[topic_key]
    pub location : String,
    /// The best approximation of the battery serial number
    pub serial_number : String,
}