struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};


@group(1) @binding(0)
var<uniform> color: vec4<f32>;
@group(1) @binding(1)
var<uniform> border: vec4<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let x: f32 = input.uv.x;
    let y: f32 = input.uv.y;
    if x < 0.005 || x > 0.995 || y < 0.03 || y > 0.97 {
        return border;
    }
    return color;
}