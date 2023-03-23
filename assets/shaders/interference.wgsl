struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> separation: vec4<f32>; // micrometers
@group(1) @binding(1)
var<uniform> slit_width: vec4<f32>; // micrometers
@group(1) @binding(2)
var<uniform> wavelength: vec4<f32>; // nanometer
@group(1) @binding(3)
var<uniform> screen_distance: vec4<f32>; // centimeters

@group(1) @binding(4)
var<uniform> background: vec4<f32>;
@group(1) @binding(5)
var<uniform> light_color: vec4<f32>;
@group(1) @binding (6)
var<uniform> border: vec4<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let x: f32 = input.uv.x;
    let y: f32 = input.uv.y;
    if x < 0.005 || x > 0.995 || y < 0.03 || y > 0.97 {
        return border;
    }
    
    let full_screen_width: f32 = 0.4; // m

    let displacement: f32 = (x - 0.5) * full_screen_width; 

    let separation: f32 = separation.x * 10e-6; // meters 
    let slit_width: f32 = slit_width.x * 10e-6; // meters
    let wavelength: f32 = wavelength.x * 10e-9; // meters
    let screen_distance: f32 = screen_distance.x * 0.01; // meters

    let sine_theta: f32 =   displacement / sqrt(displacement * displacement + screen_distance * screen_distance);
   
    let coeff_a: f32 = ((3.1415 * slit_width) / wavelength ) * sine_theta;
    let probability: f32 = (sin(coeff_a) / coeff_a) * (sin(coeff_a) / coeff_a);

    let coeff_b: f32 = ((3.1415 * separation) / wavelength) * sine_theta;
    let interference: f32 = cos(coeff_b) * cos(coeff_b);

    let intensity: f32 = probability * interference;

    var i: vec4<f32> = mix(background, light_color, intensity);

    // i.z = intensity;

    return i;
}
