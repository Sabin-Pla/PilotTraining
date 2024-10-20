#version 300 es
precision highp float;

in vec4 position;
in vec4 vert_idx;
out vec2 center;
out vec2 vert;

layout (std140) uniform u_camera {
    vec2 camera;
    vec2 zoom;
};

layout (std140) uniform u_centers {
    vec4 centers[1];
};

void main() {
    center = (centers[0].xy * zoom.xy) + camera.xy;
    vert = (position.xy * zoom.xy) + camera.xy;
    gl_Position = vec4(vert.xy, position.z, position.w);
}
