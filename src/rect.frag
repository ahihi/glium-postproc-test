#version 150

#define TAU 6.283185307179586

uniform sampler2D tex;
uniform vec2 resolution;

out vec4 color;

vec2 rect2polar(vec2 p) {
    return vec2(atan(p.y, p.x), length(p));
}

vec2 polar2rect(vec2 p) {
    return vec2(cos(p.x) * p.y, sin(p.x) * p.y);
}

vec4 rgb_shift(sampler2D tex, vec2 p, float shift) {
    float offset = 0.25 * TAU;
    vec2 rs = polar2rect(vec2(offset + 0.0/3.0 * TAU, shift));
    vec2 gs = polar2rect(vec2(offset + 1.0/3.0 * TAU, shift));
    vec2 bs = polar2rect(vec2(offset + 2.0/3.0 * TAU, shift));
    
    float r = texture(tex, p+rs, 0.0).x / 3.0;
    float g = texture(tex, p+gs, 0.0).y / 3.0;
    float b = texture(tex, p+bs, 0.0).z / 3.0;
    
    return vec4(r, g, b, 1.0);
}
void main() {
    vec2 uv = gl_FragCoord.xy/resolution;
    float dist = distance(uv, vec2(0.5, 0.5));
    
    color = rgb_shift(tex, uv, pow(0.3*dist, 2.0));
}
