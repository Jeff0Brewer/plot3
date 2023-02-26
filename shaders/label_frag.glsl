#version 330

uniform sampler2D fontTexture;
in vec2 v_texCoord;
out vec4 FragColor;

void main() {
    FragColor = texture(fontTexture, v_texCoord);
}
