use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::Header;

/// Navigation Satellite fix for any Global Navigation Satellite System
///
/// Specified using the WGS 84 reference ellipsoid
///
#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct NavSatFix {
    /// header.stamp specifies the local time for this measurement (the
    /// corresponding satellite time may be reported using the
    /// TimeReference message).
    ///
    pub header: Header,

    pub status: NavSatStatus,

    /// Latitude [degrees]. Positive is north of equator
    pub latitude: f64,

    /// Longiture [degrees]. Positive is east of the prime meridian.
    pub longitude: f64,

    /// altitude [m]. NaN if altitude is not available.
    pub altitude: f64,

    /// Position covariance [m^2] defined relative to a tangential plane
    /// through the reported position. The components are East, North, and
    /// Up (ENU), in row-major order.
    ///
    /// Beware: this coordinate system exhibits singularities at the poles.
    ///
    pub position_covariance: [f64; 9],

    /// If the covariance of the fix is known, fill it in completely. If the
    /// GPS receiver provides the variance of each measurement, put them
    /// along the diagonal. If only Dilution of Precision is available,
    /// estimate an approximate covariance from that.
    pub covariance_type: CovarianceType,
}

///
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct NavSatStatus {
    pub status: NavSatFixType,
    /// Which Global Navigation Satellite System signals were
    /// used by the receiver.
    pub service: Vec<NavService>,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum NavSatFixType {
    /// No fix
    NoFix = -1,
    /// unaugmented fix
    Fix = 0,
    /// with satellite based augmentation
    SbasFix = 1,
    /// with ground based augmentation
    GbasFix = 2,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum NavService {
    Gps,
    Glonass,
    Compass,
    Galileo,
    Irnss,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum CovarianceType {
    Unknown,
    Approximated,
    DiagonalKnown,
    Known,
}
