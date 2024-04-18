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

use std::ptr::null;

use gl;
use glam::{Mat4, Vec3, Vec4};
use sdl2::video::GLContext;

use crate::internal::shader_program::ShaderProgram;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub struct GlRenderer {
    gl_context: GLContext,
    shader: ShaderProgram,
}

impl GlRenderer {
    pub fn new(gl_context: GLContext, shader: ShaderProgram) -> GlRenderer {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        GlRenderer { gl_context, shader }
    }

    pub fn clear_buffer(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn begin_rendering(&self) {
        self.shader.enable();
    }

    pub fn end_rendering(&self) {
        self.shader.disable();
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn set_uniform_int(&self, location: i32, value: i32) {
        self.shader.set_uniform_int(location, value);
    }
    
    pub fn set_uniform_vec3(&self, location: i32, value: &Vec3) {
        self.shader.set_uniform_vec3(location, value);
    }
    
    pub fn set_uniform_vec4(&self, location: i32, value: &Vec4) {
        self.shader.set_uniform_vec4(location, value);
    }

    pub fn set_uniform_mat4(&self, location: i32, value: &Mat4) {
        self.shader.set_uniform_mat4(location, value);
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture) {
        unsafe {
            gl::BindVertexArray(mesh.vao_id());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id());

            gl::DrawElements(gl::TRIANGLES, mesh.indices_count(), gl::UNSIGNED_INT, null());

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::BindVertexArray(0);
        }
    }
}