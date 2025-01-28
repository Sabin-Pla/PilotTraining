#version 300 es
precision highp float;

uniform lowp usampler2D u_image;

in vec2 v_texCoord;

out vec4 outColor;

void main() {
   float alpha =  float(texture(u_image, v_texCoord).x)  / 255.0;
   outColor.y = alpha;
   if (alpha == 0.0) {
      discard;
   }
   outColor.w = 1.0;
}