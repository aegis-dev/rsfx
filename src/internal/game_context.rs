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

use std::ops::ControlFlow;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, self};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId, self};

use crate::game_status::GameStatus;
use crate::graphics_settings::{GraphicsSettings, RenderResolution};
use crate::input::Input;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::internal::time;

// 60 ticks per second
const TICKS_PER_SECOND: u32 = 60;
const TICK_RATE: u128 = (1.0f64 / TICKS_PER_SECOND as f64 * 1000.0f64) as u128;

pub struct GameContext {
    initialized: bool,
    window_title: String,
    graphics_settings: GraphicsSettings,
    window: Option<Box<Window>>,
    input: Option<Input>,
    renderer: Option<Renderer>,
    current_scene: Box<dyn Scene>,
    last_frame_time: u128,
}

impl GameContext {
    pub fn new(window_title: &str, graphics_settings: GraphicsSettings, starting_scene: Box<dyn Scene>) -> GameContext {
        GameContext {
            initialized: false,
            window_title: String::from(window_title),
            graphics_settings,
            window: None,
            input: None,
            renderer: None,
            current_scene: starting_scene,
            last_frame_time: 0
        }
    }

    pub fn on_draw_request(&mut self) -> bool {
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return true,
        };

        let input = match &mut self.input {
            Some(input) => input,
            None => return true,
        };

        // The first draw request
        if self.last_frame_time == 0 {
            self.last_frame_time = time::time_now();
            self.current_scene.on_start(renderer);
        }

        let time_now = time::time_now();
        if time_now >= self.last_frame_time + TICK_RATE {
            let delta_time = time_now - self.last_frame_time;
            self.last_frame_time = time_now;

            let mut game_status = GameStatus::new();

            // Update scene
            match self.current_scene.on_update(
                &mut game_status,
                renderer,
                input,
                delta_time as f64 / 1000.0
            ) {
                Some(scene) => {
                    self.current_scene.on_destroy();
                    self.current_scene = scene;
                    self.current_scene.on_start(renderer);
                }
                _ => {
                    if game_status.should_quit() {
                        self.current_scene.on_destroy();
                        return false;
                    }
                }
            };

            input.clear_states();
        }

        self.current_scene.on_render(renderer);

        renderer.run_passes();

        true
    }
}

impl ApplicationHandler for GameContext {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.initialized {
            let mut window_attributes = WindowAttributes::default();
            window_attributes = window_attributes.with_title(self.window_title.as_str());
            window_attributes = window_attributes.with_resizable(false);
            window_attributes = window_attributes.with_decorations(false);

            if self.graphics_settings.fullscreen {
                window_attributes = window_attributes.with_fullscreen(Some(Fullscreen::Borderless(None)));
            }

            self.window = Some(Box::new(event_loop.create_window(window_attributes).unwrap()));

            let window = match self.window.as_ref() {
                Some(window) => window,
                None => return,
            };

            let window_size = window.inner_size();

            let framebuffer_width = match self.graphics_settings.render_resolution {
                RenderResolution::W427h240 => 427,
                RenderResolution::W640h360 => 640,
                RenderResolution::W854h480 => 854,
                RenderResolution::Native => window_size.width as i32
            };
            let framebuffer_height = match self.graphics_settings.render_resolution {
                RenderResolution::W427h240 => 240,
                RenderResolution::W640h360 => 360,
                RenderResolution::W854h480 => 480,
                RenderResolution::Native => window_size.height as i32
            };

            self.input = Some(Input::new(framebuffer_width, framebuffer_height, window_size.width as i32, window_size.height as i32));
            self.renderer = Some(Renderer::new());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let None = self.window {
            return
        }

        if let Some(input) = &mut self.input {
            input.process_window_event(&event);
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let should_close = !self.on_draw_request();

                if let Some(window) = &mut self.window {
                    if should_close {
                        event_loop.exit();
                        return
                    }

                    window.request_redraw();
                }
            },
            _ => (),
        }

    }
}