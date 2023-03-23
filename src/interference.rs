use crate::component::{InputType, ParticlesMaterial, SlitStructure};
use crate::slit::wavelength_to_rgb;
use crate::{component::LightMaterial, WINDOW_HEIGHT};
use bevy::ecs::schedule::ShouldRun;
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct InterferencePlugin;
impl Plugin for InterferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<LightMaterial>::default())
            .add_plugin(Material2dPlugin::<ParticlesMaterial>::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(light_criteria)
                    .with_system(output_light),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(particles_criteria)
                    .with_system(output_particles),
            );
    }
}

pub const BASELINE_Y_SLITS: f32 =
    (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2. - SLIT_SCREEN_HEIGHT * 2.;
pub const BASELINE_X_SLITS: f32 = 150. - SLIT_SCREEN_WIDTH / 2.;
pub const SLIT_SCREEN_WIDTH: f32 = 500.;
pub const SLIT_SCREEN_HEIGHT: f32 = 100.;

pub const SCREEN_COLOR: Color = Color::rgba_linear(0.3, 0.1, 0., 1.0);
pub const BORDER_COLOR: Color = Color::rgba(0.27843, 0.18039, 0.12157, 1.0);

fn output_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_material: ResMut<Assets<LightMaterial>>,
    slit_structure: Res<SlitStructure>,
    particles_query: Query<Entity, With<InputType>>,
) {
    if let Ok(particles) = particles_query.get_single() {
        println!("despawning particles");
        commands.entity(particles).despawn();
    }
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

fn output_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut particles_material: ResMut<Assets<ParticlesMaterial>>,
    slit_structure: Res<SlitStructure>,
    light_query: Query<Entity, With<InputType>>,
) {
    if let Ok(light) = light_query.get_single() {
        println!("despawning light");

        commands.entity(light).despawn();
    }
    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: particles_material.add(ParticlesMaterial {
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
        .insert(InputType::Particles);
}

pub fn particles_criteria(slit_structure: Res<SlitStructure>) -> ShouldRun {
    if slit_structure.is_changed() && matches!(slit_structure.toggle_input, InputType::Particles) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn light_criteria(slit_structure: Res<SlitStructure>) -> ShouldRun {
    if matches!(slit_structure.toggle_input, InputType::Light) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
