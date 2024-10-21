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

use std::ops::{Sub, Add, Mul};

use glam::Vec3;

use crate::collision::{aabb::AABB, CollisionResult};
use super::Collidable;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f32,
    pub position: Vec3,
    pub aabb: AABB,
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3) -> Sphere {
        let min = Vec3::new(position.x - radius, position.y - radius, position.z - radius);
        let max = Vec3::new(position.x + radius, position.y + radius, position.z + radius);
        let aabb = AABB::new(min, max);
        Sphere { radius, position, aabb }
    }
}

impl Collidable for Sphere {
    fn test_ray(&self, position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult {
        // Compute L, the vector from ray origin to the sphere center
        let l = self.position.sub(*position);

        // Project L onto the ray direction (using dot product)
        let t_ca = l.dot(*direction);

        // If t_ca is negative, the sphere is behind the ray
        if t_ca < 0.0 {
            return CollisionResult::Nothing;
        }

        // Compute d^2, the squared distance from the sphere center to the ray
        let d2 = l.length_squared() - t_ca * t_ca;
        let radius2 = self.radius * self.radius;

        // If d^2 > radius^2, the ray misses the sphere
        if d2 > radius2 {
            return CollisionResult::Nothing;
        }

        // Compute the distance from the point of closest approach to the intersection point
        let thc = (radius2 - d2).sqrt();

        // Calculate the two possible distances along the ray (front and back of the sphere)
        let t0 = t_ca - thc;
        let t1 = t_ca + thc;

        if t0 >= 0.0 && t0 <= distance {
            let point = position.add(direction.mul(t0));
            return CollisionResult::Collides(point);
        } else if t1 >= 0.0 && t1 <= distance {
            let point = position.add(direction.mul(t1));
            return CollisionResult::Collides(point);
        }
        CollisionResult::Nothing
    }

    fn test_sphere(&self, sphere: &Sphere) -> CollisionResult {
        let center_vector = sphere.position.sub(self.position);
        let distance_squared = center_vector.length_squared();
        let radii_sum = self.radius + sphere.radius;

        // Check if the squared distance is less than or equal to the squared sum of radii
        if distance_squared <= radii_sum * radii_sum {
            return CollisionResult::Collides(center_vector);
        }

        CollisionResult::Nothing
    }
}