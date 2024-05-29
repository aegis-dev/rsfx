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

#version 450 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texture_coords;
layout(location = 2) in vec3 normal;

layout(location = 3)  uniform mat4 transformation_matrix;
layout(location = 7)  uniform mat4 projection_matrix;
layout(location = 11) uniform mat4 view_matrix;

layout(location = 15) uniform float fog_min;
layout(location = 16) uniform float fog_max;

out vec2 frag_texture_coords;
out vec3 frag_normal;
out float frag_fog_density;

vec2 resolution = vec2(427.0, 240.0);

// Position is post MVP translation of vertex
vec4 to_low_precision(vec4 position, vec2 resolution) {
    // https://www.hawkjames.com/indiedev/update/2022/06/02/rendering-ps1.html
    
	//Perform perspective divide
	vec3 perspective_divide = position.xyz / vec3(position.w);
	
	//Convert to screenspace coordinates
	vec2 screen_coords = (perspective_divide.xy + vec2(1.0,1.0)) * vec2(resolution.x,resolution.y) * 0.5;

	//Truncate to integer
	vec2 screen_coords_truncated = vec2(int(screen_coords.x), int(screen_coords.y));
	
	//Convert back to clip range -1 to 1
	vec2 reconverted_xy = ((screen_coords_truncated * vec2(2,2)) / vec2(resolution.x,resolution.y)) - vec2(1,1);

	//Construct return value
	vec4 snapped = vec4(reconverted_xy.x, reconverted_xy.y, perspective_divide.z, position.w);
	snapped.xyz = snapped.xyz * vec3(position.w, position.w, position.w);
	
	return snapped;
}

void main(void) {
	vec4 world_position = transformation_matrix * vec4(position, 1.0);
	vec4 world_view = view_matrix * world_position;
	vec4 projection_world_view = projection_matrix * world_view;
	
    float depth = abs(world_view.z / world_view.w);
	
	projection_world_view = to_low_precision(projection_world_view, resolution);
    
	gl_Position = projection_world_view;
	
    frag_texture_coords = texture_coords;
    frag_normal = (transformation_matrix * vec4(normal, 0.0)).xyz;
    
    frag_fog_density = 1.0 - clamp((fog_max - depth) / (fog_min - fog_max), 0.0, 1.0);
}