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
#[derive(Serialize, Deserialize,PartialEq)]
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
    Type8uc1,
    Type8uc2,
    Type8uc3,
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
    BayerGrbg16,
    YUV422,
}

impl Encoding {
    pub const fn num_channels(&self) -> usize {
        match self {
            Self::MONO16 | Self::MONO8 => 1,
            Self::BGR8 | Self::RGB8 | Self::BGR16 | Self::RGB16 => 3,
            Self::BGRA8 | Self::RGBA8 | Self::BGRA16 | Self::RGBA16 => 4,
            Self::BAYER_RGGB8 | Self::BAYER_BGGR8 | Self::BAYER_GBRG8 | Self::BAYER_GRBG8 | Self::BAYER_RGGB16 | Self::BAYER_BGGR16 | Self::BAYER_GBRG16 | Self::BAYER_GRBG16 => 1,
            Self::TYPE_16SC1 | Self::TYPE_16UC1 | Self::TYPE_32FC1 | Self::TYPE_32SC1 | Self::TYPE_64FC1 | Self::TYPE_8SC1 | Self::TYPE_8UC1 => 1,
            Self::TYPE_16SC2 | Self::TYPE_16UC2 | Self::TYPE_32FC2 | Self::TYPE_32SC2 | Self::TYPE_64FC2 | Self::TYPE_8SC2 | Self::TYPE_8UC2 => 2,
            Self::TYPE_16SC3 | Self::TYPE_16UC3 | Self::TYPE_32FC3 | Self::TYPE_32SC3 | Self::TYPE_64FC3 | Self::TYPE_8SC3 | Self::TYPE_8UC3 => 3,
            Self::TYPE_16SC4 | Self::TYPE_16UC4 | Self::TYPE_32FC4 | Self::TYPE_32SC4 | Self::TYPE_64FC4 | Self::TYPE_8SC4 | Self::TYPE_8UC4 => 4,
            Self::YUV422 => 2,
            Self::Unknown => 0,
        }
    }

    pub fn is_mono(&self) -> bool {
        *self == Self::MONO16 || *self == Self::MONO8
    }

    pub fn is_color(&self) -> bool {
        *self == Self::RGB8 || *self == Self::BGR8 ||
        *self == Self::RGBA8 || *self == Self::BGRA8 ||
        *self == Self::RGB16 || *self == Self::BGR16 ||
        *self == Self::RGBA16|| *self == Self::BGRA16
    }

    pub fn has_alpha(&self) -> bool {
        *self == Self::RGBA8 || *self == Self::BGRA8 ||
        *self == Self::RGBA16 || *self == Self::BGRA16
    }

    pub const fn bit_depth(&self) -> usize {
        match self {
            Self::MONO16 => 16,
            Self::MONO8 | Self::BGR8 | Self::RGB8 | Self::BGRA8 | Self:: RGBA8 | Self::BAYER_RGGB8 | Self::BAYER_BGGR8 | Self::BAYER_GBRG8 | Self::BAYER_GRBG8 => 8,
            Self::BGR16 | Self::RGB16 | Self::BGRA16 | Self::RGBA16 | Self::BAYER_RGGB16 | Self::BAYER_BGGR16 | Self::BAYER_GBRG16 | Self::BAYER_GRBG16 => 16,
            Self::YUV422 => 8,
            Self::TYPE_16SC1 | Self::TYPE_16SC2 | Self::TYPE_16SC3 | Self::TYPE_16SC4 => 16,
            Self::TYPE_16UC1 | Self::TYPE_16UC2 | Self::TYPE_16UC3 | Self::TYPE_16UC4 => 16,
            Self::TYPE_32SC1 | Self::TYPE_32SC2 | Self::TYPE_32SC3 | Self::TYPE_32SC4 => 32,
            Self::TYPE_32FC1 | Self::TYPE_32FC2 | Self::TYPE_32FC3 | Self::TYPE_32FC4 => 32,
            Self::TYPE_64FC1 | Self::TYPE_64FC2 | Self::TYPE_64FC3 | Self::TYPE_64FC4 => 64,
            Self::TYPE_8SC1 | Self::TYPE_8SC2 | Self::TYPE_8SC3 | Self::TYPE_8SC4 => 8,
            Self::TYPE_8UC1 | Self::TYPE_8UC2 | Self::TYPE_8UC3 | Self::TYPE_8UC4 => 8,
            Self::Unknown => 0,
         }
    }
}



macro_rules! make_image_type {
    ( $resname:ident, $width:expr, $height:expr, $encoding_type:expr ) => {
    #[repr(C)]
    #[derive(Serialize, Deserialize, TopicFixedSize)]
        pub struct $resname {
            pub header: Header,
            pub width: u32,
            pub height: u32,
            pub encoding : Encoding,
            pub stride : u32,
            #[serde(with = "BigArray")]
            pub data : [u8;$width*$height*$encoding_type.num_channels()*$encoding_type.bit_depth()/8],
        }

        impl $resname {
            //Set the resolution of this fixed size image type
            pub fn set_resolution(&mut self) {
                self.width = $width;
                self.height = $height;
                self.stride = $width;
            }

            pub fn initialize(&mut self) {
                self.set_resolution();
                self.encoding = $encoding_type;
            }

        }
    };
}

// All the following Image types are fixed size types that can be transferred via 
// shared memory

make_image_type!(ImageQCIF4BPP,176,120,Encoding::RGBA8);
make_image_type!(ImageCIF4BPP,352,240,Encoding::RGBA8);
make_image_type!(Image3MP4BPP,2048,1536,Encoding::RGBA8);
make_image_type!(Image1080p4BPP,1920,1080,Encoding::RGBA8);