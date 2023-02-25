#version 330

uniform sampler2D charTexture;
in vec2 v_texCoord;
out vec4 FragColor;

void main() {
    FragColor = texture(charTexture, v_texCoord);
}
