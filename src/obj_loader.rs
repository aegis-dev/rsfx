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

use std::str::FromStr;
use glam::{Vec3, Vec2};

use crate::mesh::{Mesh, MeshData};
use crate::vertex_data::VertexData;

pub fn load_obj_data(obj_data: &str) -> MeshData {
    let lines: Vec<_> = obj_data.lines().collect();

    let mut vertices: Vec<Vec3> = vec![];
    let mut texture_coords: Vec<Vec2> = vec![];
    let mut normals: Vec<Vec3> = vec![];

    for line in &lines {
        let mut splits: Vec<_> = line.split_whitespace().collect();
        splits.retain(|&x| x.len() != 0);

        if line.starts_with("v ") {
            vertices.push(
                Vec3::new(
                    f32::from_str(splits[1].trim()).unwrap(),
                    f32::from_str(splits[2].trim()).unwrap(),
                    f32::from_str(splits[3].trim()).unwrap()
                )
            );
        }
        else if line.starts_with("vt ") {
            texture_coords.push(
                Vec2::new(
                    f32::from_str(splits[1].trim()).unwrap(),
                    f32::from_str(splits[2].trim()).unwrap()
                )
            );
        }
        else if line.starts_with("vn ") {
            normals.push(
                Vec3::new(
                    f32::from_str(splits[1].trim()).unwrap(),
                    f32::from_str(splits[2].trim()).unwrap(),
                    f32::from_str(splits[3].trim()).unwrap()
                )
            );
        }
    }

    let mut vertex_data: Vec<VertexData> = vec![];

    for line in &lines {
        if line.starts_with("f ") {
            let mut splits: Vec<_> = line.split_whitespace().collect();
            splits.retain(|&x| x.len() != 0);
            splits.remove(0);

            let mut faces = vec![[0, 1, 2]];

            // triangulate face that has more than 3 vertices
            if splits.len() >= 4 {
                for i in 4..=splits.len() {
                    faces.push([0, i - 2, i - 1]);
                }
            }

            for face in faces {
                for vert_idx in face {
                    let vertex_info = splits[vert_idx];

                    let mut vertex_indices: Vec<&str> = vertex_info.split('/').collect();
                    vertex_indices.retain(|&x| x.len() != 0);

                    let vertex_index = i32::from_str(vertex_indices[0]).unwrap() - 1;
                    let texture_coord_index = i32::from_str(vertex_indices[1]).unwrap() - 1;
                    let normal_index = i32::from_str(vertex_indices[2]).unwrap() - 1;

                    let vertex = VertexData::new(
                        vertices[vertex_index as usize],
                        texture_coords[texture_coord_index as usize],
                        normals[normal_index as usize]
                    );
                    vertex_data.push(vertex);
                }
            }
        }
    }

    MeshData::from_data(vertex_data)
}

pub fn load_obj_mesh(obj_data: &str) -> Mesh {
    Mesh::from_mesh_data(&load_obj_data(obj_data))
}