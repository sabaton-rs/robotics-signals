use crate::standard::Header;
use serde_derive::{Deserialize, Serialize};

use serde_big_array::BigArray;
use cdds_derive::*;
use cyclonedds_rs::*;

/*
RGB8
RGBA8
RGB16
RGBA16
BGR8
BGRA8
BGR16
BGRA16
MONO8
MONO16
TYPE_8UC1
TYPE_8UC2
TYPE_8UC3
TYPE_8UC4
TYPE_8SC1
TYPE_8SC2
TYPE_8SC3
TYPE_8SC4
TYPE_16UC1
TYPE_16UC2
TYPE_16UC3
TYPE_16UC4
TYPE_16SC1
TYPE_16SC2
TYPE_16SC3
TYPE_16SC4
TYPE_32SC1
TYPE_32SC2
TYPE_32SC3
TYPE_32SC4
TYPE_32FC1
TYPE_32FC2
TYPE_32FC3
TYPE_32FC4
TYPE_64FC1
TYPE_64FC2
TYPE_64FC3
TYPE_64FC4
BAYER_RGGB8
BAYER_BGGR8
BAYER_GBRG8
BAYER_GRBG8
BAYER_RGGB16
BAYER_BGGR16
BAYER_GBRG16
BAYER_GRBG16
*/

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum Encoding {
    Unknown,
    RGB8,
    RGBA8,
    RGB16,
    RGBA16,
    BGR8,
    BGRA8,
    BGR16,
    BGRA16,
    MONO8,
    MONO16,
    TYPE_8UC1,
    TYPE_8UC2,
    TYPE_8UC3,
    TYPE_8UC4,
    TYPE_8SC1,
    TYPE_8SC2,
    TYPE_8SC3,
    TYPE_8SC4,
    TYPE_16UC1,
    TYPE_16UC2,
    TYPE_16UC3,
    TYPE_16UC4,
    TYPE_16SC1,
    TYPE_16SC2,
    TYPE_16SC3,
    TYPE_16SC4,
    TYPE_32SC1,
    TYPE_32SC2,
    TYPE_32SC3,
    TYPE_32SC4,
    TYPE_32FC1,
    TYPE_32FC2,
    TYPE_32FC3,
    TYPE_32FC4,
    TYPE_64FC1,
    TYPE_64FC2,
    TYPE_64FC3,
    TYPE_64FC4,
    BAYER_RGGB8,
    BAYER_BGGR8,
    BAYER_GBRG8,
    BAYER_GRBG8,
    BAYER_RGGB16,
    BAYER_BGGR16,
    BAYER_GBRG16,
    BAYER_GRBG16,
}

macro_rules! make_image_type {
    ( $resname:ident, $width:expr, $height:expr, $bpp:expr ) => {
    #[repr(C)]
    #[derive(Serialize, Deserialize, TopicFixedSize)]
        pub struct $resname {
            pub header: Header,
            pub width: u32,
            pub height: u32,
            pub encoding : Encoding,
            pub is_bigendian : bool,
            pub stride : u32,
            #[serde(with = "BigArray")]
            pub data : [u8;$width*$height*$bpp],
        }

        impl $resname {
            //Set the resolution of this fixed size image type
            pub fn set_resolution(&mut self) {
                self.width = $width;
                self.height = $height;
                self.stride = $width;
            }
        }
    };
}

// All the following Image types are fixed size types that can be transferred via 
// shared memory

make_image_type!(ImageQCIF4BPP,176,120,4);
make_image_type!(ImageCIF4BPP,352,240,4);
make_image_type!(Image3MP4BPP,2048,1536,4);
make_image_type!(Image1080p4BPP,1920,1080,4);