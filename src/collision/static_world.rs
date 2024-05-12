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

use core::panic;
use std::{collections::HashMap, ops::{Add, Mul, Sub}};

use glam::Vec3;

use crate::{matrices, mesh::MeshData};

pub struct StaticWorld {
    capacity_per_node: u32,
    top_level_nodes: HashMap<f32, HashMap<f32, HashMap<f32, Node>>>,
    test_faces: Vec<Face>,
}

impl StaticWorld {
    pub fn new(capacity_per_node: u32) -> StaticWorld {
        StaticWorld { capacity_per_node, top_level_nodes: HashMap::new(), test_faces: vec![] }
    }
    
    pub fn add_mesh(&mut self, mesh_data: &MeshData, position: &Vec3, rotation: &Vec3, scale: f32) {
        let indices = mesh_data.get_indices();
        let vertex_data = mesh_data.get_vertex_data();
        
        if indices.len() % 3 != 0 {
            panic!("Mesh indices counte is not divisable by 3!");
        }
        
        let transform = matrices::build_transformation_matrix(position, rotation, scale);
        
        for idx in (0..indices.len()).step_by(3) {
            let vertex_a = transform.transform_vector3(vertex_data[idx + 0].get_position().clone());
            let vertex_b = transform.transform_vector3(vertex_data[idx + 1].get_position().clone());
            let vertex_c = transform.transform_vector3(vertex_data[idx + 2].get_position().clone());
            
            let face = Face::new(vertex_a, vertex_b, vertex_c);
            self.add_face(face);
        }
    }
    
    pub fn test_ray(&self,  position: &Vec3, direction: &Vec3, distance: f32) -> CollisionResult {
        for face in &self.test_faces {
            if let CollisionResult::Collides(point) = face.test_ray(position, direction, distance) {
                return CollisionResult::Collides(point);
            }
        }
        
        CollisionResult::Nothing
    }
    
    fn add_face(&mut self, face: Face) {
        self.test_faces.push(face);
    }
    
//    fn add_face_for_point(face: Face, point: &Vec3) {
//        
//    }
}

struct Node {
    aabb: AABB,
    children_nodes: Vec<Node>,
    faces: Vec<Face>,
}

struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn in_bounds(&self, point: Vec3) -> bool {
        self.min.x < point.x && self.max.x > point.x &&
        self.min.y < point.y && self.max.y > point.y &&
        self.min.z < point.z && self.max.z > point.z
    }
}

struct Face {
    vertex_a: Vec3,
    vertex_b: Vec3,
    vertex_c: Vec3,
}

impl Face {
    pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3) -> Face {
        Face { vertex_a, vertex_b, vertex_c }
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
