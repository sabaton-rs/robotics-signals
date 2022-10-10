use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::Header;

/// Simple temperature reading
#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Temperature {
    // timestamp of the measurement
    // frame_id is the location of the temperature sensor
    pub header: Header,

    /// Measurement of the Temperature in Degrees Celsius.
    pub temperature: f64,

    /// 0 is interpreted as variance unknown
    pub variance: f64,
}
