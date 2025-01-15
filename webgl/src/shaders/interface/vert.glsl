#version 300 es
precision highp float;

in vec4 position;
out vec4 color;

void main() {
	if (gl_VertexID / 4 == 0) {
		color = vec4(1.0, 0.0, 0.0, 1.0);
	} else if (gl_VertexID / 4 == 1) {
		color = vec4(0.0, 1.0, 0.0, 1.0);
	}
	gl_Position = position;
}