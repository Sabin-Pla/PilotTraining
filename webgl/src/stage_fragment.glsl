#version 300 es
precision highp float;

in vec2 vert;
out vec4 outColor;
in vec2 center; 

void main() {
    float dist = distance(vert.xy, center.xy);
    // vec2 dist = abs(vert.xy - center.xy);
    outColor = vec4(0.2, 1.0, 0.2, 1.0);
}