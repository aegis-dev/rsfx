//
// Copyright © 2020-2024  Egidijus Lileika
//
// This file is part of RSFX - Game framework for PSX-feel games written in Rust
//
// RSFX is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// RSFX is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with RSFX. If not, see <https://www.gnu.org/licenses/>.
//

use glam::Vec3;

use crate::transform::{Transform, Transformable};

pub struct TransformAnimationBuilder {
    steps: Vec<TransformAnimationStep>,
}

impl TransformAnimationBuilder {
    pub fn new() -> TransformAnimationBuilder {
        TransformAnimationBuilder { steps: vec![] }
    }

    pub fn init(&mut self, transform: Transform) {
        self.steps.push(TransformAnimationStep::Init(transform));
    }

    pub fn transform(&mut self, transform: Transform, timestamp: f64) {
        self.steps.push(TransformAnimationStep::Transform(transform, timestamp));
    }

    pub fn do_nothing(&mut self, timestamp: f64) {
        self.steps.push(TransformAnimationStep::DoNothing(timestamp));
    }

    pub fn restart(&mut self) {
        self.steps.push(TransformAnimationStep::Restart());
    }

    pub fn build(&self) -> TransformAnimation {
        TransformAnimation {
            transform: Transform::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 1.0),
            started: false,
            time: 0.0,
            steps: self.steps.clone(),
            step_idx : 0,
        }
    }
}

pub struct TransformAnimation {
    transform: Transform,
    started: bool,
    time: f64,
    steps: Vec<TransformAnimationStep>,
    step_idx: usize,
}

impl TransformAnimation {
    pub fn start(&mut self) {
        self.time = 0.0;
        self.started = true;

        assert!(self.steps.len() > 1);
    }

    pub fn stop(&mut self) {
        self.started = false;
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn resume(&mut self) {
        self.started = true;
    }

    pub fn update(&mut self, delta_time: f64) {
        if !self.started {
            return;
        }

        let time_before_delta = self.time;
        self.time += delta_time;

        if self.step_idx >= self.steps.len() {
            self.step_idx = 0;
        }

        let current_step = &self.steps[self.step_idx];
        match current_step {
            TransformAnimationStep::Init(transform) => {
                self.transform = transform.clone();
                if self.step_idx != self.steps.len() {
                    self.step_idx += 1;
                } else {
                    self.started = false;
                }
            }
            TransformAnimationStep::Transform(target, timestamp) => {
                if self.time >= *timestamp {
                    self.transform = target.clone();
                    if self.step_idx != self.steps.len() {
                        self.step_idx += 1;
                    } else {
                        self.started = false;
                    }
                }

                let time_left = timestamp - time_before_delta;
                let completed = delta_time / time_left;

                let mut pos = self.transform.get_position().clone();
                let target_pos = target.get_position();
                pos.x += (target_pos.x - pos.x) * completed as f32;
                pos.y += (target_pos.y - pos.y) * completed as f32;
                pos.z += (target_pos.z - pos.z) * completed as f32;

                let mut rot = self.transform.get_rotation().clone();
                let target_rot = target.get_rotation();
                rot.x += (target_rot.x - rot.x) * completed as f32;
                rot.y += (target_rot.y - rot.y) * completed as f32;
                rot.z += (target_rot.z - rot.z) * completed as f32;

                let mut scale = self.transform.get_scale();
                let target_scale = target.get_scale();
                scale += (target_scale - scale) * completed as f32;

                self.transform.set_position(pos);
                self.transform.set_rotation(rot);
                self.transform.set_scale(scale);
            }
            TransformAnimationStep::DoNothing(timestamp) => {
                if self.time >= *timestamp {
                    if self.step_idx != self.steps.len() {
                        self.step_idx += 1;
                    } else {
                        self.started = false;
                    }
                }
            }
            TransformAnimationStep::Restart() => {
                self.step_idx = 0;
                self.time = 0.0;
        }
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

#[derive(Copy, Clone)]
enum TransformAnimationStep {
    Init(Transform),
    Transform(Transform, f64),
    DoNothing(f64),
    Restart()
}



