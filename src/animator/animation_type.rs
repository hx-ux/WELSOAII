use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Clone, Copy, Default, EnumString, Display, EnumIter)]
pub enum AnimationType {
    #[default]
    #[strum(to_string = "Bouncing Balls")]
    BouncingBalls,
    #[strum(to_string = "Gravity Fountain")]
    ScanLine,
    #[strum(to_string = "Pulse Background")]
    PulseBackground,
    #[strum(to_string = "Wave Lines")]
    WaveLines,
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
    #[strum(to_string = "Wrap Around")]
    Smooth,
    #[strum(to_string = "Wrap Around")]
    Elastic,
}
