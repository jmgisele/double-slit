precision highp float;

layout(location = 0) in vec2 v_world;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;


layout(set = 1, binding = 0) uniform Sep {
    vec4 separation; // micrometers
};
layout(set = 1, binding = 1) uniform Width {
    vec4 slit_width; // micrometers
};
layout(set = 1, binding = 2) uniform Wavelength {
    vec4 wavelength; // nanometer
};
layout(set = 1, binding = 3) uniform Distance {
    vec4 screen_distance; // centimeters
};

layout(set = 1, binding = 4) uniform BackgroundColor {
    vec4 background; 
};
layout(set = 1, binding = 5) uniform LightColor {
    vec4 light_color;
};
layout(set = 1, binding = 6) uniform BorderColor {
    vec4 border;
};

layout(location = 0) out vec4 color;

void main() {
    float x = v_uv.x;
    float y = v_uv.y;
    if (x < 0.005 || x > 0.995 || y < 0.03 || y > 0.97) {
        color = border;
    } else { 
        float full_screen_width = 0.4; // m

        float displacement = (x - 0.5) * full_screen_width; 

        float separation = separation.x * 10e-6; // meters 
        float slit_width = slit_width.x * 10e-6; // meters
        float wavelength = wavelength.x * 10e-9; // meters
        float screen_distance = screen_distance.x * 0.01; // meters

        float sine_theta = displacement / sqrt(displacement * displacement + screen_distance * screen_distance);
    
        float coeff_a = ((3.1415 * slit_width) / wavelength ) * sine_theta;
        float probability = (sin(coeff_a) / coeff_a) * (sin(coeff_a) / coeff_a);

        float coeff_b = ((3.1415 * separation) / wavelength) * sine_theta;
        float interference = cos(coeff_b) * cos(coeff_b);

        float intensity= probability * interference;

        vec4 i = mix(background, light_color, intensity);

        color = i;
    }
}

