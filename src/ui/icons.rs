//Phosphor Icons
#![allow(dead_code)]

use crate::modulator::wave_modulator::LfoWave;

// Transport & Master Clock
pub const ICON_PLAY: &str = "\u{E3D0}";
pub const ICON_PAUSE: &str = "\u{E39E}";
pub const ICON_STOP: &str = "\u{E46C}";
pub const ICON_METRONOME: &str = "\u{EC8E}";
pub const ICON_LINK: &str = "\u{E2E6}";
pub const ICON_CLOCK: &str = "\u{E19A}";

// Parameter & Modulation Controls
pub const ICON_REFRESH: &str = "\u{E038}";
pub const ICON_MOD_CONNECTED: &str = "\u{E5E4}"; // Atom / active modulation matrix
pub const ICON_MOD_NOT_CONNECTED: &str = "\u{E18A}"; // Clean ring
pub const ICON_SLIDERS: &str = "\u{E434}";
pub const ICON_FADERS: &str = "\u{E228}";

// Actions & Navigation
pub const ICON_ADD: &str = "\u{E3D4}";
pub const ICON_PLUS_CIRCLE: &str = "\u{E3D6}";
pub const ICON_REMOVE: &str = "\u{E4F6}";
pub const ICON_TRASH: &str = "\u{E4A6}";
pub const ICON_PLUS: &str = "\u{E3D4}";
pub const ICON_MINUS: &str = "\u{E32A}";
pub const ICON_PLUS_MINUS: &str = "\u{E3D8}";
pub const ICON_CARET_DOWN: &str = "\u{E136}";
pub const ICON_CARET_RIGHT: &str = "\u{E13A}";
pub const ICON_CARET_UP: &str = "\u{E13C}";
pub const ICON_CHECK: &str = "\u{E182}";
pub const ICON_CLEAR: &str = "\u{EC54}";

// Visibility & Glass / Ghost HUD Mode
pub const ICON_EYE_ACTIVE: &str = "\u{E220}";
pub const ICON_EYE_DISABLED: &str = "\u{E224}";
pub const ICON_GLASS_HUD: &str = "\u{EC7C}"; // Transparent cube
pub const ICON_GHOST: &str = "\u{E62A}";
pub const ICON_PRESENTATION: &str = "\u{E626}"; // Frame corners

// LFO Waveforms
pub const ICON_WAVE_SINE: &str = "\u{EA9A}";
pub const ICON_WAVE_TRIANGLE: &str = "\u{EAA0}";
pub const ICON_WAVE_SQUARE: &str = "\u{EA9E}";
pub const ICON_WAVE_SAWTOOTH: &str = "\u{EA9C}";
pub const ICON_WAVE_RAMP_UP: &str = "\u{E4AE}";
pub const ICON_WAVE_RAMP_DOWN: &str = "\u{E4AC}";

// Modules & Hardware Drawers
pub const ICON_LAYERS: &str = "\u{E466}";
pub const ICON_DEVICE: &str = "\u{E296}";
pub const ICON_BROADCAST: &str = "\u{E0F2}";
pub const ICON_PLUG_CONNECTED: &str = "\u{EB5A}";
pub const ICON_PLUG: &str = "\u{E946}";
pub const ICON_SETTINGS: &str = "\u{E272}";
pub const ICON_TERMINAL: &str = "\u{EAE8}";
pub const ICON_SCENE: &str = "\u{E6A2}";
pub const ICON_SAVE: &str = "\u{E248}";
pub const ICON_FOLDER: &str = "\u{E256}";
pub const ICON_PALETTE: &str = "\u{E5B8}";
pub const ICON_GAUGE: &str = "\u{E628}";
pub const ICON_SHUFFLE: &str = "\u{E422}";
pub const ICON_LOCK: &str = "\u{E2FA}";
pub const ICON_INFO: &str = "\u{E2CE}";
pub const ICON_WARNING: &str = "\u{E4E0}";

// Animator Engine Icons
pub const ICON_ANIM_BALLS: &str = "\u{E192}";
pub const ICON_ANIM_PULSE: &str = "\u{E47C}";
pub const ICON_ANIM_SCAN: &str = "\u{E0B8}";
pub const ICON_ANIM_WAVES: &str = "\u{E6DE}";
pub const ICON_ANIM_TUNNEL: &str = "\u{E9FA}";

/// Returns the waveform glyph for a given `LfoWave`.
pub const fn lfo_wave_icon(wave: LfoWave) -> &'static str {
    match wave {
        LfoWave::Sine => ICON_WAVE_SINE,
        LfoWave::Pyramid => ICON_WAVE_TRIANGLE,
        LfoWave::Square => ICON_WAVE_SQUARE,
        LfoWave::RampUp => ICON_WAVE_RAMP_UP,
        LfoWave::RampDown => ICON_WAVE_RAMP_DOWN,
    }
}
