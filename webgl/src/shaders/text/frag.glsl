#version 300 es
precision highp float;

uniform lowp usampler2D u_image;

in vec2 v_texCoord;

out vec4 outColor;

void main() {
   outColor.y = float(texture(u_image, v_texCoord).x)  / 255.0;
   outColor.w = 1.0;
}