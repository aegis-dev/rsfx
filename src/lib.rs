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

pub extern crate glam;

pub mod game_status;
pub mod graphics_settings;
pub mod scene;

mod internal;

use winit::event_loop::EventLoop;

use scene::Scene;
use graphics_settings::GraphicsSettings;
use internal::window_context::{self, WindowContext};

// 60 ticks per second
const TICKS_PER_SECOND: u32 = 60;
const TICK_RATE: u128 = (1.0f64 / TICKS_PER_SECOND as f64 * 1000.0f64) as u128;

pub struct Rsfx;

impl Rsfx {
    pub fn new() -> Rsfx {
        Rsfx {}
    }

    // This func is mutable to ensure that this object is not used more than once when game is running
    pub fn run(
        &mut self,
        game_name: &str,
        graphics_settings: GraphicsSettings,
        starting_scene: Box<dyn Scene>,
    ) {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();

        let mut window_context = WindowContext::new(
            game_name, 
            false
        );

        let _ = event_loop.run_app(&mut window_context);
    }
}
