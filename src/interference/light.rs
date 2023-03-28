use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    component::{InputType, LightMaterial, ParticlesMesh, SlitStructure},
    slit::wavelength_to_rgb,
    WINDOW_HEIGHT,
};

use super::{BASELINE_X_SLITS, BORDER_COLOR, SCREEN_COLOR, SLIT_SCREEN_HEIGHT, SLIT_SCREEN_WIDTH};

pub fn output_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_material: ResMut<Assets<LightMaterial>>,
    slit_structure: Res<SlitStructure>,
    mut particles_mesh: ResMut<ParticlesMesh>,
    particles_query: Query<Entity, With<InputType>>,
) {
    for entity in particles_query.iter() {
        commands.entity(entity).despawn();
    }

    particles_mesh.reset_mesh();

    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: light_material.add(LightMaterial {
                screen_distance: Vec4::new(slit_structure.screen_distance, 0., 0., 0.),
                separation: Vec4::new(slit_structure.separation, 0., 0., 0.),
                slit_width: Vec4::new(slit_structure.slit_width, 0., 0., 0.),
                wavelength: Vec4::new(slit_structure.wavelength, 0., 0., 0.),
                background_color: SCREEN_COLOR,
                light_color: wavelength_to_rgb(&slit_structure.wavelength),
                border_color: BORDER_COLOR,
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
            ..default()
        })
        .insert(InputType::Light);
}

pub fn light_criteria(slit_structure: Res<SlitStructure>) -> bool {
    if slit_structure.is_changed() && matches!(slit_structure.toggle_input, InputType::Light) {
        true
    } else {
        false
    }
}
