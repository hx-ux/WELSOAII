use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum LayoutMode {
    Row,
    Colum,
}
