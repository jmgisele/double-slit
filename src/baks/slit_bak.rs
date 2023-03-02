use bevy::prelude::*;

pub struct SlitPlugin;
impl Plugin for SlitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_slits)
            .add_system(increment_sep_system);
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
            separation: 60.,
            slit_width: 6.,
            wavelength: 500.,
            screen_distance: 1.,
        }
    }
}

#[derive(Component)]
enum SlitDisplay {
    Separation,
    Width,
    Wavelength,
    SlitDistance,
}

#[derive(Component)]
enum SlitAdjust {
    Separation,
    Width,
    Wavelength,
    SlitDistance,
}

#[derive(Component)]
pub struct Increment(f32);

fn get_slit(color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(1.0), Val::Px(10.0)),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

fn get_separator(sep: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(sep / 12.), Val::Px(10.0)),
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    }
}

fn get_base() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgb(0.80000, 1.00000, 0.80000).into(),
        ..default()
    }
}

fn get_slit_screen() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(100.0)),
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..default()
    }
}

fn get_slit_controls() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.00000, 0.60000, 1.00000).into(),
        ..default()
    }
}

fn get_control_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(60.0), Val::Px(20.)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn get_button_bkgnd() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(20.0), Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 1.,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn get_button_txt(txt: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        txt,
        TextStyle {
            font: asset_server.load("fonts/BigBlue TerminalPlus Nerd Font Complete Mono.TTF"),
            font_size: 12.0,
            color: Color::rgb(1.00000, 0.90196, 1.00000),
        },
    )
}

fn setup_slits(mut commands: Commands, asset_server: Res<AssetServer>) {
    let defaults = SlitStructure::default();

    commands.spawn(get_base()).with_children(|parent| {
        parent.spawn(get_slit_screen()).with_children(|parent| {
            // left slit
            parent
                .spawn(get_slit(Color::rgb(1., 1., 1.)))
                .insert(SlitAdjust::Width);
            // separator
            parent
                .spawn(get_separator(defaults.separation))
                .insert(SlitAdjust::Separation);
            // right slit
            parent
                .spawn(get_slit(Color::rgb(1., 1., 1.)))
                .insert(SlitAdjust::Width);
        });
        parent.spawn(get_slit_controls()).with_children(|parent| {
            // SLIT SEPARATOR
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-12.))
                        .with_children(|parent| {
                            parent.spawn(get_button_txt("-", &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_button_txt(
                                &defaults.separation.to_string(),
                                &asset_server,
                            ))
                            .insert(SlitDisplay::Separation);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(12.))
                        .with_children(|parent| {
                            parent.spawn(get_button_txt("+", &asset_server));
                        });
                });
            // SLIT WIDTH
            parent
                .spawn(get_control_container())
                .with_children(|parent| {
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(-1.))
                        .with_children(|parent| {
                            parent.spawn(get_button_txt("-", &asset_server));
                        });
                    parent.spawn(get_button_bkgnd()).with_children(|parent| {
                        parent
                            .spawn(get_button_txt(
                                &defaults.slit_width.to_string(),
                                &asset_server,
                            ))
                            .insert(SlitDisplay::Width);
                    });
                    parent
                        .spawn(get_button_bkgnd())
                        .insert(Increment(1.))
                        .with_children(|parent| {
                            parent.spawn(get_button_txt("+", &asset_server));
                        });
                });
        });
    });
}

pub fn increment_sep_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Increment),
        (Changed<Interaction>, With<Button>),
    >,
    mut separator_query: Query<(&mut SlitSeparation, &mut Style)>,
    mut display_query: Query<&mut Text, With<SlitDisplay>>,
) {
    if let Ok(slit) = separator_query.get_single_mut() {
        let (mut sep, mut style) = slit;
        let mut display = display_query.get_single_mut().unwrap();

        for (interaction, mut color, incr) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                    sep.0 = sep.0 + incr.0;
                    let mut size = &mut style.size;
                    size.width = Val::Px(f32::floor(sep.0 / 12.));
                    display.sections[0].value = sep.to_string();
                }
                _ => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.30196, 0.00000, 0.30196);
const PRESSED_BUTTON: Color = Color::rgb(0.30196, 0.00000, 0.30196);
