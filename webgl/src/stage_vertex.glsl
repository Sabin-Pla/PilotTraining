#version 300 es
precision highp float;

in vec4 position;
in vec4 vert_idx;
in vec4 vert_color;
out vec2 center;
out vec2 vert;
out vec4 color;

layout (std140) uniform u_camera {
    vec2 camera;
    vec2 zoom;
};

struct ObjectCenter {
    vec2 centers;
    vec2 offset;
};

layout (std140) uniform u_colors {
    vec4 colors [2];
};

layout (std140) uniform u_worldspace_centers {
    ObjectCenter objects_center [2];
};

void main() {
    int rect_idx = int(vert_idx / 4.0);
    center = objects_center[rect_idx].centers - objects_center[rect_idx].offset;
    color = colors[rect_idx];
    vert = (position.xy + center) * zoom.xy  - camera.xy;
    gl_Position = vec4(vert.xy, position.z, position.w);
}
