#version 330

in vec2 position;
in vec2 a_texCoord;
uniform vec2 map_size;
uniform vec2 char_size;
uniform vec2 offset;
out vec2 v_texCoord;

void main() {
    vec2 pixel_pos = position * char_size + offset;
    gl_Position = vec4(2.0 * (pixel_pos / map_size) - 1.0, 0.0, 1.0);
    v_texCoord = a_texCoord;
}
