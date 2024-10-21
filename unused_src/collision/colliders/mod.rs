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

pub mod face;
pub mod sphere;

use glam::Vec3;

use self::sphere::Sphere;

use super::CollisionResult;

pub trait Collidable {
    fn test_ray(&self, position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult;
    fn test_sphere(&self, sphere: &Sphere) -> CollisionResult;
}