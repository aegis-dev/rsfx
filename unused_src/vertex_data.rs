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

use glam::{Vec3, Vec2};

#[repr(packed(4))]
#[derive(Copy, Clone)]
pub struct VertexData {
    pub position: Vec3,
    pub texture_coordinate: Vec2,
    pub normal: Vec3,
}

impl VertexData {
    pub fn new(position: Vec3, texture_coordinate: Vec2, normal: Vec3) -> VertexData {
        VertexData { position, texture_coordinate, normal }
    }
    
    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }
}