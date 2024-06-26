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

use gl::types::{GLsizei, GLuint};
use crate::mesh::Mesh;
use crate::texture::Texture;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Renderable {
    pub mesh_id: GLuint,
    pub vertices_count: GLsizei,
    pub texture_id: GLuint,
}

impl Renderable {
    pub fn new(mesh: &Mesh, texture: &Texture) -> Renderable {
        Renderable { mesh_id: mesh.vao_id(), vertices_count: mesh.vertices_count(), texture_id: texture.texture_id() }
    }
}