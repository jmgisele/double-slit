use crate::ui::{
    get_base, get_button_bkgnd, get_control_container, get_display_box, get_interference_screen,
    get_separation_screen_slit, get_separator, get_slit, get_slit_controls, get_slit_screen,
    get_txt, BUTTON_TEXT_COLOR, LABEL_TEXT_COLOR, NORMAL_BUTTON, PRESSED_BUTTON,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*};
pub struct SlitPlugin;
impl Plugin for SlitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_slits)
            .add_system(increment_sep_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(update_display),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(update_display_criteria)
                    .with_system(update_display_buttons),
            );
    }
}

#[derive(Resource)]
pub struct SlitStructure {
    pub separation: f32,
    pub slit_width: f32,
    pub wavelength: f32,
    pub screen_distance: f32,
}

impl Default for SlitStructure {
    fn default() -> Self {
        SlitStructure {
            separation: 5.,        // centimeters
            slit_width: 10.,       // milimeters
            wavelength: 500.,      // nanometers
            screen_distance: 100., // centimeters
        }
    }
}

impl SlitStructure {
    fn add_val(&mut self, opt: &SlitControl, val: f32) {
        match opt {
            SlitControl::Separation => self.separation += val,
            SlitControl::ScreenDistance => self.screen_distance += val,
            SlitControl::Wavelength => self.wavelength += val,
            SlitControl::Width => self.slit_width += val,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum SlitControl {
    Separation,
    Width,
    Wavelength,
    ScreenDistance,
}

#[derive(Component)]
pub struct DisplayInfo;

#[derive(Component)]
pub struct Adjustable;

#[derive(Component)]
pub struct Increment(f32);

fn setup_slits(mut commands: Commands, asset_server: Res<AssetServer>) {
    let defaults = SlitStructure::default();

    commands.spawn(get_base()).with_children(|parent| {
        parent.spawn(get_display_box()).with_children(|parent| {
            // interference display
            parent.spawn(get_interference_screen());
            // screen : slit separator
            parent
                .spawn(get_separation_screen_slit())
                .insert(SlitControl::ScreenDistance)
                .insert(Adjustable);
            parent.spawn(get_slit_screen()).with_children(|parent| {
                // left slit
                parent
                    .spawn(get_slit(Color::rgb(1., 1., 1.)))
                    .insert(SlitControl::Width)
                    .insert(Adjustable);
                // separator
                parent
                    .spawn(get_separator(defaults.separation))
                    .insert(SlitControl::Separation)
                    .insert(Adjustable);
                // right slit
                parent
                    .spawn(get_slit(Color::rgb(1., 1., 1.)))
                    .insert(SlitControl::Width)
                    .insert(Adjustable);
            });
        });

        //CONTROLS
        parent.spawn(get_slit_controls()).with_children(|parent| {
            // SLIT SEPARATOR
            parent.spawn(get_txt(
                "Slit Separation (cm)",
                LABEL_TEXT_COLOR,
                &asset_server,
            ));
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-1.))
                        .insert(SlitControl::Separation)
                        .with_children(|parent| {
                            parent.spawn(get_txt("-", BUTTON_TEXT_COLOR, &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_txt(
                                &defaults.separation.to_string(),
                                BUTTON_TEXT_COLOR,
                                &asset_server,
                            ))
                            .insert(SlitControl::Separation)
                            .insert(DisplayInfo);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(1.))
                        .insert(SlitControl::Separation)
                        .with_children(|parent| {
                            parent.spawn(get_txt("+", BUTTON_TEXT_COLOR, &asset_server));
                        });
                });
            // SLIT WIDTH
            parent.spawn(get_txt("Slit Width (mm)", LABEL_TEXT_COLOR, &asset_server));
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-1.))
                        .insert(SlitControl::Width)
                        .with_children(|parent| {
                            parent.spawn(get_txt("-", BUTTON_TEXT_COLOR, &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_txt(
                                &defaults.slit_width.to_string(),
                                BUTTON_TEXT_COLOR,
                                &asset_server,
                            ))
                            .insert(SlitControl::Width)
                            .insert(DisplayInfo);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(1.))
                        .insert(SlitControl::Width)
                        .with_children(|parent| {
                            parent.spawn(get_txt("+", BUTTON_TEXT_COLOR, &asset_server));
                        });
                });
            // WAVELENGTH
            parent.spawn(get_txt("Wavelength (nm)", LABEL_TEXT_COLOR, &asset_server));
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-25.))
                        .insert(SlitControl::Wavelength)
                        .with_children(|parent| {
                            parent.spawn(get_txt("-", BUTTON_TEXT_COLOR, &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_txt(
                                &defaults.slit_width.to_string(),
                                BUTTON_TEXT_COLOR,
                                &asset_server,
                            ))
                            .insert(SlitControl::Wavelength)
                            .insert(DisplayInfo);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(25.))
                        .insert(SlitControl::Wavelength)
                        .with_children(|parent| {
                            parent.spawn(get_txt("+", BUTTON_TEXT_COLOR, &asset_server));
                        });
                });

            // DISTANCE TO SCREEN
            parent.spawn(get_txt(
                "Distance to Screen (m)",
                LABEL_TEXT_COLOR,
                &asset_server,
            ));
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-10.))
                        .insert(SlitControl::ScreenDistance)
                        .with_children(|parent| {
                            parent.spawn(get_txt("-", BUTTON_TEXT_COLOR, &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_txt(
                                &defaults.slit_width.to_string(),
                                BUTTON_TEXT_COLOR,
                                &asset_server,
                            ))
                            .insert(SlitControl::ScreenDistance)
                            .insert(DisplayInfo);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(10.))
                        .insert(SlitControl::ScreenDistance)
                        .with_children(|parent| {
                            parent.spawn(get_txt("+", BUTTON_TEXT_COLOR, &asset_server));
                        });
                });
        });
    });

    commands.insert_resource(defaults);
}

pub fn update_display_criteria(slit_structure: Res<SlitStructure>) -> ShouldRun {
    if slit_structure.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
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

pub fn update_display(
    mut display_query: Query<(&mut Style, &SlitControl), With<Adjustable>>,
    slit_structure: Res<SlitStructure>,
) {
    for (mut style, slit_type) in display_query.iter_mut() {
        match slit_type {
            SlitControl::Separation => {
                style.size.width = Val::Px(f32::floor(5. * slit_structure.separation));
            }
            SlitControl::ScreenDistance => {
                style.size.height = Val::Px(f32::floor(0.5 * slit_structure.screen_distance));
            }
            SlitControl::Wavelength => {
                todo!()
            }
            SlitControl::Width => {
                style.size.width = Val::Px(f32::floor(0.5 * slit_structure.slit_width));
            }
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
