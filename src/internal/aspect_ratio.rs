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

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AspectRatio {
    R16by9,
    WiderThan16by9,
    NarrowerThan16by9,
}

impl AspectRatio {
    pub fn from(width: i32, height: i32) -> AspectRatio {
        let ratio = width as f32 / height as f32;
        
        if ratio >= 1.77 && ratio <= 1.78 {
            // For instance, 1920x1080 ~ 1.777777778
            return AspectRatio::R16by9;
        } if ratio > 1.78 {
            // For instance, 19:10 4096x2160 ~ 1.896296296
            return AspectRatio::WiderThan16by9;
        } else {
            // For instance, 4:3 800x600 ~ 1.333333333
            return AspectRatio::NarrowerThan16by9;
        }
    }
}