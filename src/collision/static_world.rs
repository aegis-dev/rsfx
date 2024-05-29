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

use std::{collections::HashMap, ops::{Add, Mul, Sub}};
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

use glam::Vec3;
use num_traits::Zero;

use crate::{matrices, mesh::MeshData};

pub enum NodeSize {
    Size16 = 16,
    Size32 = 32,
    Size64 = 64,
    Size128 = 128,
    Size256 = 256,
    Size512 = 512,
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

        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);
        let min_y = start_y.min(end_y);
        let max_y = start_y.max(end_y);
        let min_z = start_z.min(end_z);
        let max_z = start_z.max(end_z);

        for x in (min_x..=max_x).step_by((max_x - min_x).max(1) as usize) {
            for y in (min_y..=max_y).step_by((max_y - min_y).max(1) as usize) {
                for z in (min_z..=max_z).step_by((max_z - min_z).max(1) as usize) {
                    if let Some(x_dim) = self.top_level_nodes.get(&x) {
                        if let Some(y_dim) = x_dim.get(&y) {
                            if let Some(node) = y_dim.get(&z) {
                                if !node.aabb.ray_intersect(position, &direction_normalized, distance) {
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
                }
            }
        }

        // for (_, x_level) in &self.top_level_nodes {
        //     for (_, y_level) in x_level {
        //         for (_, node) in y_level {
        //             if !node.aabb.ray_intersect(position, &direction_normalized, distance) {
        //                 continue;
        //             }
        //             for face in &node.faces {
        //                 if !face.aabb.ray_intersect(position, &direction_normalized, distance) {
        //                     continue;
        //                 }
        //                 if let CollisionResult::Collides(point) = face.test_ray(position, &direction_normalized, distance) {
        //                     if let Some(old_point) = collision_point {
        //                         let old_dist = position.distance(old_point);
        //                         let new_dist = position.distance(point);
        //                         if new_dist < old_dist {
        //                             collision_point = Some(point);
        //                         }
        //                     } else {
        //                         collision_point = Some(point);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        if let Some(point) = collision_point {
            return CollisionResult::Collides(point);
        }

        CollisionResult::Nothing
    }
    
    fn add_face(&mut self, face: Face) {
        let mut nodes_added = HashSet::new();
        {
            let corner = Vec3::new(face.aabb.min.x, face.aabb.min.y, face.aabb.min.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.max.x, face.aabb.min.y, face.aabb.min.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.min.x, face.aabb.max.y, face.aabb.min.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.max.x, face.aabb.max.y, face.aabb.min.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.min.x, face.aabb.min.y, face.aabb.max.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.max.x, face.aabb.min.y, face.aabb.max.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.min.x, face.aabb.max.y, face.aabb.max.z);
            self.add_face_for_point_into_top_level_node(&face, &corner, &mut nodes_added);
        }
        {
            let corner = Vec3::new(face.aabb.max.x, face.aabb.max.y, face.aabb.max.z);
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

#[derive(Copy, Clone, PartialEq)]
struct AABB {
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

    // THIS IS WRONG
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

#[derive(Copy, Clone)]
struct Face {
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

    pub fn test_ray(&self, position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult {
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
}

#[derive(Copy, Clone, PartialEq)]
pub enum CollisionResult {
    Nothing,
    Collides(Vec3)
}
