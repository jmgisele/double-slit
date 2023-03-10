struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct SlitData {
    separation: f32,      // centimeters
    slit_width: f32,      // milimeters
    wavelength: f32,      // nanometers
    screen_distance: f32, // centimeters
};

@group(1) @binding(0)
var<uniform> uniform_data: SlitData;


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let x: f32 = input.uv.x;
    
    let displacement: f32 = 0.5 - x;

    let separation: f32 = uniform_data.separation; // meters 
    // let slit_width: f32 = uniform_data.slit_width * 0.001; // meters
    // let wavelength: f32 = uniform_data.wavelength * 10e-9; // meters
    



    let intensity = sin( displacement * 100. );

    return vec4<f32>(0.3, 0.0, intensity, 1.0);
}
