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
use crate::internal::renderable::Renderable;
use crate::internal::renderer_command::RendererCommand;
use crate::mesh::Mesh;
use crate::texture::Texture;


pub struct Renderer {
    gl_renderer: GlRenderer,
    commands: Vec<RendererCommand>,
}
impl Renderer {
    pub fn new(gl_renderer: GlRenderer) -> Renderer {
        Renderer { gl_renderer, commands: vec![] }
    }
    
    pub fn clear_screen(&mut self) {
        self.commands.push(RendererCommand::ClearScreen());
    }
    
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32) {
        self.commands.push(RendererCommand::SetClearColor(r, g, b));
    }
    
    pub fn set_transform(&mut self, translation: &Vec3, rotation: &Vec3, scale: f32) {
        self.commands.push(RendererCommand::SetTransformMat(translation.clone(), rotation.clone(), scale));
    }
    
    pub fn set_perspective_projection(&mut self, fov: f32, aspect_ratio: f32) {
        self.commands.push(RendererCommand::SetPerspectiveProjectionMat(fov, aspect_ratio));
    }
    
    pub fn set_view(&mut self, translation: &Vec3, rotation: &Vec3) {
        self.commands.push(RendererCommand::SetViewMat(translation.clone(), rotation.clone()));
    }
    
    pub fn set_fog_minimum_distance(&mut self, distance: f32) {
        let mut value = distance;
        if value < 0.0 {
            value = 0.0;
        }
        self.commands.push(RendererCommand::SetFogMin(value));
    }
    
    pub fn set_fog_maximum_distance(&mut self, distance: f32) {
        let mut value = distance;
        if value < 0.0 {
            value = 0.0;
        }
        self.commands.push(RendererCommand::SetFogMax(value));
    }
    
    pub fn set_light_color(&mut self, color: &Vec3) {
        self.commands.push(RendererCommand::SetLightColor(color.clone()));
    }
    
    pub fn set_light_direction(&mut self, direction: &Vec3) {
        self.commands.push(RendererCommand::SetLightDirection(direction.clone()));
    }
    
    pub fn set_light_brightness(&mut self, brightness: f32) {
        let value = brightness.clamp(0.0, 1.0);
        self.commands.push(RendererCommand::SetLightBrightness(value));
    }

    pub fn render(&mut self, mesh: &Mesh, texture: &Texture) {
        self.commands.push(RendererCommand::Render(Renderable::new(mesh, texture)));
    }

    pub(crate) fn run_passes(&mut self) {
        self.gl_renderer.run_passes(&self.commands);

        // And reset renderables
        self.commands.clear();
    }
}