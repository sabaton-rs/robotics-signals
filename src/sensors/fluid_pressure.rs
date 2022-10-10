use serde_derive::{Deserialize, Serialize};
use cdds_derive::*;
use cyclonedds_rs::*;
use crate::standard::Header;

#[repr(C)]
#[derive(Serialize, Deserialize,Topic)]
pub struct FluidPressure {
    /// frame_id is the location of the pressure sensor
    pub header : Header,
    /// Absolute pressure reading in Pascals.
    pub fluid_pressure : f64,
    /// 0 is interpreted as variance unknown
    pub variance : f64, 
}