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

use crate::internal::framebuffer::Framebuffer;
use crate::internal::gl_renderer::GlRenderer;
use crate::internal::render_passes::PassStep;
use crate::internal::renderer_command::RendererCommand;
use crate::internal::shader_program::ShaderProgram;
use crate::texture::Texture;

pub struct MainPass;

impl MainPass {
    pub fn new() -> MainPass {
        MainPass { }
    }
}

impl PassStep for MainPass {
    fn on_execute(&self, gl_renderer: &GlRenderer, framebuffer: &Framebuffer, shader: &ShaderProgram, commands: &Vec<RendererCommand>, _last_pass_result: &Option<&Texture>) {
        framebuffer.bind();
        shader.enable();
        gl_renderer.enable_depth_test();
        gl_renderer.set_viewport_size(0, 0, framebuffer.get_width(), framebuffer.get_height());

        for command in commands {
            match command {
                RendererCommand::ClearScreen() => {
                    gl_renderer.clear_buffer();
                }
                RendererCommand::SetClearColor(r, g, b) => {
                    gl_renderer.set_clear_color(*r, *g, *b);
                }
                RendererCommand::Render(renderable) => {
                    gl_renderer.bind_mesh(renderable.mesh_id);
                    gl_renderer.bind_texture(renderable.texture_id, gl::TEXTURE0);
                    gl_renderer.draw_elements(renderable.indices_count);
                    gl_renderer.unbind_mesh();
                }
                RendererCommand::SetUniformInt(location, value) => {
                    shader.set_uniform_int(*location, *value);
                }
                RendererCommand::SetUniformFloat(location, value) => {
                    shader.set_uniform_float(*location, *value);
                }
                RendererCommand::SetUniformVec3(location, value) => {
                    shader.set_uniform_vec3(*location, value);
                }
                RendererCommand::SetUniformVec4(location, value) => {
                    shader.set_uniform_vec4(*location, value);
                }
                RendererCommand::SetUniformMat4(location, value) => {
                    shader.set_uniform_mat4(*location, value);
                }
            }
        }

        framebuffer.unbind();
    }
}