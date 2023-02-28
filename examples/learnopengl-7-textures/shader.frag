#version 330 core

in vec2 texture_coord;

out vec4 colour;

uniform sampler2D tex;

void main() {
    colour = texture(tex, texture_coord);
}
