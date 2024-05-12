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

use glam::{Mat4, Vec3, Vec4};
use crate::internal::renderable::Renderable;

#[derive(Copy, Clone, PartialEq)]
pub enum RendererCommand {
    ClearScreen(),
    SetClearColor(f32, f32, f32),
    Render(Renderable),
    SetUniformInt(i32, i32),
    SetUniformFloat(i32, f32),
    SetUniformVec3(i32, Vec3),
    SetUniformVec4(i32, Vec4),
    SetUniformMat4(i32, Mat4),
}