use crate::standard::Header;
use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

/// Single photometric illuminance measurement.  Light should be assumed to be
/// measured along the sensor's x-axis (the area of detection is the y-z plane).
/// The illuminance should have a 0 or positive value and be received with
/// the sensor's +X axis pointing toward the light source.
///
/// Photometric illuminance is the measure of the human eye's sensitivity of the
/// intensity of light encountering or passing through a surface.
///
/// All other Photometric and Radiometric measurements should not use this message.
/// This message cannot represent:
///  - Luminous intensity (candela/light source output)
///  - Luminance (nits/light output per area)
///  - Irradiance (watt/area), etc.

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Illuminance {
    /// frame_id is the location and direction of the reading
    pub header: Header,
    /// Measurement of the Photometric Illuminance in Lux.
    pub illuminance: f64,
    ///  0 is interpreted as variance unknown
    pub variance: f64,
}
