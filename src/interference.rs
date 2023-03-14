use crate::component::SlitStructure;
use crate::slit::update_display_criteria;
use crate::{component::CustomMaterial, WINDOW_HEIGHT};
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct InterferencePlugin;
impl Plugin for InterferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<CustomMaterial>::default())
            // .add_startup_system_to_stage(StartupStage::PostStartup, setup_screen);
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(setup_screen),
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

fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
            background_color: SCREEN_COLOR,
            border_color: BORDER_COLOR,
        }),
        transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
        ..default()
    });
}
