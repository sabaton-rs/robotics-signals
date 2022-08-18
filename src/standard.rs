use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use cdds_derive::Topic;
use cyclonedds_rs::*;

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct ColorRGBA {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32, 
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Header {
    pub seq : u32,
    pub stamp : chrono::DateTime<Utc>,
    pub frame : String,
}