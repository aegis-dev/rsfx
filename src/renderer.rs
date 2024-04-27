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

use glam::{Mat4, Vec3};

use crate::internal::gl_renderer::GlRenderer;
use crate::mesh::Mesh;
use crate::texture::Texture;

const UNIFORM_TRANSFORMATION_MATRIX_LOCATION: i32 = 3;
const UNIFORM_PROJECTION_MATRIX_LOCATION: i32 = 7;
const UNIFORM_VIEW_MATRIX_LOCATION: i32 = 11;
const UNIFORM_FOG_MIN_LOCATION: i32 = 15;
const UNIFORM_FOG_MAX_LOCATION: i32 = 16;
const UNIFORM_DIRECTIONAL_LIGT_COLOR_LOCATION: i32 = 17;
const UNIFORM_DIRECTIONAL_LIGT_DIRECTION_LOCATION: i32 = 18;
const UNIFORM_DIRECTIONAL_LIGT_BRIGHTNESS_LOCATION: i32 = 19;

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
    
    pub fn set_fog_minimum_distance(&self, distance: f32) {
        let mut value = distance;
        if value < 0.0 {
            value = 0.0;
        }
        self.gl_renderer.set_uniform_float(UNIFORM_FOG_MIN_LOCATION, value);
    }
    
    pub fn set_fog_maximum_distance(&self, distance: f32) {
        let mut value = distance;
        if value < 0.0 {
            value = 0.0;
        }
        self.gl_renderer.set_uniform_float(UNIFORM_FOG_MAX_LOCATION, value);
    }
    
    pub fn set_direction_light_color(&self, color: &Vec3) {
        self.gl_renderer.set_uniform_vec3(UNIFORM_DIRECTIONAL_LIGT_COLOR_LOCATION, color);
    }
    
    pub fn set_direction_light_direction(&self, direction: &Vec3) {
        self.gl_renderer.set_uniform_vec3(UNIFORM_DIRECTIONAL_LIGT_DIRECTION_LOCATION, direction);
    }
    
    pub fn set_direction_light_brightness(&self, brightness: f32) {
        let value = brightness.clamp(0.0, 1.0);
        self.gl_renderer.set_uniform_float(UNIFORM_DIRECTIONAL_LIGT_BRIGHTNESS_LOCATION, value);
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture) {
        self.gl_renderer.render(mesh, texture);
    }
    
    pub(crate) fn begin_rendering(&self) {
        self.gl_renderer.begin_rendering();
    }

    pub(crate) fn render_framebuffer(&self) {
        self.gl_renderer.render_framebuffer();
    }
}