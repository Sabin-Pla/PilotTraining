#version 300 es
precision highp float;


in float width;
in vec2 p0;
in vec2 p1;
in vec2 p2;
in vec4 res;
out vec4 outColor;

vec2 lerp(float t, vec2 p0, vec2 p1) {
    return p0 + t * (p1-p0);
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

    vec2 p = (gl_FragCoord.xy / res.xy -(0.5, 0.5)) * 2.0; 
   // p = gl_FragCoord.xy / 1.0*vec2(522.0, 293.0) + vec2(-0.5,-0.5); 
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
    float d=pow(pow(z1, 3.0) / 27.0 + pow(z2, 2.0) / 4.0, 0.5);

    float u1=pow((-z2/2.0)+d, 1.0/3.0);
    float diff =-z2/2.0-d;
    float u2;
    u2=pow(abs(diff), 1.0/3.0);
    if (diff < 0.0) {
        u2=-u2;
    }
    float x1=u1+u2;
    float t=x1-a/3.0;

    vec2 l0 = lerp(t, p0, p1);
    vec2 l1 = lerp(t, p1, p2);
    vec2 l2 = lerp(t, l0, l1);

    //l2 = p0 +2.0*t*p1 -2.0*t*p0+pow(t, 2.0)*p2-2.0*pow(t, 2.0)*p1+pow(t, 2.0)*p0;
    float dist = distance(l2, p);
    float test = pow(x1, 3.0) + z1*x1 + z2 ;

    if (dist >= width) { 
        // fragment should be discarded here, but we're using these values
        // to give an indication if the cubic equation solution failed
        outColor = vec4(test, test, test, 1.0);
    } else {
        float intensity = 0.3;
        outColor = vec4(intensity, 0.0, 0.0, 1.0);
        float edge_range = width * 0.1;
        float margin = width - dist;
        if (margin <= edge_range) {
            outColor.xyz = outColor.xyz * (1.0 - ((edge_range - margin) / edge_range));
        }
    }
}