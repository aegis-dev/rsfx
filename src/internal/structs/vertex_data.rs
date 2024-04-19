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

use std::mem;
use gl::types::GLuint;

use super::vec2::Vec2;
use super::vec3::Vec3;

pub const VERTEX_POSITION_ATTRIBUTE_ID: GLuint = 0;
pub const VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_ID: GLuint = 1;
pub const VERTEX_NORMAL_ATTRIBUTE_ID: GLuint = 2;

pub const VERTEX_DATA_ATTRIBUTES: &'static [GLuint] = &[VERTEX_POSITION_ATTRIBUTE_ID, VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_ID, VERTEX_NORMAL_ATTRIBUTE_ID];

pub const VERTEX_DATA_POSITION_OFFSET: GLuint = 0;
pub const VERTEX_DATA_TEXTURE_COORDINATE_OFFSET: GLuint = VERTEX_DATA_POSITION_OFFSET + mem::size_of::<Vec3>() as GLuint;
pub const VERTEX_DATA_NORMAL_OFFSET: GLuint = VERTEX_DATA_TEXTURE_COORDINATE_OFFSET + mem::size_of::<Vec2>() as GLuint;

pub const VERTEX_POSITION_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec3>() as GLuint / mem::size_of::<f32>() as GLuint;
pub const VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec2>() as GLuint / mem::size_of::<f32>() as GLuint;
pub const VERTEX_NORMAL_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec3>() as GLuint / mem::size_of::<f32>() as GLuint;

pub const VERTEX_DATA_SIZE: GLuint = mem::size_of::<VertexData>() as GLuint;


#[repr(packed(1))]
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
}