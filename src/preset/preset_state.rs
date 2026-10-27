use crate::ui;

#[derive(Default, Clone)]
pub struct PresetState {
    pub prest_save_name: String, // text box for saving
    pub available: Vec<String>,
    pub selected: String,
}
