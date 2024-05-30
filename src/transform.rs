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

use glam::{Mat4, Vec3};
use crate::matrices::build_transformation_matrix;

#[derive(Copy, Clone)]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: f32
}

pub trait Transformable {
    fn get_position(&self) -> &Vec3;

    fn set_position(&mut self, position: Vec3);

    fn get_rotation(&self) -> &Vec3;

    fn set_rotation(&mut self, rotation: Vec3);

    fn get_scale(&self) -> f32;

    fn set_scale(&mut self, scale: f32);

    fn get_matrix(&self) -> Mat4;
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: f32) -> Transform {
        Transform { position, rotation, scale }
    }
}

impl Transformable for Transform {
    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    fn get_rotation(&self) -> &Vec3 {
        &self.rotation
    }

    fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn get_matrix(&self) -> Mat4 {
        build_transformation_matrix(&self.position, &self.rotation, self.scale)
    }
}