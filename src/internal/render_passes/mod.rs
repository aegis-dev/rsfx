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

pub(crate) mod main_pass;
pub(crate) mod depth_map_pass;

use crate::internal::framebuffer::Framebuffer;
use crate::internal::gl_renderer::GlRenderer;
use crate::internal::renderer_command::RendererCommand;
use crate::internal::shader_program::ShaderProgram;
use crate::texture::Texture;

pub trait PassStep {
    fn on_execute(&self, gl_renderer: &GlRenderer, framebuffer: &Framebuffer, shader: &ShaderProgram, commands: &Vec<RendererCommand>, last_pass_result: &Option<&Texture>);
}

pub struct RenderPass {
    framebuffer: Framebuffer,
    shader: ShaderProgram,
    pass_step: Box<dyn PassStep>,
}

impl RenderPass {
    pub fn new(framebuffer: Framebuffer, shader: ShaderProgram, pass_step: Box<dyn PassStep>) -> RenderPass {
        RenderPass { framebuffer, shader, pass_step }
    }

    pub fn on_execute(&self, gl_renderer: &GlRenderer, commands: &Vec<RendererCommand>, last_pass_result: &Option<&Texture>) {
        self.pass_step.on_execute(gl_renderer, &self.framebuffer, &self.shader, commands, last_pass_result);
    }

    pub fn get_pass_result(&self) -> &Texture {
        &self.framebuffer.get_texture()
    }
}

