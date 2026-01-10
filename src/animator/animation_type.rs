use serde::{Deserialize, Serialize};
use std::slice::Iter;

pub trait ModeHelper: Sized {
    fn iterator() -> Iter<'static, Self>;
    fn as_str(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum AnimationType {
    #[default]
    BouncingBalls,
    GravityFountain,
    ScanLine,
    PulseBackground,
}

impl ModeHelper for AnimationType {
    fn iterator() -> Iter<'static, AnimationType> {
        static ANIMATION_TYPE: [AnimationType; 4] = [
            AnimationType::BouncingBalls,
            AnimationType::GravityFountain,
            AnimationType::ScanLine,
            AnimationType::PulseBackground,
        ];
        ANIMATION_TYPE.iter()
    }
    fn as_str(&self) -> &'static str {
        match self {
            AnimationType::BouncingBalls => "Bouncing Balls",
            AnimationType::GravityFountain => "Gravity Fountain",
            AnimationType::ScanLine => "Scan Line",
            AnimationType::PulseBackground => "Pulse",
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ScanLineModes {
    #[default]
    PingPong,
    WrapAround,
}

impl ModeHelper for ScanLineModes {
    fn iterator() -> Iter<'static, ScanLineModes> {
        static ANIMATION_TYPE: [ScanLineModes; 2] =
            [ScanLineModes::PingPong, ScanLineModes::WrapAround];
        ANIMATION_TYPE.iter()
    }
    fn as_str(&self) -> &'static str {
        match self {
            ScanLineModes::PingPong => "Ping Pong",
            ScanLineModes::WrapAround => "Wrap Around",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum PulseModes {
    #[default]
    Smooth,
    Flash,
}

impl ModeHelper for PulseModes {
    fn iterator() -> Iter<'static, PulseModes> {
        static ANIMATION_TYPE: [PulseModes; 2] = [PulseModes::Smooth, PulseModes::Flash];
        ANIMATION_TYPE.iter()
    }
    fn as_str(&self) -> &'static str {
        match self {
            PulseModes::Smooth => "Smooth",
            PulseModes::Flash => "Flash",
        }
    }
}
