use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::Header;

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct JointState {
    pub header: Header,
    pub name: Vec<String>,
    pub position: f64,
    pub velocity: f64,
    pub effort: f64,
}
