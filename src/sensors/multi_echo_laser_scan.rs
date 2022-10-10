use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    geometry::{Transform, Twist, Wrench},
    standard::Header,
};

/// Single scan from a multi-echo planar laser range-finder
///
/// If you have another ranging device with different behavior (e.g. a sonar
/// array), please find or create a different message, since applications
/// will make fairly laser-specific assumptions about this data

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct MultiEchoLaserScan {
    /// timestamp in the header is the acquisition time of
    /// the first ray in the scan.
    ///`
    /// in frame frame_id, angles are measured around
    /// the positive Z axis (counterclockwise, if Z is up)
    /// with zero angle being forward along the x axis
    pub header: Header,

    /// start angle of the scan in radians
    pub angle_min: f32,

    /// end angle of the scan in radians
    pub angle_max: f32,

    /// angular distance between measurements
    pub angle_increment: f32,

    /// time between scans in seconds
    /// if your scanner is moving, this will be used in interpolating position
    /// of 3d points
    pub time_increments: f32,

    /// time between scans in seconds
    pub scan_time: f32,

    /// minimum range value in metres
    pub range_min: f32,

    /// maximum range valie in metres
    pub range_max: f32,

    /// range data in metres
    pub ranges: Vec<Vec<f32>>,

    /// intensity data
    pub intensities: Vec<Vec<f32>>,
}
