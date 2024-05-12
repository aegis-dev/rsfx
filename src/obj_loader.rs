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

use gl::types::GLuint;
use glam::{Vec3, Vec2};

use crate::mesh::{Mesh, MeshData};
use crate::internal::vertex_data::VertexData;

pub fn load_obj_data(obj_data: &str) -> MeshData {
    let lines: Vec<_> = obj_data.lines().collect();
        
    let mut vertices: Vec<Vec3> = vec![];
    let mut texture_coords: Vec<Vec2> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut indices: Vec<u32> = vec![];
    
    let mut vertex_data: Vec<VertexData> = vec![];
    
    for line in &lines {
        let mut splits: Vec<_> = line.split(" ").collect();
        splits.retain(|&x| x.len() != 0);
        
        if line.starts_with("v ") {
            vertices.push(
                Vec3::new(
                    splits[1].trim().parse().unwrap(),
                    splits[2].trim().parse().unwrap(),
                    splits[3].trim().parse().unwrap()
                )
            );
        }
        else if line.starts_with("vt ") {
            texture_coords.push(
                Vec2::new(
                    splits[1].trim().parse().unwrap(),
                    splits[2].trim().parse().unwrap()
                )
            );
        }
        else if line.starts_with("vn ") {
            normals.push(
                Vec3::new(
                    splits[1].trim().parse().unwrap(),
                    splits[2].trim().parse().unwrap(),
                    splits[3].trim().parse().unwrap()
                )
            );
        }
    }
    
    for line in &lines {
        if line.starts_with("f ") {
            let mut splits: Vec<_> = line.split(" ").collect();
            splits.retain(|&x| x.len() != 0);
            
            for part in &splits {
                if part.starts_with("f") {
                    continue;
                }
                
                let mut vertex_indices: Vec<&str> = part.split('/').collect();
                vertex_indices.retain(|&x| x.len() != 0);
                let vertex_index = vertex_indices[0].parse::<i32>().unwrap() - 1;
                let texture_coord_index = vertex_indices[1].parse::<i32>().unwrap() - 1;
                let normal_index = vertex_indices[2].parse::<i32>().unwrap() - 1;
                
                let vertex = VertexData::new(
                    vertices[vertex_index as usize],
                    texture_coords[texture_coord_index as usize],
                    normals[normal_index as usize]
                );
                vertex_data.push(vertex);
                indices.push(indices.len() as GLuint);
            }
        }
    }
    
    MeshData::from_data(vertex_data, indices)
}

pub fn load_obj_mesh(obj_data: &str) -> Mesh {
    Mesh::from_mesh_data(&load_obj_data(obj_data))
}