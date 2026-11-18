use nannou::prelude::{BlendComponent, BlendFactor, BlendOperation, PolygonMode, Srgba};
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
    #[strum(to_string = "Strobe")]
    Strobe = 5,
}

/// GPU & WLED grid blend modes powered by Bevy/wgpu render pipelines.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum LayerBlendMode {
    #[default]
    #[strum(to_string = "Normal")]
    Normal,
    #[strum(to_string = "Additive")]
    Add,
    #[strum(to_string = "Lightest")]
    Lightest,
    #[strum(to_string = "Screen")]
    Screen,
    #[strum(to_string = "Multiply")]
    Multiply,
    #[strum(to_string = "Subtract")]
    Subtract,
}

impl LayerBlendMode {
    pub fn short_label(self) -> &'static str {
        match self {
            LayerBlendMode::Normal => "NRM",
            LayerBlendMode::Add => "ADD",
            LayerBlendMode::Lightest => "MAX",
            LayerBlendMode::Screen => "SCR",
            LayerBlendMode::Multiply => "MUL",
            LayerBlendMode::Subtract => "SUB",
        }
    }

    /// Converts this blend mode into a Bevy/wgpu `BlendComponent` for `draw.color_blend(...)`.
    pub fn to_wgpu_blend(self) -> BlendComponent {
        match self {
            LayerBlendMode::Normal => BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            LayerBlendMode::Add => BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            LayerBlendMode::Lightest => BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Max,
            },
            LayerBlendMode::Screen => BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrc,
                operation: BlendOperation::Add,
            },
            LayerBlendMode::Multiply => BlendComponent {
                src_factor: BlendFactor::Dst,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            LayerBlendMode::Subtract => BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::ReverseSubtract,
            },
        }
    }

    /// Software equivalent for compositing layers onto the WLED `ReceiverGrid`.
    pub fn blend_srgba(self, dst: Srgba, src: Srgba) -> Srgba {
        let sa = src.alpha.clamp(0.0, 1.0);
        let sr = src.red.clamp(0.0, 1.0);
        let sg = src.green.clamp(0.0, 1.0);
        let sb = src.blue.clamp(0.0, 1.0);

        let dr = dst.red.clamp(0.0, 1.0);
        let dg = dst.green.clamp(0.0, 1.0);
        let db = dst.blue.clamp(0.0, 1.0);
        let da = dst.alpha.clamp(0.0, 1.0);

        match self {
            LayerBlendMode::Normal => Srgba::new(
                (sr * sa + dr * (1.0 - sa)).clamp(0.0, 1.0),
                (sg * sa + dg * (1.0 - sa)).clamp(0.0, 1.0),
                (sb * sa + db * (1.0 - sa)).clamp(0.0, 1.0),
                (sa + da * (1.0 - sa)).clamp(0.0, 1.0),
            ),
            LayerBlendMode::Add => Srgba::new(
                (dr + sr * sa).clamp(0.0, 1.0),
                (dg + sg * sa).clamp(0.0, 1.0),
                (db + sb * sa).clamp(0.0, 1.0),
                (da + sa).clamp(0.0, 1.0),
            ),
            LayerBlendMode::Lightest => Srgba::new(
                dr.max(sr * sa),
                dg.max(sg * sa),
                db.max(sb * sa),
                da.max(sa),
            ),
            LayerBlendMode::Screen => {
                let s_r = sr * sa;
                let s_g = sg * sa;
                let s_b = sb * sa;
                Srgba::new(
                    (s_r + dr * (1.0 - s_r)).clamp(0.0, 1.0),
                    (s_g + dg * (1.0 - s_g)).clamp(0.0, 1.0),
                    (s_b + db * (1.0 - s_b)).clamp(0.0, 1.0),
                    (sa + da * (1.0 - sa)).clamp(0.0, 1.0),
                )
            }
            LayerBlendMode::Multiply => Srgba::new(
                (dr * sr * sa + dr * (1.0 - sa)).clamp(0.0, 1.0),
                (dg * sg * sa + dg * (1.0 - sa)).clamp(0.0, 1.0),
                (db * sb * sa + db * (1.0 - sa)).clamp(0.0, 1.0),
                da.max(sa),
            ),
            LayerBlendMode::Subtract => Srgba::new(
                (dr - sr * sa).clamp(0.0, 1.0),
                (dg - sg * sa).clamp(0.0, 1.0),
                (db - sb * sa).clamp(0.0, 1.0),
                da,
            ),
        }
    }
}

/// GPU polygon rasterization mode powered by Bevy/wgpu `PolygonMode`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Display, EnumIter)]
pub enum LayerPolygonMode {
    #[default]
    #[strum(to_string = "Fill")]
    Fill,
    #[strum(to_string = "Wireframe")]
    Line,
    #[strum(to_string = "Vertices")]
    Point,
}

impl LayerPolygonMode {
    pub fn short_label(self) -> &'static str {
        match self {
            LayerPolygonMode::Fill => "FILL",
            LayerPolygonMode::Line => "WIRE",
            LayerPolygonMode::Point => "PTS",
        }
    }

    pub fn to_wgpu_polygon_mode(self) -> PolygonMode {
        match self {
            LayerPolygonMode::Fill => PolygonMode::Fill,
            LayerPolygonMode::Line => PolygonMode::Line,
            LayerPolygonMode::Point => PolygonMode::Point,
        }
    }

    pub fn is_wireframe(self) -> bool {
        !matches!(self, LayerPolygonMode::Fill)
    }
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
