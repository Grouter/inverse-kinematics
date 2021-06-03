#version 140

in vec2 position;
in vec3 color;
in mat4 transform;

uniform mat4 perspective;

out vec3 vertex_color;

void main() {
    vertex_color = color;
    gl_Position = perspective * transform * vec4(position, 0.0, 1.0);
}
