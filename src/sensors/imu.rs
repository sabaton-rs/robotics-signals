use crate::{
    geometry::{Quarternion, Vector3},
    standard::Header,
};
use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Imu {
    pub header: Header,

    pub orientation: Quarternion,
    /// Row major about x, y, z axes
    pub orientation_covariance: [f64; 9],

    pub angular_velocity: Vector3,
    ///  Row major about x, y, z axes
    pub angular_velocity_covariance: [f64; 9],

    pub linear_acceleration: Vector3,
    /// Row major x, y z
    pub linear_acceleration_covariance: [f64; 9],
}
