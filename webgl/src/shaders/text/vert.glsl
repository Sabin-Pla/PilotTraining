#version 300 es
precision highp float;

in vec2 a_texCoord;
out vec2 v_texCoord;
 
void main() {
      // pass the texCoord to the fragment shader
   // The GPU will interpolate this value between points
   v_texCoord = a_texCoord;
}