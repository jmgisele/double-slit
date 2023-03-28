precision mediump float;

layout(location = 0) in vec2 v_world;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;


layout(set = 1, binding = 0) uniform ParticlesMaterial {
    vec4 MainColor;
};

layout(set = 1, binding = 1) uniform BorderMaterial {
    vec4 BorderColor;
};

layout(location = 0) out vec4 color;


void main() {
    float x = v_uv.x;
    float y = v_uv.y;
    if (x < 0.005 || x > 0.995 || y < 0.03 || y > 0.97) {
        color = BorderColor;
    } else {
        color =  MainColor;
    }
}
