use cdds_derive::Topic;
use serde_derive::{Deserialize, Serialize};
use cyclonedds_rs::*;
use crate::standard::Header;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Vector3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Acceleration {
    pub linear : Vector3,
    pub angular : Vector3,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct AccelerationStamped {
    pub header : Header,
    pub accel : Acceleration,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct AccelerationWithCovariance {
    pub accel : Acceleration,
    pub covariance : [[f32;6];6],
}   

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct AccelerationWithCovarianceStamped {
    pub header : Header,
    pub accel : AccelerationWithCovariance,
}

/// 
/// Inertia Tensor [kg-m^2]
///     | ixx ixy ixz |
/// I = | ixy iyy iyz |
///     | ixz iyz izz |
///
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Inertia {
    pub mass : f64,
    pub centre_of_mass : Vector3,
    pub ixx : f64,
    pub ixy : f64,
    pub ixz : f64,
    pub iyy : f64,
    pub iyz : f64,
    pub izz : f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct InertiaStamped {
    header : Header,
    inertia : Inertia,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Point32 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct PointStamped {
    pub header : Header,
    pub point : Point,    
}

///A specification of a polygon where the first and last points are assumed to be connected
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Polygon {
    pub points : Vec<Point32>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct PolygonStamped {
    pub header : Header,
    pub polygon : Polygon,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Quarternion {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct QuarternionStamped {
    pub header : Header,
    pub quaternion : Quarternion,
}

///# A representation of pose in free space, composed of position and orientation. 
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Pose {
    pub position : Point,
    pub orientation : Quarternion,
}

/// This represents the transform between two coordinate frames in free space.
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Transform {
    pub translation : Vector3,
    pub rotation : Quarternion,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct TransformStamped {
    pub header : Header,
    pub child_frame_id : String,
    pub transform : Transform,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Twist {
    pub linear : Vector3,
    pub angular : Vector3,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct TwistStamped {
    pub header : Header,
    pub twist : Twist,
}

/// This expresses velocity in free space with uncertainty.
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct TwistWithCovariance {
    pub twist : Twist,
    /// Row-major representation of the 6x6 covariance matrix
    /// The orientation parameters use a fixed-axis representation.
    /// In order, the parameters are:
    /// (x, y, z, rotation about X axis, rotation about Y axis, rotation about Z axis)
    pub covariance : [[f32;6];6],
}


#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct TwistWithCovarianceStamped {
    pub header : Header,
    pub twist : TwistWithCovariance,
}
/// This represents force in free space, separated into
/// its linear and angular parts.
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Wrench {
    pub force : Vector3,
    pub torque : Vector3,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct WrenchStamped {
    pub header : Header,
    pub wrench : Wrench,
}