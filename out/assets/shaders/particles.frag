precision mediump float;

// layout(location = 0) in vec2 v_Uv;
// layout(location = 1) in vec3 v_Idk;
// layout(location = 2) in vec2 v_Idk_2;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ParticlesMaterial {
    vec4 Color;
};


void main() {
    o_Target = Color;
}
