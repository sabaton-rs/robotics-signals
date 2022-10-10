use crate::{
    geometry::{Point, Pose, PoseStamped, PoseWithCovariance, TwistWithCovariance},
    standard::{Header, Timestamp},
};
use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct GridCells {
    pub header: Header,
    /// width of each cell
    pub cell_width: f32,
    /// height of each cell
    pub cell_height: f32,
    /// each cell is represented by the point at the centre
    /// of the cell
    pub cells: Vec<Point>,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct MapMetadata {
    /// time at which the map is loaded
    pub map_load_time: Timestamp,

    /// the map resolution [m/cell]
    pub resolution: f32,

    /// map width in cells
    pub width: u32,

    /// map height in cells
    pub height: u32,

    /// The origin of the map [m,m, rad]. This is the real
    /// world post of the bottom left corner of cell (0,0)
    /// in the map.
    pub origin: Pose,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct OccupancyGrid {
    pub header: Header,

    pub info: MapMetadata,

    /// The map data, in row-major order, starting with (0,0).
    /// Cell (1,0) will be listed second, representing the next cell in
    /// the x direction.  Cell (0,1) will be at the index equal to info.width, followed
    /// by (1,1)
    pub data: Vec<u8>,
}

/// This represents an estimate of a position and velocity in free space.
/// The pose in this message should be specified in the coordinate frame given by header.frame_id
/// The twist in this message should be specified in the coordinate frame given by the child_frame_id
#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Odometry {
    pub header: Header,

    /// Frame id the pose points to. The twist is in this coordinate frame.
    pub child_frame_id: String,

    /// Estimated pose that is typically relative to a fixed world frame.
    pub pose: PoseWithCovariance,

    /// Estimated linear and angular velocity relative to child_frame_id.
    pub twist: TwistWithCovariance,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Path {
    pub header: Header,

    /// Array of poses to follow.
    pub poses: Vec<PoseStamped>,
}
