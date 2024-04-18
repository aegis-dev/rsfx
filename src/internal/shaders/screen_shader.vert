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

#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texture_coords;

layout(location = 2)  uniform mat4 transformation_matrix;
layout(location = 6)  uniform mat4 projection_matrix;
layout(location = 10) uniform mat4 view_matrix;

out vec2 frag_texture_coords;

void main(void) {
	vec4 world_position = transformation_matrix * vec4(position, 1.0);
	gl_Position = projection_matrix * view_matrix * world_position;
    frag_texture_coords = texture_coords;
}