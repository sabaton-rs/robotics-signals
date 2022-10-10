use cdds_derive::*;
use cyclonedds_rs::*;
use serde_derive::{Deserialize, Serialize};

use crate::standard::Header;

#[repr(C)]
#[derive(Serialize, Deserialize, Topic)]
pub struct Joystick {
    pub header: Header,
    /// The axes measurements from a joystick.
    pub axes: Vec<f32>,
    /// The buttons measurements from a joystick.
    pub buttons: Vec<i32>,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum FeedbackType {
    Led = 0,
    Rumble = 1,
    Buzzer = 2,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct JoystickFeedback {
    pub ty: FeedbackType,
    /// This will hold an id number for each type of each feedback.
    /// Example, the first led would be id=0, the second would be id=1
    pub id: u8,
    /// Intensity of the feedback, from 0.0 to 1.0, inclusive.  If device is
    /// actually binary, driver should treat 0<=x<0.5 as off, 0.5<=x<=1 as on.
    pub intensity: f32,
}
