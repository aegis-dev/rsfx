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


const UNIFORM_TRANSFORMATION_MATRIX_LOCATION: i32 = 3;
const UNIFORM_PROJECTION_MATRIX_LOCATION: i32 = 7;
const UNIFORM_VIEW_MATRIX_LOCATION: i32 = 11;
const UNIFORM_FOG_MIN_LOCATION: i32 = 15;
const UNIFORM_FOG_MAX_LOCATION: i32 = 16;
const UNIFORM_DIRECTIONAL_LIGT_COLOR_LOCATION: i32 = 17;
const UNIFORM_DIRECTIONAL_LIGT_DIRECTION_LOCATION: i32 = 18;
const UNIFORM_DIRECTIONAL_LIGT_BRIGHTNESS_LOCATION: i32 = 19;

pub struct Renderer {
    
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {  }
    }

    pub fn clear_screen(&mut self) {
        //self.commands.push(RendererCommand::ClearScreen());
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32) {
        //self.commands.push(RendererCommand::SetClearColor(r, g, b));
    }

    pub fn set_transformation_matrix(&mut self, matrix: &Mat4) {
        //self.commands.push(RendererCommand::SetUniformMat4(UNIFORM_TRANSFORMATION_MATRIX_LOCATION, matrix.clone()));
    }

    pub fn set_projection_matrix(&mut self, matrix: &Mat4) {
       // self.commands.push(RendererCommand::SetUniformMat4(UNIFORM_PROJECTION_MATRIX_LOCATION, matrix.clone()));
    }

    pub fn set_view_matrix(&mut self, matrix: &Mat4) {
        //self.commands.push(RendererCommand::SetUniformMat4(UNIFORM_VIEW_MATRIX_LOCATION, matrix.clone()));
    }

    pub fn set_fog_minimum_distance(&mut self, distance: f32) {
//        let mut value = distance;
//        if value < 0.0 {
//            value = 0.0;
//        }
//        self.commands.push(RendererCommand::SetUniformFloat(UNIFORM_FOG_MIN_LOCATION, value));
    }

    pub fn set_fog_maximum_distance(&mut self, distance: f32) {
//        let mut value = distance;
//        if value < 0.0 {
//            value = 0.0;
//        }
//        self.commands.push(RendererCommand::SetUniformFloat(UNIFORM_FOG_MAX_LOCATION, value));
    }

    pub fn set_direction_light_color(&mut self, color: &Vec3) {
        //self.commands.push(RendererCommand::SetUniformVec3(UNIFORM_DIRECTIONAL_LIGT_COLOR_LOCATION, color.clone()));
    }

    pub fn set_direction_light_direction(&mut self, direction: &Vec3) {
       // self.commands.push(RendererCommand::SetUniformVec3(UNIFORM_DIRECTIONAL_LIGT_DIRECTION_LOCATION, direction.clone()));
    }

    pub fn set_direction_light_brightness(&mut self, brightness: f32) {
        let value = brightness.clamp(0.0, 1.0);
       // self.commands.push(RendererCommand::SetUniformFloat(UNIFORM_DIRECTIONAL_LIGT_BRIGHTNESS_LOCATION, value));
    }

//    pub fn render(&mut self, mesh: &Mesh, texture: &Texture) {
//      //  self.commands.push(RendererCommand::Render(Renderable::new(mesh, texture)));
//    }

    pub(crate) fn run_passes(&mut self) {
      //  self.gl_renderer.run_passes(&self.commands);

        // And reset renderables
       // self.commands.clear();
    }
}