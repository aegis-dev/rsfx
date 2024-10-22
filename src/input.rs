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

use core::f32;
use std::collections::HashMap;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use winit::{keyboard::Key, event::{WindowEvent, ElementState, KeyEvent, MouseButton}};

#[derive(Clone)]
pub struct Input {
    screen_width: i32,
    screen_height: i32,
    cursor_x: f32,
    cursor_y: f32,
    cursor_delta_x: f32,
    cursor_delta_y: f32,
    last_cursor_abs_x: f32,
    last_cursor_abs_y: f32,
    max_cursor_x: f32,
    max_cursor_y: f32,
    min_cursor_x: f32,
    min_cursor_y: f32,
    cursor_x_sensitivity: f32,
    cursor_y_sensitivity: f32,
    key_state: HashMap<Key, State>,
    mouse_button_state: HashMap<MouseButton, State>,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum State {
    Up,
    Down,
    Repeating,
}

impl Input {
    pub fn new(screen_buffer_width: i32, screen_buffer_height: i32, screen_width: i32, screen_height: i32) -> Input {
        Input {
            screen_width,
            screen_height,
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_delta_x: 0.0,
            cursor_delta_y: 0.0,
            last_cursor_abs_x: 0.0,
            last_cursor_abs_y: 0.0,
            max_cursor_x: (screen_buffer_width / 2) as f32,
            max_cursor_y: (screen_buffer_height / 2) as f32,
            min_cursor_x: (-screen_buffer_width / 2) as f32,
            min_cursor_y: (-screen_buffer_height / 2) as f32,
            cursor_x_sensitivity: screen_buffer_width as f32 / screen_width as f32,
            cursor_y_sensitivity: screen_buffer_height as f32 / screen_height as f32,
            key_state: HashMap::new(),
            mouse_button_state: HashMap::new(),
        }
    }

    pub(crate) fn process_window_event(&mut self, event:& WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                self.key_state.insert(logical_key.clone(), State::Down);
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Released,
                    ..
                },
                ..
            } => {
                self.key_state.insert(logical_key.clone(), State::Up);
            },
            WindowEvent::MouseInput { button, state: ElementState::Pressed, .. } => {
                self.mouse_button_state.insert(*button, State::Down);
            },
            WindowEvent::MouseInput { button, state: ElementState::Released, .. } => {
                self.mouse_button_state.insert(*button, State::Up);
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_x = position.x as f32;
                self.cursor_y = position.y as f32;
            }
            _ => (),
        }
    }

    pub(crate) fn clear_states(&mut self) {
        self.cursor_delta_x = 0.0;
        self.cursor_delta_y = 0.0;
    }

    pub fn get_key_state(&self, key: Key) -> State {
        match self.key_state.get(&key) {
            None => State::Up,
            Some(state) => *state
        }
    }

    pub fn get_button_state(&self, button: MouseButton) -> State {
        match self.mouse_button_state.get(&button) {
            None => State::Up,
            Some(state) => *state
        }
    }

    pub fn get_cursor_position(&self) -> (i64, i64)  {
        (self.cursor_x as i64, self.cursor_y as i64)
    }

    pub fn get_cursor_movement_delta(&self) -> (f32, f32)  {
        (self.cursor_delta_x, self.cursor_delta_y)
    }
}
