#version 150

out vec4 color;

void main() {
    float k = 0.02;
    bool odd = fract(k * gl_FragCoord.x) < 0.5 != fract(k * gl_FragCoord.y) < 0.5;
    
    color = odd
        ? vec4(0.3, 0.3, 0.3, 1.0)
        : vec4(0.5, 0.5, 0.5, 1.0);
}
