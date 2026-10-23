use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter, Serialize, Deserialize, PartialEq, Eq)]
pub enum Polarity {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "+-")]
    PlusMinus,
}

impl Polarity {
    #[allow(dead_code)]
    pub fn range(&self) -> RangeInclusive<f32> {
        match self {
            Polarity::Plus => RangeInclusive::new(0.0, 1.0),
            Polarity::Minus => RangeInclusive::new(0.0, 1.0),
            Polarity::PlusMinus => RangeInclusive::new(-1.0, 1.0),
        }
    }
}

