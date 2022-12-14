use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::{
    geometry::{Transform, Twist, Wrench},
    standard::Header,
};

/// Representation of state for joints with multiple degrees of freedom,
/// following the structure of JointState which can only represent a single degree of freedom.
///
/// It is assumed that a joint in a system corresponds to a transform that gets applied
/// along the kinematic chain. For example, a planar joint (as in URDF) is 3DOF (x, y, yaw)
/// and those 3DOF can be expressed as a transformation matrix, and that transformation
/// matrix can be converted back to (x, y, yaw)
///
/// Each joint is uniquely identified by its name
/// The header specifies the time at which the joint states were recorded. All the joint states
/// in one message have to be recorded at the same time.
///
/// This message consists of a multiple arrays, one for each part of the joint state.
/// The goal is to make each of the fields optional. When e.g. your joints have no
/// wrench associated with them, you can leave the wrench array empty.
///
/// All arrays in this message should have the same size, or be empty.
/// This is the only way to uniquely associate the joint name with the correct
/// states.

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct MultiDOFJointState {
    pub header: Header,
    pub joint_names: Vec<String>,
    pub transforms: Vec<Transform>,
    pub twist: Vec<Twist>,
    pub wrench: Vec<Wrench>,
}
