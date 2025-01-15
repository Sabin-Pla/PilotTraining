#version 300 es
precision highp float;

in vec4 position;
out vec2 p0;
out vec2 p1;
out vec2 p2;
out float width;
out vec4 res;

layout (std140) uniform u_camera {
    vec2 camera;
    vec2 zoom;
};

layout (std140) uniform u_bezier_nodes {
    mat3x4 nodes[1]; 
};

layout (std140) uniform u_resolution {
    vec2 resolution;
    vec2 placeholder;
};

bool vert_equality(vec2 a, vec2 b) {
	return distance(a, b) <= 0.01;
}

void main() {
	res = vec4(resolution.xy, placeholder.xy);

	int vert_idx = gl_VertexID / 3;
	width = 0.05;

	p0 = nodes[vert_idx][0].xy;
	p1 = nodes[vert_idx][2].xy;
	p2 = nodes[vert_idx][1].xy;
	vec2 p = position.xy;

	if (vert_equality(p, p0)) { // start node
		// Make room so that the whole width of the curve is actually inside the
		// triangle fragment. otherwise the bezier nodes would be directly
		// on the fragment edge.
		p = p0 + (width * 2.0 * (p0-p2) / distance(p0, p2)); 
		p += (width * 2.0 * (p0-p1) / distance(p0, p1)); 
	} else if (vert_equality(p, p1)) { // control node
		p = p1 + (width * 2.0 * (p1-p0) / distance(p1, p0)); 
		p += (width * 2.0 * (p1-p2) / distance(p1, p2)); 
	} else if (vert_equality(p, p2)) { // end node
		p = p2 + (width * 2.0 * (p2-p0) / distance(p2, p0)); 
		p += (width * 2.0 * (p2-p1) / distance(p2, p1)); 
	}



    gl_Position = vec4(p, 1.0, 1.0);
}
