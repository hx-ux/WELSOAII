use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
// Defines how the animators behave if a Param is changed
pub enum UpdateBehaviour {
    None,
    // Resets the current animator and its object.
    // Mainly used for switching between Animators
    // Does call Animator::new()
    NeedsReset,
    // Hot updates, which affect the animator in the next frame(s)
    // Does not call Animator::new()
    HotUpdate,
    LoadPreset,
    SavePresets,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, EnumString, Display, EnumIter)]
pub enum AnimationType {
    #[default]
    #[strum(to_string = "Bouncing Balls")]
    BouncingBalls = 0,
    #[strum(to_string = "Pulse Background")]
    PulseBackground = 1,
    #[strum(to_string = "Scan Line")]
    ScanLine = 2,
    #[strum(to_string = "Wave Lines")]
    WaveLines = 3,
    #[strum(to_string = "Quantum Tunnel")]
    QuantumTunnel = 4,
    // #[strum(to_string = "Strobe")]
    // Strobe = 5,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum ScanLineModes {
    #[default]
    #[strum(to_string = "Ping Pong")]
    PingPong,
    #[strum(to_string = "Wrap Around")]
    WrapAround,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseModes {
    #[default]
    #[strum(to_string = "Smooth")]
    Smooth,
    #[strum(to_string = "Elastic")]
    Elastic,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseShape {
    #[default]
    #[strum(to_string = "Square")]
    Square,
    #[strum(to_string = "Circle")]
    Circle,
    #[strum(to_string = "Diamond")]
    Diamond,
}
