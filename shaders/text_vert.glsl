#version 330

in vec4 position;
in vec2 offset;
in vec2 a_texCoord;
uniform mat4 mvp;
out vec2 v_texCoord;

void main() {
    gl_Position = mvp * position;
    gl_Position.xy += offset * .003;
    v_texCoord = a_texCoord;
}
