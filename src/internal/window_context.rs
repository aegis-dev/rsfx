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

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};

pub struct WindowContext {
    initialized: bool,
    window_title: String, 
    fullscreen: bool,
    window: Option<Box<Window>>,
}

impl WindowContext {
    pub fn new(window_title: &str, fullscreen: bool) -> WindowContext {
        WindowContext { 
            initialized: false, 
            window_title: String::from(window_title), 
            fullscreen,
            window: None,  
        }
    }
}

impl ApplicationHandler for WindowContext {
    
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.initialized {
            let mut window_attributes = WindowAttributes::default();
            window_attributes = window_attributes.with_title(self.window_title.as_str());
            window_attributes = window_attributes.with_resizable(false);
    
            if self.fullscreen {
                window_attributes = window_attributes.with_fullscreen(Some(Fullscreen::Borderless(None)))
            }
                                                
            self.window = Some(Box::new(event_loop.create_window(window_attributes).unwrap()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {

        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // TODO: draw here
                
                window.request_redraw();
            },
            _ => (),
        }

    }
}