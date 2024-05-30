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

use std::mem;
use std::ffi::c_void;
use std::ptr::null;

use gl::types::{GLuint, GLsizei, GLint, GLfloat};
use crate::internal::vertex_attributes;

use crate::vertex_data::VertexData;

pub struct MeshData {
    vertex_data: Vec<VertexData>,
}

impl MeshData {
    pub fn from_data(vertex_data: Vec<VertexData>) -> MeshData {
        MeshData { vertex_data }
    }
    
    pub fn get_vertices(&self) -> &Vec<VertexData> {
        &self.vertex_data
    }

    pub fn get_vertices_count(&self) -> GLsizei {
        self.vertex_data.len() as GLsizei
    }
}

pub struct Mesh {
    vao_id: GLuint,
    vbo_ids: Vec<GLuint>,
    vertices_count: GLsizei,
}

impl Mesh {
    pub fn from_mesh_data(mesh_data: &MeshData) -> Mesh {
        Mesh::from_raw_data(mesh_data.get_vertices())
    }
    
    pub fn from_raw_data(vertex_data: &Vec<VertexData>) -> Mesh {
        let vao_id = {
            let mut vao_ids = vec![0];
            unsafe {
                gl::GenVertexArrays(vao_ids.len() as GLsizei, vao_ids.as_mut_ptr());
            }
            vao_ids[0]
        };

        unsafe {
            gl::BindVertexArray(vao_id);
        }

        unsafe {
            for attribute_id in vertex_attributes::VERTEX_DATA_ATTRIBUTES {
                gl::EnableVertexAttribArray(*attribute_id);
            }
        }

        let mut vbo_ids: Vec<GLuint> = vec![];
        vbo_ids.push(Mesh::store_vertex_data_in_attribute_list(vertex_data));

        unsafe {
            gl::BindVertexArray(0);
        }

        let vertices_count = vertex_data.len() as GLsizei;
        Mesh { vao_id, vbo_ids, vertices_count}
    }

    fn bind_indices_buffer(indices: &Vec<GLuint>) -> GLuint {
        let vbo_id = {
            let mut vbo_ids = vec![0];
            unsafe {
                gl::GenBuffers(vbo_ids.len() as GLsizei, vbo_ids.as_mut_ptr());
            }
            vbo_ids[0]
        };

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * indices.len()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        vbo_id
    }

    fn store_data_in_attribute_list(attribute_id: GLuint, attribute_size: GLint, data: &Vec<GLfloat>) -> GLuint {
        let vbo_id = {
            let mut vbo_ids = vec![0];
            unsafe {
                gl::GenBuffers(vbo_ids.len() as GLsizei, vbo_ids.as_mut_ptr());
            }
            vbo_ids[0]
        };

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * data.len()) as isize, data.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(attribute_id, attribute_size, gl::FLOAT, gl::FALSE, 0, null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        vbo_id
    }
    
    fn store_vertex_data_in_attribute_list(vertex_data: &Vec<VertexData>) -> GLuint {
        let vbo_id = {
            let mut vbo_ids = vec![0];
            unsafe {
                gl::GenBuffers(vbo_ids.len() as GLsizei, vbo_ids.as_mut_ptr());
            }
            vbo_ids[0]
        };
        
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (vertex_attributes::VERTEX_DATA_SIZE as usize * vertex_data.len()) as isize, vertex_data.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(
                vertex_attributes::VERTEX_POSITION_ATTRIBUTE_ID,
                vertex_attributes::VERTEX_POSITION_ATTRIBUTE_SIZE_IN_FLOATS as i32,
                gl::FLOAT,
                gl::FALSE,
                vertex_attributes::VERTEX_DATA_SIZE as i32,
                vertex_attributes::VERTEX_DATA_POSITION_OFFSET as _
            );
            gl::VertexAttribPointer(
                vertex_attributes::VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_ID,
                vertex_attributes::VERTEX_TEXTURE_COORDINATE_ATTRIBUTE_SIZE_IN_FLOATS as i32,
                gl::FLOAT,
                gl::FALSE,
                vertex_attributes::VERTEX_DATA_SIZE as i32,
                vertex_attributes::VERTEX_DATA_TEXTURE_COORDINATE_OFFSET as _
            );
            gl::VertexAttribPointer(
                vertex_attributes::VERTEX_NORMAL_ATTRIBUTE_ID,
                vertex_attributes::VERTEX_NORMAL_ATTRIBUTE_SIZE_IN_FLOATS as i32,
                gl::FLOAT,
                gl::FALSE,
                vertex_attributes::VERTEX_DATA_SIZE as i32,
                vertex_attributes::VERTEX_DATA_NORMAL_OFFSET as _
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        
        vbo_id
    }

    pub fn vao_id(&self) -> GLuint {
        self.vao_id
    }

    pub fn vertices_count(&self) -> GLsizei {
        self.vertices_count
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.vbo_ids.len() as GLsizei, self.vbo_ids.as_ptr());
            let temp_vec = vec![self.vao_id];
            gl::DeleteVertexArrays(temp_vec.len() as GLsizei, temp_vec.as_ptr());
        }
    }
}
