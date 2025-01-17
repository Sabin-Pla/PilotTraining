#version 300 es
precision highp float;


in float width;
in vec2 p0;
in vec2 p1;
in vec2 p2;
in vec4 res;
out vec4 outColor;

// (entity_space.xy + world_space.xy) * zoom.xy  - camera.xy;
#define PI 3.1415926538

vec2 lerp(float t, vec2 p0, vec2 p1) {
    return p0 + t * (p1-p0);
}

vec2 complex_mult(vec2 c1, vec2 c2) {
    return vec2(c1.x * c2.x - c1.y * c2.y, c1.x * c2.y + c1.y * c2.x); 
}

vec2 complex_recip(vec2 c) {
    return vec2(
        c.x / (pow(c.x, 2.0) + pow(c.y, 2.0)), 
        -c.y / (pow(c.x, 2.0) + pow(c.y, 2.0)));
}

mat3x4 complex_cuberoot(float a, float b) {
    float mag = distance(vec2(0.0,0.0), vec2(a, b));
    vec2 unit = vec2(a, b) / mag;

    float rad = atan(b/a);
    if (a <= 0.0) {
        rad += PI;
    }
    rad = rad / 3.0;

    float cuberoot_mag = pow(mag, 1.0/3.0);

    // line which line with 1/3rd the angle of (a, b) goes through.
    vec2 primary_cuberoot = cuberoot_mag * vec2(cos(rad), sin(rad));
    
    // use cube roots of unity to return all 3 cube roots
    vec2 e1 = vec2(-0.5, -sqrt(3.0)/2.0);
    vec2 e2 = vec2(-0.5, sqrt(3.0)/2.0);
    return mat3x4(
        vec4(primary_cuberoot.xy, cuberoot_mag, rad), 
        vec4(complex_mult(primary_cuberoot, e1), cuberoot_mag, rad), 
        vec4(complex_mult(primary_cuberoot, e2), cuberoot_mag, rad));
}

bool is_expected(float f, float e) {
    float limit = 0.01;
    return ((f - e) < limit && (f - e) > -limit);
}

bool is_expected_range(float f, float e, float limit) {
    return ((f - e) < limit && (f - e) > -limit);
}

bool draw_bezier_node(vec2 p, vec2 p_other, vec4 node_color) {
    float node_size = 0.025;

    if (p.x > (p_other.x - node_size / 2.0) && p.x <  (p_other.x + node_size / 2.0) ) {
        if (p.y > (p_other.y - node_size / 2.0) && p.y <  (p_other.y + node_size / 2.0) ) {
            outColor = node_color;
            return true;
        }
    }
    return false;
}

void main() {

    // given and a current point P (gl_Position), which we want to verify is on a bezier curve or not, and 
    // a triangle (p0, p1, p2) with LERP points l0 and l1 along its sides:
    //     p1 
    //       l1
    //    l2  
    //  l0
    // p0       p2


    // LERP points are define with parameter 0 <= t <= 1
    // lerp(t, p1, p2) = p1 + t(p2-p1)
    // l0 = lerp(t, p0, p1)
    // l1 = lerp(t, p1, p2)
    // l2 = lerp(t, l0, l1) <--- evolving this point through time defines the bezier curve

    //  that produces this equality,
    // then draw the pixel if |(P - midpoint(l0, l1))| is greater than the width of the curve.
    // dot_product( (P - l2(l0, l1),   d/dt (l2(l0, l1))) == 0
    // (P - l2(l0, l1)).x  (d/dt (l2(l0, l1))).x +  (P - l2(l0, l1)).y  (d/dt (l2(l0, l1))).y = 0
    // after expanding and collecting like terms, equation for l2 is in the form of a cubic function

    // solve using cardano's method.
    // graphing calculator demo: https://www.desmos.com/calculator/7tc0crnzai?lang=ja

    vec2 p = (gl_FragCoord.xy / res.xy - (0.5, 0.5)) * 2.0; 

    if (draw_bezier_node(p, p0,  vec4(0.9, 0.9, 0.9, 1.0))) {
        return; 
    }
    
    if (draw_bezier_node(p, p1,  vec4(0.2, 0.9, 0.2, 1.0))) {
        return;
    }

    if (draw_bezier_node(p, p2,  vec4(0.9, 0.2, 0.2, 1.0))) {
        return;
    }

    float A = p0.x;
    float B = p1.x;
    float C = p2.x;
    float D = p0.y;
    float E = p1.y;
    float F = p2.y;

    float c0=2.0*pow(A, 2.0)-2.0*A*B-2.0*A*p.x+2.0*B*p.x+2.0*pow(D, 2.0)-2.0*D*E-2.0*D*p.y+2.0*E*p.y;
    float c1=-6.0 *pow(A, 2.0)+
        12.0*A*B-2.0*A*C+2.0*A*p.x-4.0*pow(B, 2.0)-
        4.0*B*p.x+2.0*C*p.x-6.0*pow(D,2.0)+12.0*D*E-
        2.0*D*F+2.0*D*p.y-4.0*pow(E,2.0)-4.0*E*p.y+2.0*F*p.y;
    float c2=(6.0 *pow(A, 2.0)-18.0*A*B+6.0*A*C+12.0*pow(B, 2.0)-6.0*B*C+6.0*pow(D, 2.0)-18.0*D*E+6.0*D*F+12.0*pow(E, 2.0)-6.0*E*F);
    float c3=(-2.0*pow(A, 2.0)+8.0*A*B-4.0*A*C-8.0*pow(B, 2.0)+8.0*B*C-2.0*pow(C, 2.0)-2.0*pow(D, 2.0)+8.0*D*E-4.0*D*F-8.0*pow(E,2.0)+8.0*E*F-2.0*pow(F,2.0));
    float a=c2/c3;
    float b=c1/c3;
    float c=c0/c3;

    float z1= b - pow(a, 2.0)/(3.0);
    float z2= 2.0 * pow(a, 3.0)/(27.0) - (a*b/3.0) + c;
    float rooted_val = (pow(z1, 3.0) / 27.0) + (pow(z2, 2.0) / 4.0);

    //float d=sqrt(abs(rooted_val)); // <--- can't use this???

    mat3x2 roots;
    mat3x4 cbrt;
    float d=0.0;
    float x1 =0.0;
    if (rooted_val <= 0.0) {
        rooted_val = -1.0 * rooted_val;
        cbrt = complex_cuberoot(-z2/2.0, pow(rooted_val, 0.5));
    }  else {
        cbrt = complex_cuberoot(-z2/2.0 + pow(rooted_val, 0.5), 0.0);
    }

    float dist = 2.0 * width;
    float t; 
    vec2 l0;
    vec2 l1;
    vec2 l2;
    vec2 last_l2;
    float used_root;
    float testval;
    int i=0;
    while (i < 3) {
        float root = cbrt[i].x - (z1 / (3.0 * cbrt[i].x));
        //  ^ fix this. should be complex subtraction
        vec2 recip = complex_recip(cbrt[i].xy);
        root = cbrt[i].x - (z1/3.0 * recip.x);
        testval = pow(cbrt[i].x, 2.0) + pow(cbrt[i].y, 2.0);

        float expected_zero = pow(root, 3.0) + z1*root + z2 ;
        roots[i] = vec2(root, expected_zero);
        float t_temp=root-a/3.0; 
        vec2 l0 = lerp(t_temp, p0, p1);
        vec2 l1 = lerp(t_temp, p1, p2);
        vec2 l2 = lerp(t_temp, l0, l1);
        float dist_new = distance(l2, p);
        if (dist_new < dist) {
            last_l2 = l2;
            dist = dist_new;
            t = t_temp;
            used_root = root;
        }
        i += 1;
    }


    if (dist >= width ) { 
        discard;
    } else {
        float intensity = 0.7;
  
        outColor = vec4(intensity * 0.4, intensity * 0.45, intensity, 1.0); 
  
        float edge_range = width * 0.2;

        
        float d1 = distance(p, p0);
        float d2 =  distance(p, p2);
        if (t < 0.0 && (d1 > width - edge_range)) {
            dist = d1;
        } else if (t > 1.0 && (d2 > width - edge_range)) {
            dist = d2;
        }

        float margin = width - dist;
        if (margin <= edge_range) {
            outColor.xyz = outColor.xyz * (1.0 - ((edge_range - margin) / edge_range));
        }
        return;
    }



} 