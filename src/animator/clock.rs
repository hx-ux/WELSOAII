use crate::animator::clock::Clock;
use nannou::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of wave generators
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WaveType {
    Sine,
    Sawtooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnvelopeMode {
    Up,
    Down,
}

/// Representation of a signal value that can be sampled over time
#[derive(Debug, Clone)]
pub struct SignalValue(pub f32);

/// LFO (Low Frequency Oscillator) generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LfoGenerator {
    pub wave_type: WaveType,
    pub frequency_hz: f32,
    pub amplitude: f32,
    pub offset: f32,
    pub phase_offset: f32,
    #[serde(skip)]
    clock: Clock,
}

impl LfoGenerator {
    pub fn new(wave_type: WaveType, frequency_hz: f32, amplitude: f32, offset: f32) -> Self {
        Self {
            wave_type,
            frequency_hz,
            amplitude,
            offset,
            phase_offset: 0.0,
            clock: Clock::new(),
        }
    }

    pub fn sample(&mut self, beats_per_minute: f32, delta_time: f32) -> SignalValue {
        self.clock.update(beats_per_minute, delta_time);

        let time_seconds = self.clock.get_time();
        let phase = time_seconds * self.frequency_hz * TAU + self.phase_offset;

        let raw_value = match self.wave_type {
            WaveType::Sine => phase.sin(),
            WaveType::Sawtooth => {
                let normalized_phase = (phase / TAU).fract();
                normalized_phase * 2.0 - 1.0
            }
        };

        SignalValue(raw_value * self.amplitude + self.offset)
    }

    pub fn set_phase_offset(&mut self, phase: f32) {
        self.phase_offset = phase;
    }
}

/// Envelope generator with pulse modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeGenerator {
    pub mode: EnvelopeMode,
    pub attack_time: f32,  // in beats
    pub release_time: f32, // in beats
    pub min_value: f32,
    pub max_value: f32,
    pub triggered: bool,
    #[serde(skip)]
    pub elapsed_beats: f32,
    #[serde(skip)]
    clock: Clock,
}

impl EnvelopeGenerator {
    pub fn new(mode: EnvelopeMode) -> Self {
        Self {
            mode,
            attack_time: 1.0,
            release_time: 1.0,
            min_value: 0.0,
            max_value: 1.0,
            triggered: false,
            elapsed_beats: 0.0,
            clock: Clock::new(),
        }
    }

    pub fn trigger(&mut self) {
        self.triggered = true;
        self.elapsed_beats = 0.0;
    }

    pub fn release(&mut self) {
        self.triggered = false;
    }

    pub fn sample(&mut self, beats_per_minute: f32, delta_time: f32) -> SignalValue {
        self.clock.update(beats_per_minute, delta_time);
        let beats_elapsed = self.clock.get_beats();

        match self.mode {
            EnvelopeMode::Up => {
                if self.triggered {
                    self.elapsed_beats += beats_elapsed;
                    let progress = (self.elapsed_beats / self.attack_time).clamp(0.0, 1.0);
                    SignalValue(self.min_value + progress * (self.max_value - self.min_value))
                } else {
                    self.elapsed_beats = 0.0;
                    SignalValue(self.min_value)
                }
            }
            EnvelopeMode::Down => {
                if self.triggered {
                    self.elapsed_beats += beats_elapsed;
                    let progress = (self.elapsed_beats / self.release_time).clamp(0.0, 1.0);
                    SignalValue(self.max_value - progress * (self.max_value - self.min_value))
                } else {
                    self.elapsed_beats = 0.0;
                    SignalValue(self.max_value)
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        if self.triggered {
            match self.mode {
                EnvelopeMode::Up => self.elapsed_beats >= self.attack_time,
                EnvelopeMode::Down => self.elapsed_beats >= self.release_time,
            }
        } else {
            true
        }
    }
}

/// Generic signal generator enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalGenerator {
    Lfo(LfoGenerator),
    Envelope(EnvelopeGenerator),
}

impl SignalGenerator {
    pub fn sample(&mut self, bpm: f32, delta_time: f32) -> SignalValue {
        match self {
            SignalGenerator::Lfo(lfo) => lfo.sample(bpm, delta_time),
            SignalGenerator::Envelope(env) => env.sample(bpm, delta_time),
        }
    }

    pub fn trigger(&mut self) {
        if let SignalGenerator::Envelope(env) = self {
            env.trigger();
        }
    }

    pub fn release(&mut self) {
        if let SignalGenerator::Envelope(env) = self {
            env.release();
        }
    }

    pub fn is_envelope_finished(&self) -> bool {
        match self {
            SignalGenerator::Lfo(_) => false,
            SignalGenerator::Envelope(env) => env.is_finished(),
        }
    }
}
