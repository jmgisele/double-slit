use crate::component::SlitStructure;
use crate::{component::CustomMaterial, WINDOW_HEIGHT};
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct InterferencePlugin;
impl Plugin for InterferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<CustomMaterial>::default())
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_screen);
        // .add_system(change_color);
    }
}

pub const BASELINE_Y_SLITS: f32 =
    (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2. - SLIT_SCREEN_HEIGHT * 2.;
pub const BASELINE_X_SLITS: f32 = 150. - SLIT_SCREEN_WIDTH / 2.;
pub const SLIT_SCREEN_WIDTH: f32 = 500.;
pub const SLIT_SCREEN_HEIGHT: f32 = 100.;

fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    slit_structure: Res<SlitStructure>,
) {
    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
            .into(),
        material: materials.add(CustomMaterial {
            screen_distance: slit_structure.screen_distance,
            separation: slit_structure.separation,
            slit_width: slit_structure.slit_width,
            wavelength: slit_structure.wavelength,
        }),
        transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
        ..default()
    });
}

pub fn change_color(mut materials: ResMut<Assets<CustomMaterial>>) {
    for material in materials.iter_mut() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let color = Color::rgba(
            rng.gen_range(0.0..1.),
            rng.gen_range(0.0..1.),
            rng.gen_range(0.0..1.),
            1.,
        );

        // material.1.color = color;
    }
}
