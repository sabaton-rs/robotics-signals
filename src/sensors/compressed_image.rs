use crate::standard::Header;
use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum CompressedImageType {
    Jpeg,
    Png,
    Tiff,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct CompressedImage {
    pub header: Header,
    pub format: CompressedImageType,
    pub data: Vec<u8>,
}
