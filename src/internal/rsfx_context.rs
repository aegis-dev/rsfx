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
use sdl2::video::Window;
use sdl2::event::Event;

use crate::internal::gl_renderer::GlRenderer;
use crate::internal::shader_program::ShaderProgram;
use crate::input::Input;
use crate::renderer::Renderer;

pub struct RsfxContext {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    renderer: Renderer,
    input: Input,
}

const UNIFORM_PALETTE_SIZE_LOCATION: i32 = 2;
const UNIFORM_BACKGROUND_COLOR_INDEX_LOCATION: i32 = 3;

impl RsfxContext {
    pub fn new(game_name: &str) -> Result<RsfxContext, String> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        
        gl_attr.set_context_major_version(4);
        gl_attr.set_context_minor_version(3);
        
        // Hide mouse cursor
        sdl.mouse().show_cursor(false);
        sdl.mouse().set_relative_mouse_mode(true);

        // Get primary display bounds
        let current_display = video_subsystem.display_bounds(0)?;
        let display_width = current_display.width();
        let display_height = current_display.height();

        let window = video_subsystem
            .window(game_name, display_width, display_height)
            .opengl()
            .borderless()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // disable vsync
        video_subsystem.gl_set_swap_interval(0).unwrap();

        unsafe {
            gl::Viewport(0,0, display_width as i32, display_height as i32);
        }

        // Load shaders
        let shader = {
            use std::ffi::CString;
            ShaderProgram::load_shaders(
                &CString::new(include_str!("shaders/screen_shader.vert")).unwrap(),
                &CString::new(include_str!("shaders/screen_shader.frag")).unwrap(),
            )
        };

        let renderer = Renderer::new(GlRenderer::new(gl_context, shader));
        renderer.set_clear_color(0.0, 0.0, 0.0);

        let input = Input::new(
            display_width as i32,
            display_height as i32,
            display_width as i32,
            display_height as i32
        );

        Ok(RsfxContext {
            sdl,
            video_subsystem,
            window,
            renderer,
            input
        })
    }

    pub fn poll_input_events(&mut self) -> Input {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match self.input.process_sdl_event(&event) {
                Err(error) => println!("{}", error),
                Ok(_) => {}
            };
        }

        self.input.clone()
    }

    pub fn poll_sdl_input_event(&mut self) -> Option<Event> {
        let mut event_pump = self.sdl.event_pump().unwrap();
        event_pump.poll_event()
    }

    pub fn get_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    
    pub fn begin_rendering(&self) {
        self.renderer.begin_rendering();
    }
    
    pub fn end_rendering(&self) {
        self.renderer.end_rendering();
    }
    
    pub fn swap_buffer(&self) { 
        self.window.gl_swap_window(); 
    } 

    pub fn time_now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}

