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

use std::cmp;
use std::ptr::null;
use std::ffi::CString;

use gl;
use glam::{Mat4, Vec3, Vec4, Vec2};
use sdl2::video::GLContext;

use crate::internal::shader_program::ShaderProgram;
use crate::mesh::Mesh;
use crate::texture::Texture;
use super::aspect_ratio::AspectRatio;
use super::framebuffer::Framebuffer;
use super::vertex_data::{self, VertexData};

pub struct GlRenderer {
    gl_context: GLContext,
    framebuffer_shader: ShaderProgram,
    framebuffer: Framebuffer,
    framebuffer_quad: Mesh,
    screen_shader: ShaderProgram,
    window_width: i32,
    window_height: i32,
    window_aspect_ratio: AspectRatio,
}

impl GlRenderer {
    pub fn new(gl_context: GLContext, framebuffer_shader: ShaderProgram, framebuffer_width: i32, framebuffer_height: i32, window_width: i32, window_height: i32) -> GlRenderer {
        unsafe {
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        
        let framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
        let screen_shader = {
            ShaderProgram::load_shaders(
                &CString::new(include_str!("shaders/screen_shader.vert")).unwrap(),
                &CString::new(include_str!("shaders/screen_shader.frag")).unwrap(),
            )
        };
        
        let quad_data: Vec<VertexData> = vec![
            VertexData::new(Vec3::new(-1.0, -1.0, 0.0), Vec2::new(0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            VertexData::new(Vec3::new(-1.0,  1.0, 0.0), Vec2::new(0.0, 1.0), Vec3::new(0.0, 0.0, 0.0)),
            VertexData::new(Vec3::new( 1.0,  1.0, 0.0), Vec2::new(1.0, 1.0), Vec3::new(0.0, 0.0, 0.0)),
            VertexData::new(Vec3::new( 1.0, -1.0, 0.0), Vec2::new(1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            VertexData::new(Vec3::new(-1.0, -1.0, 0.0), Vec2::new(0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            VertexData::new(Vec3::new( 1.0,  1.0, 0.0), Vec2::new(1.0, 1.0), Vec3::new(0.0, 0.0, 0.0))
        ];
        let indices = vec![0, 1, 2, 3, 4, 5];
        let framebuffer_quad = Mesh::from_data(&quad_data, &indices);
        
        let window_aspect_ratio = AspectRatio::from(window_width, window_height);
        let frambuffer_aspect_ratio = AspectRatio::from(framebuffer_width, framebuffer_height);
        
        if frambuffer_aspect_ratio != AspectRatio::R16by9 {
            panic!("Unexpected framebuffer aspect ratio! Expecting framebuffer resolution to be 16:9");
        }

        GlRenderer {
            gl_context,
            framebuffer_shader,
            framebuffer,
            framebuffer_quad,
            screen_shader,
            window_width,
            window_height,
            window_aspect_ratio
        }
    }

    pub fn clear_buffer(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    
    pub fn enable_depth_test(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }
    
    pub fn disable_depth_test(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    pub fn begin_rendering(&self) {
        self.framebuffer.bind();
        self.framebuffer_shader.enable();
        self.enable_depth_test();
        self.set_viewposrt_size(0, 0, self.framebuffer.get_width(), self.framebuffer.get_height());
    }
    
    pub fn render_framebuffer(&self) {
        self.framebuffer.unbind();
        self.screen_shader.enable();
        self.disable_depth_test();
        self.set_framebuffer_viewport_for_window();
        self.render(&self.framebuffer_quad, self.framebuffer.get_texture());
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32) {
        let real_r = (r % 255.0) / 255.0;
        let real_g = (g % 255.0) / 255.0;
        let real_b = (b % 255.0) / 255.0;
        
        unsafe {
            gl::ClearColor(real_r, real_g, real_b, 1.0);
        }
    }

    pub fn set_uniform_int(&self, location: i32, value: i32) {
        self.framebuffer_shader.set_uniform_int(location, value);
    }
    
    pub fn set_uniform_vec3(&self, location: i32, value: &Vec3) {
        self.framebuffer_shader.set_uniform_vec3(location, value);
    }
    
    pub fn set_uniform_vec4(&self, location: i32, value: &Vec4) {
        self.framebuffer_shader.set_uniform_vec4(location, value);
    }

    pub fn set_uniform_mat4(&self, location: i32, value: &Mat4) {
        self.framebuffer_shader.set_uniform_mat4(location, value);
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture) {
        unsafe {
            gl::BindVertexArray(mesh.vao_id());
            
            for attribute_id in vertex_data::VERTEX_DATA_ATTRIBUTES {
                gl::EnableVertexAttribArray(*attribute_id);
            }

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id());

            gl::DrawElements(gl::TRIANGLES, mesh.indices_count(), gl::UNSIGNED_INT, null());

            for attribute_id in vertex_data::VERTEX_DATA_ATTRIBUTES {
                gl::DisableVertexAttribArray(*attribute_id);
            }
            
            gl::BindVertexArray(0);
        }
    }
    
    pub fn set_viewposrt_size(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
    
    pub fn set_framebuffer_viewport_for_window(&self) {
        let width = self.framebuffer.get_width();
        let height = self.framebuffer.get_height();
        
        if self.window_aspect_ratio == AspectRatio::R16by9 {
            self.set_viewposrt_size(0, 0, self.window_width, self.window_height);
        } else if self.window_aspect_ratio == AspectRatio::WiderThan16by9 {
            let min_height = cmp::min(self.window_height, height);
            let max_height = cmp::max(self.window_height, height);
            
            let modifier = max_height as f32 / min_height as f32;
            let viewport_width = (width as f32 * modifier) as i32;
            let viewport_height = self.window_height;
            let offset_x = (self.window_width - viewport_width) / 2;
            
            self.set_viewposrt_size(offset_x, 0, viewport_width, viewport_height);
        } else {
            let min_width = cmp::min(self.window_width, width);
            let max_width = cmp::max(self.window_width, width);
            
            let modifier = max_width as f32 / min_width as f32;
            let viewport_width = self.window_width;
            let viewport_height = (height as f32 * modifier) as i32;
            let offset_y = (self.window_height - viewport_height) / 2;
            
            self.set_viewposrt_size(0, offset_y, viewport_width, viewport_height);
        }
    }
}
