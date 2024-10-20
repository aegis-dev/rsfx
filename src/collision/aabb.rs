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

use std::hash::{Hash, Hasher};

use glam::Vec3;

#[derive(Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn in_bounds(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn ray_intersect(&self, position: &Vec3, direction: &Vec3, distance: f32) -> bool {
        let inv_dir = Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);

        let t1 = (self.min.x - position.x) * inv_dir.x;
        let t2 = (self.max.x - position.x) * inv_dir.x;
        let t3 = (self.min.y - position.y) * inv_dir.y;
        let t4 = (self.max.y - position.y) * inv_dir.y;
        let t5 = (self.min.z - position.z) * inv_dir.z;
        let t6 = (self.max.z - position.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        tmax >= tmin && tmax >= 0.0 && tmin <= distance
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        // Check for overlap on each axis
        self.min.x <= other.max.x && self.max.x >= other.min.x && // x-axis overlap
        self.min.y <= other.max.y && self.max.y >= other.min.y && // y-axis overlap
        self.min.z <= other.max.z && self.max.z >= other.min.z    // z-axis overlap
    }

    pub fn get_corners(&self) -> Vec<Vec3> {
        let mut corners = Vec::with_capacity(8); // AABB has 8 corners

        // Loop through all combinations of min/max for each axis
        for &x in &[self.min.x, self.max.x] {
            for &y in &[self.min.y, self.max.y] {
                for &z in &[self.min.z, self.max.z] {
                    corners.push(Vec3::new(x, y, z));
                }
            }
        }

        corners
    }
}

impl Hash for AABB {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Good enough but might cause issues with overlapping and very similar AABBs
        (self.min.x as i32).hash(state);
        (self.min.y as i32).hash(state);
        (self.min.z as i32).hash(state);
        (self.max.x as i32).hash(state);
        (self.max.y as i32).hash(state);
        (self.max.z as i32).hash(state);
    }
}