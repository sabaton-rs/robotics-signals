use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::{geometry::Vector3, standard::Header};

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct MagneticField {
    /// timestamp is the time the
    /// field was measured
    /// frame_id is the location and orientation
    /// of the field measurement
    pub header: Header,

    /// x, y, and z components of the
    /// field vector in Tesla
    /// If your sensor does not output 3 axes,
    /// put NaNs in the components not reported.    
    pub magnetic_field: Vector3,

    /// Row major about x, y, z axes
    /// 0 is interpreted as variance unknown
    pub magnetic_field_covariance: [f64; 9],
}
