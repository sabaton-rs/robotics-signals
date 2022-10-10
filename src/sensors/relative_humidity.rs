use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    standard::Header,
};

/// Single reading from a relative humidity sensor.
/// Defines the ratio of partial pressure of water vapor to the saturated vapor
/// pressure at a temperature.

#[repr(C)]
#[derive(Serialize, Deserialize,Topic)]
pub struct RelativeHumdity {
    // timestamp of the measurement
    // frame_id is the location of the humidity sensor
    pub header : Header,

    /// Expression of the relative humidity
    /// from 0.0 to 1.0.
    /// 0.0 is no partial pressure of water vapor
    /// 1.0 represents partial pressure of saturation
    pub relative_humidity : f64,

    /// 0 is interpreted as variance unknown
    pub variance : f64,
}