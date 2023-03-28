use crate::component::{
    InputType, ParticleTimer, ParticlesMaterial, ParticlesMesh, ScreenMaterial, SlitStructure,
};
use crate::slit::wavelength_to_rgb;
use crate::{component::LightMaterial, WINDOW_HEIGHT};
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use self::light::{light_criteria, output_light};
use self::particles::{
    add_particle, add_particles_criteria, output_particles, output_particles_criteria,
    reset_particles, reset_particles_criteria,
};

mod light;
mod particles;
pub struct InterferencePlugin;

impl Plugin for InterferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<LightMaterial>::default())
            .add_plugin(Material2dPlugin::<ScreenMaterial>::default())
            .add_plugin(Material2dPlugin::<ParticlesMaterial>::default())
            .init_resource::<ParticleTimer>()
            .add_startup_system(setup_screen.in_base_set(StartupSet::PostStartup))
            .add_system(output_light.run_if(light_criteria))
            .add_system(output_particles.run_if(output_particles_criteria))
            .add_system(
                add_particle
                    .run_if(add_particles_criteria)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(reset_particles.run_if(reset_particles_criteria));
    }
}

pub const BASELINE_Y_SLITS: f32 =
    (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2. - SLIT_SCREEN_HEIGHT * 2.;
pub const BASELINE_X_SLITS: f32 = 150. - SLIT_SCREEN_WIDTH / 2.;
pub const SLIT_SCREEN_WIDTH: f32 = 500.;
pub const SLIT_SCREEN_HEIGHT: f32 = 100.;

pub const SCREEN_COLOR: Color = Color::rgba_linear(0.3, 0.1, 0., 1.0);
pub const BORDER_COLOR: Color = Color::rgba(0.27843, 0.18039, 0.12157, 1.0);

fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_material: ResMut<Assets<LightMaterial>>,
    slit_structure: Res<SlitStructure>,
) {
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

    let mesh = ParticlesMesh::default();
    commands.insert_resource(mesh);
}
