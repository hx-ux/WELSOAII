use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Clone, Display, EnumIter, Serialize, Deserialize, PartialEq)]
pub enum Polarity {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "+-")]
    PlusMinus,
}

impl Polarity {
    pub fn range(&self) -> RangeInclusive<f32> {
        match self {
            Polarity::Plus => RangeInclusive::new(0.0, 1.0),
            Polarity::Minus => RangeInclusive::new(0.0, 1.0),
            Polarity::PlusMinus => RangeInclusive::new(-1.0, 1.0),
        }
    }
}
