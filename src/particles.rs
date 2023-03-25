use crate::component::{InputType, ParticleTimer, ParticlesMesh, SlitStructure};
use bevy::prelude::*;
use bevy::time::Time;

use rand::prelude::*;

pub fn add_particle(
    time: Res<Time>,
    mut timer: ResMut<ParticleTimer>,
    mut particles_mesh: ResMut<ParticlesMesh>,
    slit: Res<SlitStructure>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        for _ in 0..10 {
            let new_coord = get_particle_coord(&slit);

            particles_mesh.add_particle(new_coord);
        }
    }
}

pub fn add_particles_criteria(slit_structure: Res<SlitStructure>) -> bool {
    if matches!(slit_structure.toggle_input, InputType::Particles) {
        true
    } else {
        false
    }
}

pub fn get_particle_coord(slit: &SlitStructure) -> [f32; 3] {
    let mut rng = rand::thread_rng();

    let y: f32 = rng.gen_range(-49.0..49.0);

    loop {
        let x: f32 = rng.gen(); // generates a float between 0 and 1
        let b_prob: f32 = rng.gen();

        let p_x = probability(x, &slit);

        if b_prob < p_x {
            return [500. * x - 250., y, 0.];
        }
    }
}

pub fn probability(x: f32, slit: &SlitStructure) -> f32 {
    // x is in range 0 to 1
    let full_screen_width: f32 = 0.4; // m

    let displacement: f32 = (x - 0.5) * full_screen_width;

    let separation: f32 = slit.separation * 10e-6; // meters
    let slit_width: f32 = slit.slit_width * 10e-6; // meters
    let wavelength: f32 = slit.wavelength * 10e-9; // meters
    let screen_distance: f32 = slit.screen_distance * 0.01; // meters

    let sine_theta: f32 =
        displacement / (displacement * displacement + screen_distance * screen_distance).sqrt();

    let coeff_a: f32 = ((3.1415 * slit_width) / wavelength) * sine_theta;
    let probability: f32 = (coeff_a.sin() / coeff_a) * (coeff_a.sin() / coeff_a);

    let coeff_b: f32 = ((3.1415 * separation) / wavelength) * sine_theta;
    let interference: f32 = coeff_b.cos() * coeff_b.cos();

    probability * interference // between 0 and 1
}
