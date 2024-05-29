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

use glam::Vec3;

use crate::matrices;

pub fn get_direction_from_euler(rotation: &Vec3) -> Vec3 {
        let x = rotation.x.to_radians().cos() * rotation.y.to_radians().cos();
        let y = rotation.x.to_radians().sin();
        let z = rotation.x.to_radians().cos() * rotation.y.to_radians().sin();
        Vec3::new(x, y, z)
}

pub fn transform_vector(vector: &Vec3, position: &Vec3, rotation: &Vec3, scale: f32) -> Vec3 {
    let transform = matrices::build_transformation_matrix(position, rotation, scale);
    transform.transform_point3(vector.clone())
}