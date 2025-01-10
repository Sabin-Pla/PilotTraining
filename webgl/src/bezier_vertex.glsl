#version 300 es
precision highp float;

in vec4 vert_idx;
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

layout (std140) uniform u_nodes {
    vec2 n0;
    vec2 n1;
    vec2 n2;
    vec2 n;
};

layout (std140) uniform u_resolution {
    vec2 renamed_res;
    vec2 placeholder;
};


void main() {

	res = vec4(renamed_res.xy, placeholder.xy);
	//res = vec4(500.0, 500.0, 0.0, 0.0);

	int vert_idx = int(vert_idx / 3.0);
	width = 1.0;

	p0 = n0;
	p1 = n1;
	p2 = n2;
	vec2 p = position.xy;

	if (0 == 1) {
		if (p == p0) {
			// Make room so that the whole width of the curve is actually inside the
			// triangle fragment. otherwise the bezier nodes would be directly
			// on the fragment edge.
			p = p0 + (width * (p0-p2) / distance(p0, p2)); 
		} else if (p == p1) {
			vec2 lower_mid = (p0 + p2) / 2.0;
			p = p1 + (width * (p1-lower_mid) / distance(p1, lower_mid)); 
		} else {
			p = p2 + (width * (p2-p0) / distance(p2, p0)); 
		}
	}

    gl_Position = vec4(p, 1.0, 1.0);
}
