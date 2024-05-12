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
use crate::internal::uniform_locations::{
    UNIFORM_LIGT_BRIGHTNESS_LOCATION, UNIFORM_LIGT_COLOR_LOCATION,
    UNIFORM_LIGT_DIRECTION_LOCATION, UNIFORM_FOG_MIN_LOCATION,
    UNIFORM_PROJECTION_MATRIX_LOCATION, UNIFORM_TRANSFORMATION_MATRIX_LOCATION,
    UNIFORM_VIEW_MATRIX_LOCATION, UNIFORM_FOG_MAX_LOCATION
};
use crate::matrices::{build_perspective_matrix, build_transformation_matrix, build_view_matrix};
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
                RendererCommand::SetTransformMat(pos, rot, scale) => {
                    let transform = build_transformation_matrix(pos, rot, *scale);
                    shader.set_uniform_mat4(UNIFORM_TRANSFORMATION_MATRIX_LOCATION, &transform);
                }
                RendererCommand::SetViewMat(pos, rot) => {
                    let view = build_view_matrix(pos, rot);
                    shader.set_uniform_mat4(UNIFORM_VIEW_MATRIX_LOCATION, &view);
                }
                RendererCommand::SetPerspectiveProjectionMat(fov, aspect_ratio) => {
                    let projection = build_perspective_matrix(*fov, *aspect_ratio);
                    shader.set_uniform_mat4(UNIFORM_PROJECTION_MATRIX_LOCATION, &projection);
                }
                RendererCommand::SetFogMin(value) => {
                    shader.set_uniform_float(UNIFORM_FOG_MIN_LOCATION, *value);
                }
                RendererCommand::SetFogMax(value) => {
                    shader.set_uniform_float(UNIFORM_FOG_MAX_LOCATION, *value);
                }
                RendererCommand::SetLightColor(color) => {
                    shader.set_uniform_vec3(UNIFORM_LIGT_COLOR_LOCATION, color);
                }
                RendererCommand::SetLightDirection(direction) => {
                    shader.set_uniform_vec3(UNIFORM_LIGT_DIRECTION_LOCATION, direction);
                }
                RendererCommand::SetLightBrightness(value) => {
                    shader.set_uniform_float(UNIFORM_LIGT_BRIGHTNESS_LOCATION, *value);
                }
            }
        }

        framebuffer.unbind();
    }
}