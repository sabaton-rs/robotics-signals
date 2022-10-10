use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::{Header, Timestamp};

/// Simple temperature reading
#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct TimeReference {
    // local timestamp of the measurement
    // frame_id is not used
    pub header: Header,

    /// Measurement of the Temperature in Degrees Celsius.
    pub time_ref: Timestamp,

    /// The source of this time
    pub source: String,
}
