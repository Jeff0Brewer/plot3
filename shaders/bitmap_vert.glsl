#version 330

in vec2 position;
in vec2 a_texCoord;
uniform vec2 offset;
uniform vec2 dimensions;
out vec2 v_texCoord;

void main() {
    v_texCoord = a_texCoord;
    gl_Position = position * dimensions + offset;
}
