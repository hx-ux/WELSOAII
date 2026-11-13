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
    #[strum(to_string = "Acid Mirage")]
    AcidMirage = 5,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum MirageMode {
    #[default]
    #[strum(to_string = "Kundalini")]
    Kundalini,
    #[strum(to_string = "Vortex")]
    HypnoVortex,
    #[strum(to_string = "Attractor Web")]
    AttractorWeb,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum ScanLineModes {
    #[default]
    #[strum(to_string = "Prism Lattice")]
    PrismLattice,
    #[strum(to_string = "Mirror Clash")]
    MirrorClash,
    #[strum(to_string = "Ping Pong")]
    PingPong,
    #[strum(to_string = "Wrap Around")]
    WrapAround,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseModes {
    #[default]
    #[strum(to_string = "Ripple Wave")]
    RippleWave,
    #[strum(to_string = "Heartbeat")]
    Heartbeat,
    #[strum(to_string = "Elastic")]
    Elastic,
    #[strum(to_string = "Smooth")]
    Smooth,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum PulseShape {
    #[default]
    #[strum(to_string = "Lotus")]
    Lotus,
    #[strum(to_string = "Star")]
    Star,
    #[strum(to_string = "Diamond")]
    Diamond,
    #[strum(to_string = "Hexagon")]
    Hexagon,
    #[strum(to_string = "Circle")]
    Circle,
    #[strum(to_string = "Square")]
    Square,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum WaveMode {
    #[default]
    #[strum(to_string = "Aurora Silk")]
    AuroraSilk,
    #[strum(to_string = "DNA Braid")]
    DnaBraid,
    #[strum(to_string = "Cyber Ocean")]
    CyberOcean,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum TunnelShape {
    #[default]
    #[strum(to_string = "Sacred Star")]
    SacredStar,
    #[strum(to_string = "Hyperspace")]
    HyperCircle,
    #[strum(to_string = "Crystal Hex")]
    CrystalHex,
}
