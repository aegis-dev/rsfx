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

use std::{collections::HashMap, ops::{Add, Mul}};
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

use glam::Vec3;
use num_traits::Zero;

use crate::{matrices, mesh::MeshData};

use super::{aabb::AABB, colliders::{face::Face, Collidable}, CollisionResult};

pub enum NodeSize {
    Size16 = 16,
    Size32 = 32,
    Size64 = 64,
    Size128 = 128,
    Size256 = 256,
    Size512 = 512,
    Size1024 = 1024,
    Size4096 = 4096,
    Size8192 = 8192,
}

pub struct StaticWorld {
    node_size: i32,
    top_level_nodes: HashMap<i32, HashMap<i32, HashMap<i32, Node>>>,
}

impl StaticWorld {
    pub fn new(node_size: NodeSize) -> StaticWorld {
        StaticWorld { node_size: node_size as i32, top_level_nodes: HashMap::new() }
    }

    pub fn add_mesh(&mut self, mesh_data: &MeshData, position: &Vec3, rotation: &Vec3, scale: f32) -> Result<(), &'static str> {
        let vertices = mesh_data.get_vertices();
        let vertices_count = mesh_data.get_vertices_count() as usize;

        if vertices_count % 3 != 0 {
            return Err("Mesh vertices count is not divisable by 3!")
        }

        let transform = matrices::build_transformation_matrix(position, rotation, scale);

        for idx in (0..vertices_count).step_by(3) {
            let vertex_a = transform.transform_point3(vertices[idx + 0].get_position().clone());
            let vertex_b = transform.transform_point3(vertices[idx + 1].get_position().clone());
            let vertex_c = transform.transform_point3(vertices[idx + 2].get_position().clone());

            let face = Face::new(vertex_a, vertex_b, vertex_c);

            let face_size = face.aabb.max - face.aabb.min;
            let max_face_dimension = face_size.x.max(face_size.y.max(face_size.z));
            if max_face_dimension > self.node_size as f32 {
                return Err("Mesh face dimensions are bigger than node's dimensions")
            }

            self.add_face(face);
        }

        Ok(())
    }

    pub fn test_ray(&self, position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult {
        // We can't have zeros in direction as the elements are used for division.
        let direction_normalized = self.normalize_direction_vector(direction);

        let mut collision_point: Option<Vec3> = None;

        let (start_x, start_y, start_z) = self.point_to_top_level_node_coord(position);
        let (end_x, end_y, end_z) = self.point_to_top_level_node_coord(&position.add(direction_normalized.mul(distance)));

        let min_x = start_x.min(end_x) as f32;
        let max_x = start_x.max(end_x) as f32;
        let min_y = start_y.min(end_y) as f32;
        let max_y = start_y.max(end_y) as f32;
        let min_z = start_z.min(end_z) as f32;
        let max_z = start_z.max(end_z) as f32;

        let ray_aabb = AABB::new(Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z));

        for (_, x_level) in &self.top_level_nodes {
            for (_, y_level) in x_level {
                for (_, node) in y_level {
                    if !node.aabb.intersects(&ray_aabb) {
                        continue;
                    }

                    for face in &node.faces {
                        if let CollisionResult::Collides(point) = face.test_ray(position, &direction_normalized, distance) {
                            if let Some(old_point) = collision_point {
                                let old_dist = position.distance(old_point);
                                let new_dist = position.distance(point);
                                if new_dist < old_dist {
                                    collision_point = Some(point);
                                }
                            } else {
                                collision_point = Some(point);
                            }
                        }
                    }
                }
            }
        }

        if let Some(point) = collision_point {
            return CollisionResult::Collides(point);
        }

        CollisionResult::Nothing
    }

    fn add_face(&mut self, face: Face) {
        let mut nodes_added = HashSet::new();
        for corner in face.aabb.get_corners() {
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
    }

    fn add_face_for_point_into_top_level_node(&mut self, face: &Face, point: &Vec3, nodes_added: &mut HashSet<u64>) {
        let (x, y, z) = self.point_to_top_level_node_coord(point);

        let node = self.top_level_nodes.entry(x).or_default().entry(y).or_default().entry(z)
            .or_insert_with(
                || Node::new(
                    Vec3::new(x as f32, y as f32, z as f32),
                    Vec3::new(x as f32 + self.node_size as f32, y as f32 + self.node_size as f32, z as f32 + self.node_size as f32)
                )
            );

        let node_hash = node.get_hash();
        if nodes_added.contains(&node_hash) {
            return;
        }
        nodes_added.insert(node_hash);

        node.add_face(face.clone());
    }

    // We convert zeros into the smallest positive f32 number
    fn normalize_direction_vector(&self, vector: &Vec3) -> Vec3 {
        let mut normalized = vector.clone();

        if normalized.x.is_zero() {
            normalized.x = f32::MIN_POSITIVE;
        }
        if normalized.y.is_zero() {
            normalized.y = f32::MIN_POSITIVE;
        }
        if normalized.z.is_zero() {
            normalized.z = f32::MIN_POSITIVE;
        }
        normalized
    }

    #[inline(always)]
    fn point_to_top_level_node_coord(&self, point: &Vec3) -> (i32, i32, i32) {
        let x = point.x as i32 & !(self.node_size - 1);
        let y = point.y as i32 & !(self.node_size - 1);
        let z = point.z as i32 & !(self.node_size - 1);
        (x, y, z)
    }
}

#[derive(Clone)]
struct Node {
    aabb: AABB,
    children_nodes: Vec<Node>,
    faces: Vec<Face>,
}

impl Node {
    pub fn new(min: Vec3, max: Vec3) -> Node {
        let aabb = AABB::new(min, max);
        Node { aabb, children_nodes: vec![], faces: vec![] }
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.aabb.hash(state)
    }
}



