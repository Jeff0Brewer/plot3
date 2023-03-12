#version 330

in vec4 position;
in vec2 offset;
in vec2 a_texCoord;
uniform mat4 mvp;
uniform float scale;
out vec2 v_texCoord;

void main() {
    gl_Position = mvp * position;
    gl_Position.xy += offset * scale * gl_Position.w;
    v_texCoord = a_texCoord;
}
