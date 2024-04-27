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

use glam::{Mat4, Vec3, Quat};

use crate::math;

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

// Create a perspective projection matrix.
//  Parameters:
//  - fov (float): Field of view angle in degrees.
//  - aspect_ratio (float): Aspect ratio of the viewport (width / height).
pub fn build_perspective_matrix(fov: f32, aspect_ratio: f32) -> Mat4 {
    Mat4::perspective_infinite_rh(fov.to_radians(), aspect_ratio, 0.1)
}

pub fn build_view_matrix(position: &Vec3, rotation: &Vec3) -> Mat4 {
    let direction = math::get_direction_from_euler(rotation);
    let camera_right = UP.cross(direction).normalize();
    let camera_up = direction.cross(camera_right).normalize();
    
    Mat4::look_to_rh(position.clone(), direction, camera_up)
}

pub fn build_transformation_matrix(translation: &Vec3, rotation: &Vec3, scale: f32) -> Mat4 {
    let rot_quat = Quat::from_euler(glam::EulerRot::YXZ, rotation.y.to_radians(), rotation.x.to_radians(), rotation.z.to_radians());
    Mat4::from_scale_rotation_translation(Vec3::new(scale, scale, scale), rot_quat, translation.clone())
}