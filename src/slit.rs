use crate::{
    component::{DisplayInfo, Increment, Light, Slit, SlitControl, SlitScreen, SlitStructure},
    interference::{BASELINE_Y_SLITS, SLIT_SCREEN_HEIGHT},
    ui::{setup_ui, BACKDROUND_COLOR, NORMAL_BUTTON, PRESSED_BUTTON, SLIT_COLOR},
};
use bevy::math::{f32::Quat, vec4};
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
                    .with_system(interpolate_light_color),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(update_display_buttons),
            );
    }
}

const BASELINE_SLIT_WIDTH: f32 = 1.5;
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
            material: materials.add(ColorMaterial::from(SLIT_COLOR)),
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
                    material: materials.add(ColorMaterial::from(BACKDROUND_COLOR)),
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
                    material: materials.add(ColorMaterial::from(BACKDROUND_COLOR)),
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

pub fn interpolate_light_color(
    display_query: Query<&Handle<ColorMaterial>, With<Light>>,
    slit_structure: Res<SlitStructure>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for handle in display_query.iter() {
        let mut color_mat = materials.get_mut(&handle).unwrap();

        let wavelength = slit_structure.wavelength;

        let min_visible = 380.;
        let max_visible: f32 = 780.;

        if wavelength <= min_visible || wavelength >= max_visible {
            color_mat.color = Color::BLACK;
            return;
        } else {
            // let wave_range: f32 = MAX_WAVELENGTH - min_visible;
            // let t: f32 = (wavelength - min_visible) / wave_range;
            // color_mat.color = lerp_hsv(t);
            color_mat.color = wavelength_to_rgb(&wavelength);
        }
    }
}

pub fn wavelength_to_rgb(lambda: &f32) -> Color {
    // adapted from https://codepen.io/pen?editors=0010
    // thx
    let mut red;
    let mut blue;
    let mut green;
    let factor;
    let wavelength = lambda.clone();

    const INTENSITYMAX: f32 = 1.;
    const GAMMA: f32 = 0.8;

    if (wavelength >= 380.) && (wavelength < 440.) {
        red = -(wavelength - 440.) / (440. - 380.);
        green = 0.0;
        blue = 1.0;
    } else if (wavelength >= 440.) && (wavelength < 490.) {
        red = 0.0;
        green = (wavelength - 440.) / (490. - 440.);
        blue = 1.0;
    } else if (wavelength >= 490.) && (wavelength < 510.) {
        red = 0.0;
        green = 1.0;
        blue = -(wavelength - 510.) / (510. - 490.);
    } else if (wavelength >= 510.) && (wavelength < 580.) {
        red = (wavelength - 510.) / (580. - 510.);
        green = 1.0;
        blue = 0.0;
    } else if (wavelength >= 580.) && (wavelength < 645.) {
        red = 1.0;
        green = -(wavelength - 645.) / (645. - 580.);
        blue = 0.0;
    } else if (wavelength >= 645.) && (wavelength < 781.) {
        red = 1.0;
        green = 0.0;
        blue = 0.0;
    } else {
        red = 0.0;
        green = 0.0;
        blue = 0.0;
    };
    // Let the intensity fall off near the vision limits
    if (wavelength >= 380.) && (wavelength < 420.) {
        factor = 0.3 + 0.7 * (wavelength - 380.) / (420. - 380.);
    } else if (wavelength >= 420.) && (wavelength < 701.) {
        factor = 1.0;
    } else if (wavelength >= 701.) && (wavelength < 781.) {
        factor = 0.3 + 0.7 * (780. - wavelength) / (780. - 700.);
    } else {
        factor = 0.0;
    };

    if red != 0. {
        red = INTENSITYMAX * f32::powf(red * factor, GAMMA);
    }
    if green != 0. {
        green = INTENSITYMAX * f32::powf(green * factor, GAMMA);
    }
    if blue != 0. {
        blue = INTENSITYMAX * f32::powf(blue * factor, GAMMA);
    }

    return Color::rgb(red, green, blue);
}

pub fn _lerp_hsv(t: f32) -> Color {
    let a = vec4(0., 100., 50., 1.0);
    let b = vec4(285., 100., 50., 1.);

    let d = b.x - a.x;

    // a.x = a.x + 360.;
    let h = b.x - t * d;

    return Color::Hsla {
        hue: h,
        saturation: 1.,
        lightness: 0.5,
        alpha: 1.,
    };
}

pub fn update_slit_width(
    mut display_query: Query<&mut Transform, With<Slit>>,
    slit_structure: Res<SlitStructure>,
) {
    for mut transform in display_query.iter_mut() {
        transform.scale = Vec3::new(
            BASELINE_SLIT_WIDTH - (5. - slit_structure.slit_width) / 5.,
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
        transform.translation = Vec3::new(
            BASELINE_SLIT_SCREEN_X + (100. - slit_structure.screen_distance) / 5.,
            BASELINE_Y_SLITS + (100. - slit_structure.screen_distance) / 5.,
            0.,
        );
    }
}

pub fn update_slit_separation(
    mut display_query: Query<(&mut Transform, &Slit)>,
    slit_structure: Res<SlitStructure>,
) {
    for (mut transform, slit) in display_query.iter_mut() {
        match slit {
            Slit::LeftSlit => {
                transform.translation = Vec3::new(
                    -(slit_structure.separation - 50.) / 2. - BASELINE_SLIT_WIDTH * 3.,
                    0.,
                    0.1,
                )
            }
            Slit::RightSlit => {
                transform.translation = Vec3::new(
                    (slit_structure.separation - 50.) / 2. + BASELINE_SLIT_WIDTH * 3.,
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
            SlitControl::Input => slit_structure.toggle_input.to_string(),
        };
    }
}

pub fn increment_sep_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Increment, &SlitControl)>,
    mut slit_structure: ResMut<SlitStructure>,
) {
    for (interaction, mut color, incr, adjust_type) in &mut interaction_query {
        match *interaction {
            // dreaming about the day bevy adds a "plus" state
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
