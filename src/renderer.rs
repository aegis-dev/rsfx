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

use glam::Mat4;

use crate::{internal::gl_renderer::GlRenderer, mesh::Mesh, texture::Texture};

const UNIFORM_TRANSFORMATION_MATRIX_LOCATION: i32 = 2;
const UNIFORM_PROJECTION_MATRIX_LOCATION: i32 = 6;
const UNIFORM_VIEW_MATRIX_LOCATION: i32 = 10;

pub struct Renderer {
    gl_renderer: GlRenderer,
}

impl Renderer {
    pub fn new(gl_renderer: GlRenderer) -> Renderer {
        Renderer { gl_renderer }
    }
    
    pub fn clear_screen(&self) {
        self.gl_renderer.clear_buffer();
    }
    
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32) {
        self.gl_renderer.set_clear_color(r, g, b);
    }
    
    pub fn set_transformation_matrix(&self, matrix: &Mat4) {
        self.gl_renderer.set_uniform_mat4(UNIFORM_TRANSFORMATION_MATRIX_LOCATION, matrix);
    }
    
    pub fn set_projection_matrix(&self, matrix: &Mat4) {
        self.gl_renderer.set_uniform_mat4(UNIFORM_PROJECTION_MATRIX_LOCATION, matrix);
    }
    
    pub fn set_view_matrix(&self, matrix: &Mat4) {
        self.gl_renderer.set_uniform_mat4(UNIFORM_VIEW_MATRIX_LOCATION, matrix);
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture) {
        self.gl_renderer.render(mesh, texture);
    }
    
    pub(crate) fn begin_rendering(&self) {
        self.gl_renderer.begin_rendering();
    }

    pub(crate) fn end_rendering(&self) {
        self.gl_renderer.end_rendering();
    }
}