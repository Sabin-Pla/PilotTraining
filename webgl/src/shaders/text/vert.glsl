#version 300 es
precision highp float;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec4 a_texCoord;
out vec2 v_texCoord;
 
void main() {
   gl_Position = vec4(a_position.xy, 1.0, 1.0);
   v_texCoord = a_texCoord.xy;
   return;
   if (gl_VertexID  == 0)  {
     //gl_Position = vec4(-1.0, -1.0, 1.0, 1.0);
   } else if (gl_VertexID  == 1)  {
      gl_Position = vec4(-1.0, 0.0, 1.0, 1.0);
   } else if (gl_VertexID  == 2)  {
      gl_Position = vec4(0.0, -1.0, 1.0, 1.0);
   } if (gl_VertexID  == 3)  {
      gl_Position = vec4(0.0, 0.0, 1.0, 1.0);
   }
}