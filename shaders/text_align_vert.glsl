#version 330

in vec4 position;
in vec2 offset;
in vec2 a_texCoord;
uniform mat4 mvp;
uniform vec3 alignment;
out vec2 v_texCoord;

vec2 rotate2d(vec2 vec, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rotation = mat2(c, -s, s, c);
    return rotation * vec;
}

void main() {
    vec4 pos = mvp * position;
    vec4 align_pos = mvp * vec4(position.xyz + alignment, 1.0);

    // calculate text orientation from diff between position and alignment vecs
    vec3 p0 = align_pos.xyz/align_pos.w;
    vec3 p1 = pos.xyz/pos.w;
    vec3 orient = normalize(p0 - p1);

    // text layed out in x axis by default
    vec3 defaultOrient = vec3(1.0, 0.0, 0.0);

    // rotate text verts in 2d to match orientation
    float angle = acos(dot(orient, defaultOrient));
    // ensure text isn't upside down
    if (abs(angle) > 1.5708) { angle -= 3.1416; }
    vec2 rotated_off = rotate2d(offset, angle);

    gl_Position = pos;
    gl_Position.xy += rotated_off * pos.w * .002;
    v_texCoord = a_texCoord;
}
