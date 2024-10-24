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

#![allow(dead_code)]

extern crate gl;
extern crate sdl2;
pub extern crate glam;

mod internal;
pub mod mesh;
pub mod texture;
pub mod game_status;
pub mod scene;
pub mod input;
pub mod renderer;
pub mod obj_loader;
pub mod matrices;
pub mod math;
pub mod collision;
pub mod vertex_data;
pub mod transform;
pub mod transform_animation;
pub mod graphics_settings;

use crate::scene::Scene;
use crate::game_status::GameStatus;
use crate::graphics_settings::{GraphicsSettings, RenderResolution};
use crate::input::Input;
use crate::internal::gl_renderer::GlRenderer;
use crate::internal::time;
use crate::internal::window_context::WindowContext;
use crate::renderer::Renderer;

// 60 ticks per second
const TICKS_PER_SECOND: u32 = 60;
const TICK_RATE: u128 = (1.0f64 / TICKS_PER_SECOND as f64 * 1000.0f64) as u128;

pub struct Rsfx;

impl Rsfx {
    pub fn new() -> Rsfx {
        Rsfx { }
    }

    // This func is mutable to ensure that this object is not used more than once when game is running
    pub fn run(&mut self, game_name: &str, graphics_settings: GraphicsSettings, starting_scene: Box<dyn Scene>) -> Result<(), String> {
        let mut window_context = WindowContext::new(game_name, graphics_settings.vsync, graphics_settings.fullscreen)?;

        let display_width = window_context.get_display_width();
        let display_height = window_context.get_display_height();

        let framebuffer_width = match graphics_settings.render_resolution {
            RenderResolution::W427h240 => 427,
            RenderResolution::W640h360 => 640,
            RenderResolution::W854h480 => 854,
            RenderResolution::Native => display_width
        };
        let framebuffer_height = match graphics_settings.render_resolution {
            RenderResolution::W427h240 => 240,
            RenderResolution::W640h360 => 360,
            RenderResolution::W854h480 => 480,
            RenderResolution::Native => display_height
        };

        let gl_renderer = GlRenderer::new(framebuffer_width, framebuffer_height, display_width, display_height);

        let mut renderer = Renderer::new(gl_renderer);
        renderer.set_clear_color(0.0, 0.0, 0.0);

        let mut input = Input::new(
            framebuffer_width,
            framebuffer_height,
            display_width,
            display_height
        );

        let mut current_scene = starting_scene;

        current_scene.on_start(&mut renderer);

        let delta_time = time::time_now();
        let mut last_frame_time = delta_time;

        let mut game_status = GameStatus::new();
        'main_loop: loop {
            window_context.poll_input_events(&mut input);

            if input.should_quit() {
                break 'main_loop;
            }

            let time_now = time::time_now();
            if time_now >= last_frame_time + TICK_RATE {
                let delta_time = time_now - last_frame_time;
                last_frame_time = time_now;

                // Update scene
                match current_scene.on_update(
                    &mut game_status,
                    &mut renderer,
                    &input,
                    delta_time as f64 / 1000.0
                ) {
                    Some(scene) => {
                        current_scene.on_destroy();
                        current_scene = scene;
                        current_scene.on_start(&mut renderer);
                    }
                    _ => {
                        if game_status.should_quit() {
                            current_scene.on_destroy();
                            break 'main_loop
                        }
                    }
                };

                input.clear_states();
            }

            current_scene.on_render(&mut renderer);

            renderer.run_passes();
            window_context.swap_buffer();
        }

        Ok(())
    }
}
