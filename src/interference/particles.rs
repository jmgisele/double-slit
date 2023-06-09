use crate::component::{
    InputType, ParticleTimer, ParticlesMaterial, ParticlesMesh, ScreenMaterial, SlitStructure,
};
use crate::slit::wavelength_to_rgb;
use crate::WINDOW_HEIGHT;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology::PointList;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::Time;
use rand::prelude::*;

use super::{BASELINE_X_SLITS, BORDER_COLOR, SCREEN_COLOR, SLIT_SCREEN_HEIGHT, SLIT_SCREEN_WIDTH};

pub fn output_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut screen_material: ResMut<Assets<ScreenMaterial>>,
    mut particles_material: ResMut<Assets<ParticlesMaterial>>,
    slit_structure: Res<SlitStructure>,
    particles_mesh: Res<ParticlesMesh>,
    light_query: Query<Entity, With<InputType>>,
) {
    for entity in light_query.iter() {
        commands.entity(entity).despawn();
    }
    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;

    // screen
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: screen_material.add(ScreenMaterial {
                color: SCREEN_COLOR,
                border: BORDER_COLOR,
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
            ..default()
        })
        .insert(InputType::Particles);

    let particles_mesh = get_particles_mesh(&particles_mesh, &slit_structure);
    // particles
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(particles_mesh.into()).into(),
            material: particles_material.add(ParticlesMaterial {
                color: wavelength_to_rgb(&slit_structure.wavelength),
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.1)),
            ..default()
        })
        .insert(InputType::Particles);
}

pub fn output_particles_criteria(
    slit_structure: Res<SlitStructure>,
    mesh: Res<ParticlesMesh>,
) -> bool {
    if mesh.is_changed() && matches!(slit_structure.toggle_input, InputType::Particles) {
        true
    } else {
        false
    }
}

pub fn get_particles_mesh(particles_mesh: &ParticlesMesh, slit: &SlitStructure) -> Mesh {
    let mut mesh = Mesh::new(PointList);

    let _old_mesh = vec![[-20., -20., 0.], [1., 1., 0.], [0., 1., 0.], [1., 0., 0.]];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, particles_mesh.0.clone());
    // y: 50 to -50
    // x : -250 to 250

    // normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0., 1., 0.]; particles_mesh.0.len()],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; particles_mesh.0.len()]);

    // color comes from wavelength
    let color = wavelength_to_rgb(&slit.wavelength);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[color.r(), color.g(), color.b(), 1.]; particles_mesh.0.len()],
    );

    mesh
}

pub fn reset_particles(mut particles_mesh: ResMut<ParticlesMesh>) {
    particles_mesh.reset_mesh();
}

pub fn reset_particles_criteria(slit_structure: Res<SlitStructure>) -> bool {
    if slit_structure.is_changed() && matches!(slit_structure.toggle_input, InputType::Particles) {
        true
    } else {
        false
    }
}

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

fn get_particle_coord(slit: &SlitStructure) -> [f32; 3] {
    let mut rng = rand::thread_rng();

    let x: f32;
    let y: f32;

    loop {
        let x_prob: f32 = rng.gen(); // generates a float between 0 and 1
        let b_prob: f32 = rng.gen();

        let p_x = prob_x(x_prob, &slit);

        if b_prob < p_x {
            x = 498. * x_prob - 249.;
            break;
        }
    }

    loop {
        let y_prob: f32 = rng.gen();
        let b_prob: f32 = rng.gen();

        let p_y = prob_y(y_prob, &slit);

        if b_prob < p_y {
            y = 98. * y_prob - 49.;
            break;
        }
    }

    return [x, y, 0.];
}

fn prob_y(y: f32, slit: &SlitStructure) -> f32 {
    let full_screen_width: f32 = 0.2; // m

    let displacement: f32 = (y - 0.5) * full_screen_width;

    let slit_height: f32 = 5. * 10e-6; // meters
    let wavelength: f32 = slit.wavelength * 10e-9; // meters
    let screen_distance: f32 = slit.screen_distance * 0.01; // meters

    let sine_theta: f32 =
        displacement / (displacement * displacement + screen_distance * screen_distance).sqrt();

    let coeff_a: f32 = ((3.1415 * slit_height) / wavelength) * sine_theta;

    (coeff_a.sin() / coeff_a) * (coeff_a.sin() / coeff_a)
}

fn prob_x(x: f32, slit: &SlitStructure) -> f32 {
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
