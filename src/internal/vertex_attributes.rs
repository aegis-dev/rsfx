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
use glam::{Vec2, Vec3};
use crate::vertex_data::VertexData;

pub(crate) const VERTEX_POSITION_ATTRIBUTE_ID: GLuint = 0;
pub(crate) const VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_ID: GLuint = 1;
pub(crate) const VERTEX_NORMAL_ATTRIBUTE_ID: GLuint = 2;

pub(crate) const VERTEX_DATA_ATTRIBUTES: &'static [GLuint] = &[VERTEX_POSITION_ATTRIBUTE_ID, VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_ID, VERTEX_NORMAL_ATTRIBUTE_ID];

pub(crate) const VERTEX_DATA_POSITION_OFFSET: GLuint = 0;
pub(crate) const VERTEX_DATA_TEXTURE_COORDINATE_OFFSET: GLuint = VERTEX_DATA_POSITION_OFFSET + mem::size_of::<Vec3>() as GLuint;
pub(crate) const VERTEX_DATA_NORMAL_OFFSET: GLuint = VERTEX_DATA_TEXTURE_COORDINATE_OFFSET + mem::size_of::<Vec2>() as GLuint;

pub(crate) const VERTEX_POSITION_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec3>() as GLuint / mem::size_of::<f32>() as GLuint;
pub(crate) const VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec2>() as GLuint / mem::size_of::<f32>() as GLuint;
pub(crate) const VERTEX_NORMAL_ATTRIBUTE_SIZE_IN_FLOATS: GLuint = mem::size_of::<Vec3>() as GLuint / mem::size_of::<f32>() as GLuint;

pub(crate) const VERTEX_DATA_SIZE: GLuint = mem::size_of::<VertexData>() as GLuint;