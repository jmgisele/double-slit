use crate::{
    component::{
        CustomMaterial, DisplayInfo, Increment, Light, Slit, SlitControl, SlitScreen, SlitStructure,
    },
    interference::{BASELINE_Y_SLITS, SLIT_SCREEN_HEIGHT},
    ui::{setup_ui, NORMAL_BUTTON, PRESSED_BUTTON},
};
use bevy::math::f32::Quat;
use bevy::sprite::Material2dPlugin;
use bevy::{ecs::schedule::ShouldRun, prelude::*, sprite::MaterialMesh2dBundle};

pub struct SlitPlugin;
impl Plugin for SlitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_startup_system(setup_slits)
            .add_system(increment_sep_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(update_slit_distance)
                    .with_system(update_slit_width)
                    .with_system(update_slit_separation)
                    .with_system(update_light_color),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(update_display_buttons),
            );
    }
}

const BASELINE_SLIT_WIDTH: f32 = 3.;
const BASELINE_SLIT_HEIGHT: f32 = 5.;
const BASELINE_SLIT_SCREEN_WIDTH: f32 = 300.;
const BASELINE_SLIT_SCREEN_X: f32 = -200.;

fn setup_slits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(BASELINE_SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(BASELINE_SLIT_SCREEN_X, 0., 0.)),
            ..default()
        })
        .insert(SlitScreen)
        .with_children(|parent| {
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Box::new(BASELINE_SLIT_WIDTH, BASELINE_SLIT_HEIGHT, 0.).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        BASELINE_SLIT_WIDTH * 3.,
                        0.,
                        0.1,
                    )),
                    ..default()
                })
                .insert(Slit::RightSlit);

            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Box::new(BASELINE_SLIT_WIDTH, BASELINE_SLIT_HEIGHT, 0.).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        -BASELINE_SLIT_WIDTH * 3.,
                        0.,
                        0.1,
                    )),
                    ..default()
                })
                .insert(Slit::LeftSlit);

            let laser_height = SLIT_SCREEN_HEIGHT - BASELINE_SLIT_HEIGHT;
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Box::new(2., laser_height, 0.).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    transform: Transform::from_translation(Vec3::new(
                        -0.5 * laser_height * (15. as f32).sin(),
                        -(laser_height / 2.) + BASELINE_SLIT_HEIGHT * 2.,
                        0.2,
                    ))
                    .with_rotation(Quat::from_rotation_z(15.)),
                    ..default()
                })
                .insert(Light);
        });
}

pub fn update_display_criteria(slit_structure: Res<SlitStructure>) -> ShouldRun {
    if slit_structure.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn update_light_color(
    display_query: Query<&Handle<ColorMaterial>, With<Light>>,
    slit_structure: Res<SlitStructure>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for handle in display_query.iter() {
        let mut color_mat = materials.get_mut(&handle).unwrap();
        color_mat.color = match slit_structure.wavelength.floor() as i32 {
            400..=500 => Color::BLUE,
            501..=600 => Color::GREEN,
            601..=700 => Color::RED,
            _ => Color::WHITE,
        }
    }
}

pub fn update_slit_width(
    mut display_query: Query<&mut Transform, With<Slit>>,
    slit_structure: Res<SlitStructure>,
) {
    for mut transform in display_query.iter_mut() {
        println!("updating slit width");
        transform.scale = Vec3::new(
            BASELINE_SLIT_WIDTH - (10. - slit_structure.slit_width) / 2.,
            BASELINE_SLIT_HEIGHT,
            0.1,
        );
    }
}

pub fn update_slit_distance(
    mut display_query: Query<&mut Transform, With<SlitScreen>>,
    slit_structure: Res<SlitStructure>,
) {
    for mut transform in display_query.iter_mut() {
        println!("updating screen distance");
        transform.translation = Vec3::new(
            BASELINE_SLIT_SCREEN_X,
            BASELINE_Y_SLITS + (100. - slit_structure.screen_distance) / 2.,
            0.,
        );
    }
}

pub fn update_slit_separation(
    mut display_query: Query<(&mut Transform, &Slit)>,
    slit_structure: Res<SlitStructure>,
) {
    println!("{}", slit_structure.separation);
    for (mut transform, slit) in display_query.iter_mut() {
        println!("updating slit separation");
        match slit {
            Slit::LeftSlit => {
                transform.translation = Vec3::new(
                    -(slit_structure.separation - 5.) / 2. - BASELINE_SLIT_WIDTH * 3.,
                    0.,
                    0.1,
                )
            }
            Slit::RightSlit => {
                transform.translation = Vec3::new(
                    (slit_structure.separation - 5.) / 2. + BASELINE_SLIT_WIDTH * 3.,
                    0.,
                    0.1,
                )
            }
        }
    }
}

pub fn update_display_buttons(
    mut display_query: Query<(&mut Text, &SlitControl), With<DisplayInfo>>,
    slit_structure: Res<SlitStructure>,
) {
    for (mut text, slit_type) in display_query.iter_mut() {
        text.sections[0].value = match slit_type {
            SlitControl::Separation => slit_structure.separation.to_string(),
            SlitControl::ScreenDistance => (slit_structure.screen_distance / 100.).to_string(),
            SlitControl::Wavelength => slit_structure.wavelength.to_string(),
            SlitControl::Width => slit_structure.slit_width.to_string(),
        };
    }
}

pub fn increment_sep_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Increment, &SlitControl),
        (Changed<Interaction>, With<Button>),
    >,
    mut slit_structure: ResMut<SlitStructure>,
) {
    for (interaction, mut color, incr, adjust_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                slit_structure.add_val(adjust_type, incr.0);
            }
            _ => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
