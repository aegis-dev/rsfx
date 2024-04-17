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

mod shaders;
mod mesh;
mod texture;
mod gl_renderer;
mod byte_buffer_reader;
pub mod game_status;
pub mod scene;
pub mod color;
pub mod input;
pub mod renderer;
pub mod rsfx_context;

use crate::scene::Scene;
use crate::game_status::GameStatus;
use crate::rsfx_context::RsfxContext;

pub struct Rsfx;

impl Rsfx {
    const TICK_RATE: u128 = (1.0f64 / 60.0f64 * 1000.0f64) as u128;

    pub fn new() -> Rsfx {
        Rsfx { }
    }

    // This func is mutable to ensure that this object is not used more than once when game is running
    pub fn run(&mut self, game_name: &str, starting_scene: Box<dyn Scene>) -> Result<(), String> {
        let mut rsfx_context = RsfxContext::new(game_name)?;

        let mut current_scene = starting_scene;

        current_scene.on_start(&mut rsfx_context.get_renderer_mut());

        let delta_time = RsfxContext::time_now();
        let mut last_frame_time = delta_time;

        let mut game_status = GameStatus::new();
        'main_loop: loop {
            let input = rsfx_context.poll_input_events();
            if input.should_quit() {
                break 'main_loop;
            }
            
            rsfx_context.begin_rendering();

            let time_now = RsfxContext::time_now();
            if time_now >= last_frame_time + Rsfx::TICK_RATE {
                let delta_time = time_now - last_frame_time;
                last_frame_time = time_now;

                // Update scene
                match current_scene.on_update(
                    &mut game_status,
                    rsfx_context.get_renderer_mut(),
                    &input,
                    delta_time as f64 / 1000.0
                ) {
                    Some(scene) => {
                        current_scene.on_destroy();
                        current_scene = scene;

                        let renderer = rsfx_context.get_renderer_mut();

                        // TODO: Reset renderer if needed

                        current_scene.on_start(renderer);
                    }
                    _ => {
                        if game_status.should_quit() {
                            break 'main_loop
                        }
                    }
                };
            }
            
            rsfx_context.end_rendering();
            rsfx_context.swap_buffer();
        }

        Ok(())
    }
}
