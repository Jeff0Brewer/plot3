#version 330

in vec2 position
in vec2 a_texCoord;
out vec2 v_texCoord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_texCoord = a_texCoord;
}
