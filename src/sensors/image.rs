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
Type8UC1
Type8UC2
Type8UC3
Type8UC4
Type8SC1
Type8SC2
Type8SC3
Type8SC4
Type16UC1
Type16UC2
Type16UC3
Type16UC4
Type16SC1
Type16SC2
Type16SC3
Type16SC4
Type32SC1
Type32SC2
Type32SC3
Type32SC4
Type32FC1
Type32FC2
Type32FC3
Type32FC4
Type64FC1
Type64FC2
Type64FC3
Type64FC4
BayerRGGB8
BayerBGGR8
BayerGBRG8
BayerGRBG8
BayerRGGB16
BayerBGGR16
BayerGBRG16
BayerGRBG16
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
    Type8UC1,
    Type8UC2,
    Type8UC3,
    Type8UC4,
    Type8SC1,
    Type8SC2,
    Type8SC3,
    Type8SC4,
    Type16UC1,
    Type16UC2,
    Type16UC3,
    Type16UC4,
    Type16SC1,
    Type16SC2,
    Type16SC3,
    Type16SC4,
    Type32SC1,
    Type32SC2,
    Type32SC3,
    Type32SC4,
    Type32FC1,
    Type32FC2,
    Type32FC3,
    Type32FC4,
    Type64FC1,
    Type64FC2,
    Type64FC3,
    Type64FC4,
    BayerRGGB8,
    BayerBGGR8,
    BayerGBRG8,
    BayerGRBG8,
    BayerRGGB16,
    BayerBGGR16,
    BayerGBRG16,
    BayerGrbg16,
    YUV422,
}

impl Encoding {
    pub const fn num_channels(&self) -> usize {
        match self {
            Self::MONO16 | Self::MONO8 => 1,
            Self::BGR8 | Self::RGB8 | Self::BGR16 | Self::RGB16 => 3,
            Self::BGRA8 | Self::RGBA8 | Self::BGRA16 | Self::RGBA16 => 4,
            Self::BayerRGGB8 | Self::BayerBGGR8 | Self::BayerGBRG8 | Self::BayerGRBG8 | Self::BayerRGGB16 | Self::BayerBGGR16 | Self::BayerGBRG16 | Self::BayerGrbg16 => 1,
            Self::Type16SC1 | Self::Type16UC1 | Self::Type32FC1 | Self::Type32SC1 | Self::Type64FC1 | Self::Type8SC1 | Self::Type8UC1 => 1,
            Self::Type16SC2 | Self::Type16UC2 | Self::Type32FC2 | Self::Type32SC2 | Self::Type64FC2 | Self::Type8SC2 | Self::Type8UC2 => 2,
            Self::Type16SC3 | Self::Type16UC3 | Self::Type32FC3 | Self::Type32SC3 | Self::Type64FC3 | Self::Type8SC3 | Self::Type8UC3 => 3,
            Self::Type16SC4 | Self::Type16UC4 | Self::Type32FC4 | Self::Type32SC4 | Self::Type64FC4 | Self::Type8SC4 | Self::Type8UC4 => 4,
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
            Self::MONO8 | Self::BGR8 | Self::RGB8 | Self::BGRA8 | Self:: RGBA8 | Self::BayerRGGB8 | Self::BayerBGGR8 | Self::BayerGBRG8 | Self::BayerGRBG8 => 8,
            Self::BGR16 | Self::RGB16 | Self::BGRA16 | Self::RGBA16 | Self::BayerRGGB16 | Self::BayerBGGR16 | Self::BayerGBRG16 | Self::BayerGrbg16 => 16,
            Self::YUV422 => 8,
            Self::Type16SC1 | Self::Type16SC2 | Self::Type16SC3 | Self::Type16SC4 => 16,
            Self::Type16UC1 | Self::Type16UC2 | Self::Type16UC3 | Self::Type16UC4 => 16,
            Self::Type32SC1 | Self::Type32SC2 | Self::Type32SC3 | Self::Type32SC4 => 32,
            Self::Type32FC1 | Self::Type32FC2 | Self::Type32FC3 | Self::Type32FC4 => 32,
            Self::Type64FC1 | Self::Type64FC2 | Self::Type64FC3 | Self::Type64FC4 => 64,
            Self::Type8SC1 | Self::Type8SC2 | Self::Type8SC3 | Self::Type8SC4 => 8,
            Self::Type8UC1 | Self::Type8UC2 | Self::Type8UC3 | Self::Type8UC4 => 8,
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
make_image_type!(Image1080pYUV422,1920,1080,Encoding::YUV422);
