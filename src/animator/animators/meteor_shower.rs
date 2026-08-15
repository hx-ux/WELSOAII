use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::AnimationType,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize, Deserialize)]
pub struct MeteorShowerSettings {
    pub count: ConstantParam<u32>,
    pub speed: ModulatedParam,
    pub trail_length: ModulatedParam,
    pub angle_spread: ModulatedParam,
    pub size: ModulatedParam,
    pub beat_burst: ConstantParam<u8>,
    color: ColorParam,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
}

impl MeteorShowerSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            count: ConstantParam::new(8, 1, 60, "Meteor Count", "meteor_count"),
            speed: ModulatedParam::new(600.0, 100.0, 2000.0, "Speed", "meteor_speed"),
            trail_length: ModulatedParam::new(12.0, 2.0, 40.0, "Trail Length", "meteor_trail"),
            angle_spread: ModulatedParam::new(0.4, 0.0, 1.57, "Angle Spread", "meteor_angle"),
            size: ModulatedParam::new(5.0, 2.0, 20.0, "Head Size", "meteor_size"),
            beat_burst: ConstantParam::new(0, 0, 20, "Beat Burst", "meteor_burst"),
            color: ColorParam::default(),
            dimension: *win_rect,
        }
    }
}

impl AnimatorSettings for MeteorShowerSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.speed,
            &mut self.trail_length,
            &mut self.angle_spread,
            &mut self.size,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> UpdateBehaviour {
        let mut change = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));

        if self.count.to_slider(ui) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.speed.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.trail_length.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.size.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.angle_spread.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.beat_burst.to_slider(ui) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.color.ui(ui) {
            change = UpdateBehaviour::HotUpdate;
        }

        change
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::MeteorShower
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        (0..self.count.value)
            .map(|idx| {
                Box::new(Meteor::new(
                    &self.dimension,
                    self.color.clone().value_mapped(idx as usize),
                    *self.speed.value(),
                    *self.trail_length.value() as usize,
                    *self.angle_spread.value(),
                    *self.size.value(),
                    self.beat_burst.value,
                    idx as usize,
                )) as Box<dyn AnimatedObject>
            })
            .collect()
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        let current = objects.len();
        let target = self.count.value as usize;

        if target > current {
            for idx in current..target {
                objects.push(Box::new(Meteor::new(
                    &self.dimension,
                    self.color.clone().value_mapped(idx),
                    *self.speed.value(),
                    *self.trail_length.value() as usize,
                    *self.angle_spread.value(),
                    *self.size.value(),
                    self.beat_burst.value,
                    idx,
                )));
            }
        } else if target < current {
            objects.truncate(target);
        }

        for obj in objects.iter_mut() {
            if let Some(m) = obj.as_any_mut().downcast_mut::<Meteor>() {
                m.speed = *self.speed.value();
                m.max_trail = *self.trail_length.value() as usize;
                m.size = *self.size.value();
                m.beat_burst = self.beat_burst.value;
                m.color = self.color.clone().value_mapped(m.index);
            }
        }
    }

    fn reset(&mut self) {
        self.speed.reset();
        self.trail_length.reset();
        self.angle_spread.reset();
        self.size.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

fn spawn_position(win_rect: &Rect) -> (Vec2, Vec2) {
    // Spawn from top or left edge going diagonally down-right
    let base_angle = std::f32::consts::PI * 1.25; // ~225° = down-left direction (meteor falls down-right from top-left)
    let spread = random_range(-0.3_f32, 0.3_f32);
    let angle = base_angle + spread;

    // Spawn from top edge or left edge
    let pos = if random_f32() > 0.5 {
        // top edge
        vec2(
            random_range(win_rect.left(), win_rect.right()),
            win_rect.top() + 20.0,
        )
    } else {
        // left edge
        vec2(
            win_rect.left() - 20.0,
            random_range(win_rect.bottom(), win_rect.top()),
        )
    };

    let vel = vec2(angle.cos(), angle.sin());
    (pos, vel)
}

pub struct Meteor {
    index: usize,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    size: f32,
    max_trail: usize,
    beat_burst: u8,
    trail: std::collections::VecDeque<Vec2>,
    color: Rgba8,
    dead: bool,
    last_beat_floor: u32,
    dimension: Rect,
}

impl Meteor {
    pub fn new(
        win_rect: &Rect,
        color: Rgba8,
        speed: f32,
        max_trail: usize,
        _angle_spread: f32,
        size: f32,
        beat_burst: u8,
        index: usize,
    ) -> Self {
        let (pos, dir) = spawn_position(win_rect);
        Self {
            index,
            position: pos,
            direction: dir,
            speed,
            size,
            max_trail,
            beat_burst,
            trail: std::collections::VecDeque::new(),
            color,
            dead: false,
            last_beat_floor: 0,
            dimension: *win_rect,
        }
    }

    fn respawn(&mut self) {
        let (pos, dir) = spawn_position(&self.dimension);
        self.position = pos;
        self.direction = dir;
        self.trail.clear();
        self.dead = false;
    }
}

impl AnimatedObject for Meteor {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode) {
        self.dimension = *win_rect;

        // Beat burst: on each beat, kick speed momentarily or respawn extra meteors
        let beat_floor = clock.get_beats() as u32;
        if beat_floor != self.last_beat_floor {
            self.last_beat_floor = beat_floor;
            if self.beat_burst > 0 && self.index < self.beat_burst as usize {
                self.respawn();
            }
        }

        self.trail.push_front(self.position);
        while self.trail.len() > self.max_trail.max(1) {
            self.trail.pop_back();
        }

        self.position += self.direction * self.speed * delta_time;

        // Respawn when out of bounds
        let margin = 50.0;
        if self.position.x < win_rect.left() - margin
            || self.position.x > win_rect.right() + margin
            || self.position.y < win_rect.bottom() - margin
            || self.position.y > win_rect.top() + margin
        {
            self.respawn();
        }
    }

    fn draw(&self, draw: &Draw) {
        let trail_len = self.trail.len();
        if trail_len == 0 {
            return;
        }

        // Draw trail as line segments with fading opacity and shrinking width
        for i in 0..trail_len.saturating_sub(1) {
            let t = 1.0 - (i as f32 / trail_len as f32);
            let alpha = (self.color.alpha as f32 * t * t) as u8;
            let weight = self.size * t * 0.6 + 1.0;
            let mut c = self.color;
            c.alpha = alpha;
            let a = self.trail[i];
            let b = self.trail[i + 1];
            draw.line().start(a).end(b).weight(weight).color(c);
        }

        // Draw bright head
        draw.ellipse()
            .xy(self.position)
            .radius(self.size)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.size)
    }

    fn color(&self) -> Rgba8 {
        self.color
    }

    fn is_dead(&self) -> bool {
        self.dead
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
