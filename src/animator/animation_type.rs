use std::slice::Iter;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationType {
    BouncingBalls, 
    GravityFountain,
    ScanLine,
    PulseBackground,
}

impl AnimationType {
    pub fn iterator() -> Iter<'static, AnimationType> {
        static ANIMATION_TYPE: [AnimationType; 4] = [
            AnimationType::BouncingBalls,
            AnimationType::GravityFountain,
            AnimationType::ScanLine,
            AnimationType::PulseBackground,
        ];
        ANIMATION_TYPE.iter()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimationType::BouncingBalls => "Bouncing Balls",
            AnimationType::GravityFountain => "Gravity Fountain",
            AnimationType::ScanLine => "Scan Line",
            AnimationType::PulseBackground => "Pulse",
        }
    }
}
