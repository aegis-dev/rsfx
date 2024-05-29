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
use std::ffi::CString;

use gl;
use gl::types::{GLenum, GLsizei, GLuint};
use glam::{Vec3, Vec2};
use crate::internal::render_passes::main_pass::MainPass;
use crate::internal::render_passes::RenderPass;
use crate::internal::renderer_command::RendererCommand;

use crate::internal::shader_program::ShaderProgram;
use crate::mesh::Mesh;
use crate::texture::Texture;
use super::aspect_ratio::AspectRatio;
use super::framebuffer::Framebuffer;
use crate::vertex_data::VertexData;

pub struct GlRenderer {
    render_passes: Vec<RenderPass>,
    screen_quad: Mesh,
    screen_shader: ShaderProgram,
    window_width: i32,
    window_height: i32,
    window_aspect_ratio: AspectRatio,
    framebuffer_width: i32,
    framebuffer_height: i32,
}

impl GlRenderer {
    pub fn new(framebuffer_width: i32, framebuffer_height: i32, window_width: i32, window_height: i32) -> GlRenderer {
        unsafe {
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut render_passes = vec![];

        // Main renderer
        {
            let framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
            let frambuffer_aspect_ratio = AspectRatio::from(framebuffer_width, framebuffer_height);
            if frambuffer_aspect_ratio != AspectRatio::R16by9 {
                panic!("Unexpected framebuffer aspect ratio! Expecting framebuffer resolution to be 16:9");
            }

            let shader = {
                ShaderProgram::load_shaders(
                    &CString::new(include_str!("shaders/main_pass_shader.vert")).unwrap(),
                    &CString::new(include_str!("shaders/main_pass_shader.frag")).unwrap(),
                )
            };

            let pass_steps = Box::new(MainPass::new());

            let render_pass = RenderPass::new(framebuffer, shader, pass_steps);
            render_passes.push(render_pass);
        }

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
        let screen_quad = Mesh::from_raw_data(&quad_data);
        
        let window_aspect_ratio = AspectRatio::from(window_width, window_height);

        GlRenderer {
            render_passes,
            screen_quad,
            screen_shader,
            window_width,
            window_height,
            window_aspect_ratio,
            framebuffer_width,
            framebuffer_height
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

    pub fn run_passes(&self, commands: &Vec<RendererCommand>) {
        let mut last_pass_result = None;

        for render_pass in &self.render_passes {
            render_pass.on_execute(&self, commands, &last_pass_result);
            last_pass_result = Some(render_pass.get_pass_result());
        }

        self.screen_shader.enable();
        self.disable_depth_test();
        self.set_framebuffer_viewport_for_window();
        self.render_mesh_with_one_textures(&self.screen_quad, last_pass_result.unwrap());
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32) {
        let real_r = (r % 255.0) / 255.0;
        let real_g = (g % 255.0) / 255.0;
        let real_b = (b % 255.0) / 255.0;
        
        unsafe {
            gl::ClearColor(real_r, real_g, real_b, 1.0);
        }
    }

    pub fn bind_mesh(&self, mesh_id: GLuint) {
        unsafe {
            gl::BindVertexArray(mesh_id);
        }
    }

    pub fn unbind_mesh(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn bind_texture(&self, texture_id: GLuint, texture_slot: GLenum) {
        unsafe {
            gl::ActiveTexture(texture_slot);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }

    pub fn draw_arrays(&self, indices_count: GLsizei) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, indices_count);
        }
    }

    pub fn render_mesh_with_one_textures(&self, mesh: &Mesh, texture: &Texture) {
        self.bind_mesh(mesh.vao_id());
        self.bind_texture(texture.texture_id(), gl::TEXTURE0);
        self.draw_arrays(mesh.vertices_count());
        self.unbind_mesh();
    }

    pub fn render_mesh_with_two_textures(&self, mesh: &Mesh, texture_1: &Texture, texture_2: &Texture) {
        self.bind_mesh(mesh.vao_id());
        self.bind_texture(texture_1.texture_id(), gl::TEXTURE0);
        self.bind_texture(texture_2.texture_id(), gl::TEXTURE1);
        self.draw_arrays(mesh.vertices_count());
        self.unbind_mesh();
    }

    pub fn set_viewport_size(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
    
    pub fn set_framebuffer_viewport_for_window(&self) {
        let width = self.framebuffer_width;
        let height = self.framebuffer_height;
        
        if self.window_aspect_ratio == AspectRatio::R16by9 {
            self.set_viewport_size(0, 0, self.window_width, self.window_height);
        } else if self.window_aspect_ratio == AspectRatio::WiderThan16by9 {
            let min_height = cmp::min(self.window_height, height);
            let max_height = cmp::max(self.window_height, height);
            
            let modifier = max_height as f32 / min_height as f32;
            let viewport_width = (width as f32 * modifier) as i32;
            let viewport_height = self.window_height;
            let offset_x = (self.window_width - viewport_width) / 2;
            
            self.set_viewport_size(offset_x, 0, viewport_width, viewport_height);
        } else {
            let min_width = cmp::min(self.window_width, width);
            let max_width = cmp::max(self.window_width, width);
            
            let modifier = max_width as f32 / min_width as f32;
            let viewport_width = self.window_width;
            let viewport_height = (height as f32 * modifier) as i32;
            let offset_y = (self.window_height - viewport_height) / 2;
            
            self.set_viewport_size(0, offset_y, viewport_width, viewport_height);
        }
    }
}
