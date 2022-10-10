use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::Header;

/// This message holds a collection of N-dimensional points, which may
/// contain additional information such as normals, intensity, etc. The
/// point data is stored as a binary blob, its layout described by the
/// contents of the "fields" array.
///
/// The point cloud data may be organized 2d (image-like) or 1d (unordered).
/// Point clouds organized as 2d images may be produced by camera depth sensors
/// such as stereo or time-of-flight.
#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct PointCloud {
    /// Time of sensor data acquisition, and the coordinate frame ID (for 3d points).
    pub header: Header,

    /// 2D structure of the point cloud. If the point cloud is
    /// unordered, height is 1 and width is the length of the point cloud.
    pub height: u32,
    pub width: u32,

    /// Describes the channels and their layout in the binary data blob.
    pub fields: Vec<PointField>,

    pub is_bigendian: bool,
    /// length of a point in bytes
    pub point_step: u32,
    /// length of a row in bytes
    pub row_step: u32,
    /// Point data. size is (row_step*height)
    pub data: Vec<u8>,

    /// true if there are no invalid points
    pub is_dense: bool,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum PointFieldDataType {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Float32,
    Float64,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct PointField {
    /// name of field
    pub name: String,
    /// offset from start of point struct
    pub offset: u32,
    pub datatype: PointFieldDataType,
    pub cound: u32,
}
