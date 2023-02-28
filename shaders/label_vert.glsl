#version 330

in vec2 position;
in vec2 a_texCoord;
out vec2 v_texCoord;
uniform mat4 mvp;
uniform vec4 offset;
uniform vec4 alignment;

vec2 rotate2d(vec2 vec, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rotation = mat2(c, -s, s, c);
    return rotation * vec;
}

void main() {
    vec4 off_pos = mvp * offset;
    vec2 rotated_pos = position;
    if (length(alignment) != 0.0) {
        // calculate text orientation from diff
        // between offset and alignment vecs
        vec4 align_pos = mvp * alignment;
        vec3 p0 = align_pos.xyz/align_pos.w;
        vec3 p1 = off_pos.xyz/off_pos.w;
        vec3 orient = normalize(p0 - p1);

        // text layed out in x axis by default
        vec3 defaultOrient = vec3(1.0, 0.0, 0.0);

        // rotate text verts in 2d to match orientation
        float angle = acos(dot(orient, defaultOrient));
        // ensure text isn't upside down
        if (abs(angle) > 1.5708) { angle -= 3.1416; }
        rotated_pos = rotate2d(position, angle);
    }
    gl_Position = vec4(vec3(rotated_pos * .003, 0.0) + off_pos.xyz, off_pos.z);
    v_texCoord = a_texCoord;
}
