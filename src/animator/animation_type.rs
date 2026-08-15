use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq)]
// Defines, how the animators behave, if an Param is changed
pub enum UpdateBehaviour {
    None,
    // Resets the current animator and its object.
    // Mainly used for switching between Animators
    // Does call Animator::new()
    NeedsReset,
    // Hot updates, which affect the animator in the next frame(s)
    // Does not call Animator::new()
    HotUpdate,
    //
    LoadPreset,
    //
    SavePresets,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, EnumString, Display, EnumIter)]
pub enum AnimationType {
    #[default]
    #[strum(to_string = "Gravity Balls")]
    BouncingBalls = 0,
    #[strum(to_string = "Radial Burst")]
    PulseBackground = 1,
    #[strum(to_string = "Particle Sweep")]
    ScanLine = 2,
    #[strum(to_string = "Lissajous Grid")]
    WaveLines = 3,
    #[strum(to_string = "Meteor Shower")]
    MeteorShower = 4,
    #[strum(to_string = "Strobe")]
    Strobe = 5,
}

impl From<usize> for AnimationType {
    fn from(value: usize) -> Self {
        match value {
            _ if value == AnimationType::BouncingBalls as usize => AnimationType::BouncingBalls,
            _ if value == AnimationType::PulseBackground as usize => AnimationType::PulseBackground,
            _ if value == AnimationType::ScanLine as usize => AnimationType::ScanLine,
            _ if value == AnimationType::WaveLines as usize => AnimationType::WaveLines,
            _ if value == AnimationType::MeteorShower as usize => AnimationType::MeteorShower,
            _ if value == AnimationType::Strobe as usize => AnimationType::Strobe,
            _ => AnimationType::BouncingBalls,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum ScanLineModes {
    #[default]
    #[strum(to_string = "Ping Pong")]
    PingPong,
    #[strum(to_string = "Wrap Around")]
    WrapAround,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseModes {
    #[default]
    #[strum(to_string = "Smooth")]
    Smooth,
    #[strum(to_string = "Elastic")]
    Elastic,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseShape {
    #[default]
    #[strum(to_string = "Square")]
    Square,
    #[strum(to_string = "Circle")]
    Circle,
    #[strum(to_string = "Diamond")]
    Diamond,
}
