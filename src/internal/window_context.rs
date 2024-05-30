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

use std::time::{SystemTime, UNIX_EPOCH};

use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::video::{GLContext, Window};

use crate::input::Input;

pub struct WindowContext {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    gl_context: GLContext,
    display_width: i32,
    display_height: i32,
}

impl WindowContext {
    pub fn new(game_name: &str, vsync: bool) -> Result<WindowContext, String> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_major_version(4);
        gl_attr.set_context_minor_version(5);

        // Hide mouse cursor
        sdl.mouse().show_cursor(false);
        sdl.mouse().set_relative_mouse_mode(true);

        // Get primary display bounds
        let current_display = video_subsystem.display_bounds(0)?;
        let display_width = current_display.width() as i32;
        let display_height = current_display.height() as i32;

        let window = video_subsystem
            .window(game_name, display_width as u32, display_height as u32)
            .opengl()
            .borderless()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Vsync
        video_subsystem.gl_set_swap_interval(vsync as i32).unwrap();

        Ok(WindowContext {
            sdl,
            video_subsystem,
            window,
            gl_context,
            display_width,
            display_height,
        })
    }

    pub fn get_display_width(&self) -> i32 {
        self.display_width
    }

    pub fn get_display_height(&self) -> i32 {
        self.display_height
    }

    pub fn poll_input_events(&mut self, input: &mut Input) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match input.process_sdl_event(&event) {
                Err(error) => println!("{}", error),
                Ok(_) => {}
            };
        }
    }

    pub fn swap_buffer(&self) {
        self.window.gl_swap_window(); 
    } 

    pub fn time_now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}

