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

use std::ffi::c_void;
use gl::{self, types::{GLuint, GLint}};
use gl::types::GLsizei;

use crate::internal::byte_buffer_reader::ByteBufferReader;

pub struct Texture {
    texture_id: GLuint,
    width: GLuint,
    height: GLuint,
}

#[derive(Clone, Copy)]
pub enum ImageMode {
    RED = gl::RED as isize,
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize
}

impl Texture {
    pub fn new(texture_id: GLuint, width: GLuint, height: GLuint) -> Texture {
        Texture { texture_id, width, height }
    }
    
    pub fn from_png_bytes(png_bytes: &[u8]) -> Result<Texture, String> {
        let decoder = png::Decoder::new(ByteBufferReader::from(png_bytes));
        let mut reader = match decoder.read_info() {
            Ok(reader) => reader.1,
            Err(error) => return Err(error.to_string())
        };
    
        let mut raw_image_data = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut raw_image_data) {
            Err(error) => return Err(error.to_string()),
            _ => { }
        };
    
        let info = reader.info();
        let width = info.width as usize;
        let height = info.height as usize;
        let color_size = raw_image_data.len() / (width * height);
        if color_size != 3 && color_size != 4 {
            return Err(
                String::from(format!("Unexpected image size or it is corrupted.\nwidth: {0} \
                , height: {1}, byte_count: {2}", width, height, raw_image_data.len()))
            );
        }
        let mode = match color_size {
            3 => ImageMode::RGB,
            4 => ImageMode::RGBA,
            _ => {
                return Err(
                    String::from("Unsupported image color mode. Expected RGB or RGBA color mode.")
                );
            }
        };
        
        Ok(Texture::from_data(&raw_image_data, width as u32, height as u32, mode))
    }
    
    pub fn from_data(data: &Vec<u8>, width: GLuint, height: GLuint, mode: ImageMode) -> Texture {
        let texture_id = {
            let mut texture_ids = vec![0];
            unsafe {
                gl::GenTextures(texture_ids.len() as GLsizei, texture_ids.as_mut_ptr());
            }
            texture_ids[0]
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            let internal_format = match mode {
                //ImageMode::RED => { 1 }
                _ => { mode as GLint }
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                width as GLint,
                height as GLint,
                0,
                mode as GLuint,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        }

        Texture { texture_id, width, height }
    }

    pub fn texture_id(&self) -> GLuint {
        self.texture_id
    }

    pub fn width(&self) -> GLuint {
        self.width
    }

    pub fn height(&self) -> GLuint {
        self.height
    }

    fn update_texture_data(&self, data: &Vec<u8>, mode: ImageMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            let internal_format = match mode {
                //ImageMode::ColorIndex => { 1 }
                _ => { mode as GLint }
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                self.width as GLint,
                self.height as GLint,
                0,
                mode as GLuint,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            let temp_vec = vec![self.texture_id];
            gl::DeleteTextures(temp_vec.len() as GLsizei, temp_vec.as_ptr());
        }
    }
}