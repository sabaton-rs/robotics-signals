use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    standard::Header,
};

/// Single range reading from an active ranger that emits energy and reports
/// one range reading that is valid along an arc at the distance measured.
/// This message is  not appropriate for laser scanners. See the LaserScan
/// message if you are working with a laser scanner.
///
/// This message also can represent a fixed-distance (binary) ranger.  This
/// sensor will have min_range===max_range===distance of detection.
/// These sensors follow REP 117 and will output -Inf if the object is detected
/// and +Inf if the object is outside of the detection range.

#[repr(C)]
#[derive(Serialize, Deserialize,Topic)]
pub struct Range {
    pub header : Header,

    pub radiation_type : RadiationType,

    /// the size of the arc that the distance reading is
    /// valid for [rad]
    /// the object causing the range reading may have
    /// been anywhere within -field_of_view/2 and
    /// field_of_view/2 at the measured range.
    /// 0 angle corresponds to the x-axis of the sensor.
    pub field_of_view : f32,

    /// minimum range value [m]
    pub min_range : f32,

    /// maximum range value [m]
    pub max_range : f32,
    /// range data [m]
    /// (Note: values < range_min or > range_max should be discarded)
    /// Fixed distance rangers only output -Inf or +Inf.
    /// -Inf represents a detection within fixed distance.
    /// (Detection too close to the sensor to quantify)
    /// +Inf represents no detection within the fixed distance. (Object out of range)
    pub range : f32,


}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum RadiationType {
    Ultrasound=0,
    Infrared=1,
}