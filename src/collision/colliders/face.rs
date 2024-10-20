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

use std::ops::{Add, Mul, Sub};

use glam::Vec3;

use crate::collision::{aabb::AABB, CollisionResult};

use super::{Collidable, sphere::Sphere};

#[derive(Copy, Clone)]
pub struct Face {
    vertex_a: Vec3,
    vertex_b: Vec3,
    vertex_c: Vec3,
    pub aabb: AABB,
}

impl Face {
    pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3) -> Face {
        let min = Vec3::new(
            vertex_a.x.min(vertex_b.x.min(vertex_c.x)),
            vertex_a.y.min(vertex_b.y.min(vertex_c.y)),
            vertex_a.z.min(vertex_b.z.min(vertex_c.z))
        );
        let max = Vec3::new(
            vertex_a.x.max(vertex_b.x.max(vertex_c.x)),
            vertex_a.y.max(vertex_b.y.max(vertex_c.y)),
            vertex_a.z.max(vertex_b.z.max(vertex_c.z))
        );

        let aabb = AABB::new(min, max);

        Face { vertex_a, vertex_b, vertex_c, aabb }
    }

    fn is_point_in_triangle(&self, p: &Vec3) -> bool {
        let ab = self.vertex_b.sub(self.vertex_a);
        let ac = self.vertex_c.sub(self.vertex_a);
        let ap = p.sub(self.vertex_a);

        let dot_ab_ab = ab.dot(ab);
        let dot_ab_ac = ab.dot(ac);
        let dot_ab_ap = ab.dot(ap);
        let dot_ac_ac = ac.dot(ac);
        let dot_ac_ap = ac.dot(ap);

        let inv_denom = 1.0 / (dot_ab_ab * dot_ac_ac - dot_ab_ac * dot_ab_ac);
        let u = (dot_ac_ac * dot_ab_ap - dot_ab_ac * dot_ac_ap) * inv_denom;
        let v = (dot_ab_ab * dot_ac_ap - dot_ab_ac * dot_ab_ap) * inv_denom;

        u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
    }

    // Closest point on a line segment
    fn closest_point_on_line_segment(a: &Vec3, b: &Vec3, p: &Vec3) -> Vec3 {
        let ab = b.sub(*a);
        let ap = p.sub(*a);
        let ab_length_squared = ab.length_squared();

        // Project point p onto line ab, computing the scalar projection t
        let t = ap.dot(ab) / ab_length_squared;

        // Clamp t to [0, 1] to stay within the segment
        let t_clamped = t.clamp(0.0, 1.0);

        // Compute the projection point along the segment
        a.add(ab.mul(t_clamped))
    }
}

impl Collidable for Face {
    fn test_ray(&self, position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult {
        let edge_1 = self.vertex_b.sub(self.vertex_a);
        let edge_2 = self.vertex_c.sub(self.vertex_a);

        let pvec = direction.cross(edge_2);
        let determinant = edge_1.dot(pvec);

        if determinant.abs() < 1e-8 {
            return CollisionResult::Nothing;
        }

        let inverted_determinant = 1.0 / determinant;
        let t = position.sub(self.vertex_a);

        let u = t.dot(pvec).mul(inverted_determinant);
        if u < 0.0 || u > 1.0 {
            return CollisionResult::Nothing;
        }

        let q = t.cross(edge_1);
        let v = direction.dot(q).mul(inverted_determinant);
        if v < 0.0 || u + v > 1.0 {
            return CollisionResult::Nothing;
        }

        let t_param = edge_2.dot(q).mul(inverted_determinant);
        if t_param > 0.0 && t_param <= distance {
            let point = position.add(direction.mul(t_param));
            return CollisionResult::Collides(point);
        }

        CollisionResult::Nothing
    }

    fn test_sphere(&self, sphere: &Sphere) -> CollisionResult {
        // Find the plane normal of the triangle
        let ab = self.vertex_b.sub(self.vertex_a);
        let ac = self.vertex_c.sub(self.vertex_a);
        let normal = ab.cross(ac).normalize();

        // Project the sphere's center onto the plane
        let sphere_to_vertex_a = sphere.position.sub(self.vertex_a);
        let distance_to_plane = sphere_to_vertex_a.dot(normal);

        // Closest point on the plane
        let closest_point_on_plane = sphere.position.sub(normal.mul(distance_to_plane));

        // Check if the closest point on the plane is inside the triangle
        if self.is_point_in_triangle(&closest_point_on_plane) {
            // If the point is inside the triangle, check if the distance to the plane is less than the radius
            if distance_to_plane.abs() <= sphere.radius {
                return CollisionResult::Collides(closest_point_on_plane);
            }
            return CollisionResult::Nothing;
        }

        // If the closest point is not inside the triangle, check the edges
        let closest_to_ab = Face::closest_point_on_line_segment(&self.vertex_a, &self.vertex_b, &sphere.position);
        let closest_to_bc = Face::closest_point_on_line_segment(&self.vertex_b, &self.vertex_c, &sphere.position);
        let closest_to_ca = Face::closest_point_on_line_segment(&self.vertex_c, &self.vertex_a, &sphere.position);

        // Find the closest point on any of the triangle edges
        let mut closest_point = closest_to_ab;
        let mut closest_distance_squared = sphere.position.sub(closest_to_ab).length_squared();

        let dist_bc = sphere.position.sub(closest_to_bc).length_squared();
        if dist_bc < closest_distance_squared {
            closest_distance_squared = dist_bc;
            closest_point = closest_to_bc;
        }

        let dist_ca = sphere.position.sub(closest_to_ca).length_squared();
        if dist_ca < closest_distance_squared {
            closest_distance_squared = dist_ca;
            closest_point = closest_to_ca;
        }

        // Check if the closest point on the triangle edges is within the sphere's radius
        if closest_distance_squared <= sphere.radius * sphere.radius {
            return CollisionResult::Collides(closest_point);
        }

        CollisionResult::Nothing
    }
}