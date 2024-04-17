//
// Copyright © 2020-2024  Egidijus Lileika
//
// This file is part of RSFX - Game framework for PSX-feel games written in Rust
//
// RSFX is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
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

use crate::input::Input;
use crate::renderer::Renderer;
use crate::game_status::GameStatus;

pub trait Scene {
    // It is useful to pass renderer on_start to do basic setup such as background color or set palette
    fn on_start(&mut self, renderer: &mut Renderer);

    fn on_update(&mut self, game_status: &mut GameStatus, renderer: &mut Renderer, input: &Input, delta_time: f64) -> Option<Box<dyn Scene>>;

    fn on_destroy(&mut self);
}