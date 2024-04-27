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

use std::ptr::null;

use gl::types::{GLuint, GLint};

use crate::texture::Texture;

pub struct Framebuffer {
    width: i32,
    height: i32,
    framebuffer_object: GLuint,
    depth_buffer: GLuint,
    texture: Texture,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
        unsafe {
            let framebuffer_object = {
                let mut fbos = vec![0];
                gl::GenFramebuffers(fbos.len() as gl::types::GLsizei, fbos.as_mut_ptr());
                fbos[0]
            };
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_object);
            
            let texture = {
                let mut texture_ids = vec![0];
                gl::GenTextures(texture_ids.len() as gl::types::GLsizei, texture_ids.as_mut_ptr());
                texture_ids[0]
            };
            
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_INT, null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);
            
            let depth_buffer = {
                let mut rbos = vec![0];
                gl::GenRenderbuffers(rbos.len() as gl::types::GLsizei, rbos.as_mut_ptr());
                rbos[0]
            };
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
            
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, depth_buffer);
            
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Failed to create frame buffer!");
            }
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);  
            
            Framebuffer {
                width,
                height,
                framebuffer_object,
                texture: Texture::new(texture, width as GLuint, height as GLuint),
                depth_buffer
            }
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_object);   
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);   
        }
    }
    
    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }
    
    pub fn get_width(&self) -> i32 {
        self.width
    }
    
    pub fn get_height(&self) -> i32 {
        self.height
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            let rbos_vec = vec![self.depth_buffer];
            gl::DeleteRenderbuffers(rbos_vec.len() as gl::types::GLsizei, rbos_vec.as_ptr());
            
            let fbos_vec = vec![self.framebuffer_object];
            gl::DeleteFramebuffers(fbos_vec.len() as gl::types::GLsizei, fbos_vec.as_ptr());
        }
    }
}