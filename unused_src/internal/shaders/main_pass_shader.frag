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

in vec2 frag_texture_coords;
in vec3 frag_normal;
in float frag_fog_density;

out vec4 color;

layout(binding = 0) uniform sampler2D texture_sampler;

layout(location = 17) uniform vec3 directional_light_color;
layout(location = 18) uniform vec3 directional_light_direction;
layout(location = 19) uniform float directional_light_brightness;

// https://en.wikipedia.org/wiki/Ordered_dithering
const int threshold_map[8][8] = {
    { 0, 32,  8, 40,  2, 34, 10, 42},
    {48, 16, 56, 24, 50, 18, 58, 26}, 
    {12, 44,  4, 36, 14, 46,  6, 38}, 
    {60, 28, 52, 20, 62, 30, 54, 22}, 
    { 3, 35, 11, 43,  1, 33,  9, 41}, 
    {51, 19, 59, 27, 49, 17, 57, 25},
    {15, 47,  7, 39, 13, 45,  5, 37},
    {63, 31, 55, 23, 61, 29, 53, 21}
};

float number_of_colors = 255.0;
float dither_scale = 1.0;

float get_threshold(int x, int y, float brightness) {
    float limit = (threshold_map[x][y] + 1.0) / 64.0;
    
    return brightness < limit ? 0.0 : 1.0; 
}

float luma(vec3 color) {
    return dot(color, vec3(0.299, 0.587, 0.114));
}

void main(void) {
//    vec4 sampled_color = texture2D(texture_sampler, frag_texture_coords);
//    
//    vec2 xy = gl_FragCoord.xy * dither_scale;
//    int x = int(mod(xy.x, 8));
//    int y = int(mod(xy.y, 8));
//    
//    //vec3 dithered_color = floor(sampled_color.rgb * number_of_colors + 0.5) / number_of_colors;
//    
//    float threshold = get_threshold(x, y, luma(sampled_color.rgb));
//    vec3 dithered_color = sampled_color.rgb * threshold;
//
//    color = vec4(dithered_color, sampled_color.a);

    vec3 unit_normal = normalize(frag_normal);
    float normal_light_dot_product = dot(unit_normal, directional_light_direction);
    float brightness = max(normal_light_dot_product, directional_light_brightness);
    vec3 diffuse = vec3(brightness * directional_light_color);

    color = vec4(diffuse, 1.0) * texture2D(texture_sampler, frag_texture_coords) * frag_fog_density;
}