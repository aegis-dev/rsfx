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

pub enum RenderResolution {
    W427h240,
    W640h360,
    W854h480,
    Native
}

pub struct GraphicsSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub render_resolution: RenderResolution,
}

impl GraphicsSettings {
    pub fn new() -> GraphicsSettings {
        GraphicsSettings { vsync: false, fullscreen: true, render_resolution: RenderResolution::W854h480 }
    }
}