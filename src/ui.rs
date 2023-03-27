use crate::component::{DisplayInfo, Increment, SlitControl, SlitStructure};
use bevy::prelude::*;

pub const SLIT_COLOR: Color = Color::rgb(0.43137, 0.27843, 0.17647);
pub const BACKDROUND_COLOR: Color = Color::rgb(0.78824, 0.76863, 0.43137);
pub const CONTROL_BACKGROUND: Color = Color::rgb(0.27843, 0.18431, 0.12157);

pub const NORMAL_BUTTON: Color = Color::rgb(0.83137, 0.33725, 0.14902);
pub const PRESSED_BUTTON: Color = Color::rgb(0.88235, 0.47451, 0.24706);
pub const BUTTON_TEXT_COLOR: Color = BACKDROUND_COLOR;
pub const LABEL_TEXT_COLOR: Color = BACKDROUND_COLOR;

pub fn get_base() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    }
}

pub fn get_slit_controls() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Percent(100.)),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: CONTROL_BACKGROUND.into(),
        ..default()
    }
}

pub fn get_slit_controls_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(95.0), Val::Percent(80.)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: CONTROL_BACKGROUND.into(),
        ..default()
    }
}

pub fn get_control_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(60.0), Val::Px(20.)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            margin: UiRect {
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
                ..default()
            },
            ..Default::default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn get_button_bkgnd() -> ButtonBundle {
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

pub fn get_big_button_bkgnd() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(60.0), Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 1.,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn get_txt(txt: &str, clr: Color, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        txt,
        TextStyle {
            font: asset_server.load("fonts/BigBlue TerminalPlus Nerd Font Complete Mono.TTF"),
            font_size: 8.0,
            color: clr,
        },
    )
    .with_text_alignment(TextAlignment::Center)
}

pub fn get_asterisk_txt(txt: &str, clr: Color, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        txt,
        TextStyle {
            font: asset_server.load("fonts/BigBlue TerminalPlus Nerd Font Complete Mono.TTF"),
            font_size: 8.0,
            color: clr,
        },
    )
    .with_style(Style {
        margin: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
        },
        align_self: AlignSelf::FlexEnd,
        ..default()
    })
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let defaults = SlitStructure::default();

    commands
        .spawn(get_base())
        .insert(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(ZIndex::Global(1))
        .with_children(|parent| {
            //CONTROLS
            parent.spawn(get_slit_controls()).with_children(|parent| {
                parent
                    .spawn(get_slit_controls_container())
                    .with_children(|parent| {
                        // SLIT SEPARATOR
                        parent.spawn(get_txt(
                            "Slit Separation (micrometers)",
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
                                        parent.spawn(get_txt(
                                            "-",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
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
                                        parent.spawn(get_txt(
                                            "+",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
                                    });
                            });
                        // SLIT WIDTH
                        parent.spawn(get_txt(
                            "Slit Width (micrometers)",
                            LABEL_TEXT_COLOR,
                            &asset_server,
                        ));
                        parent
                            .spawn(get_control_container())
                            .with_children(|parent| {
                                parent
                                    .spawn(get_button_bkgnd())
                                    .insert(Increment(-1.))
                                    .insert(SlitControl::Width)
                                    .with_children(|parent| {
                                        parent.spawn(get_txt(
                                            "-",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
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
                                        parent.spawn(get_txt(
                                            "+",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
                                    });
                            });
                        // WAVELENGTH
                        parent.spawn(get_txt("Wavelength* (nm)", LABEL_TEXT_COLOR, &asset_server));
                        parent
                            .spawn(get_control_container())
                            .with_children(|parent| {
                                parent
                                    .spawn(get_button_bkgnd())
                                    .insert(Increment(-10.))
                                    .insert(SlitControl::Wavelength)
                                    .with_children(|parent| {
                                        parent.spawn(get_txt(
                                            "-",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
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
                                    .insert(Increment(10.))
                                    .insert(SlitControl::Wavelength)
                                    .with_children(|parent| {
                                        parent.spawn(get_txt(
                                            "+",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
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
                                        parent.spawn(get_txt(
                                            "-",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
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
                                        parent.spawn(get_txt(
                                            "+",
                                            BUTTON_TEXT_COLOR,
                                            &asset_server,
                                        ));
                                    });
                            });

                        // Light vs Particles
                        parent.spawn(get_txt(
                            "Light vs Particles",
                            LABEL_TEXT_COLOR,
                            &asset_server,
                        ));
                        parent
                            .spawn(get_control_container())
                            .with_children(|parent| {
                                parent
                                    .spawn(get_big_button_bkgnd())
                                    .insert(Increment(0.))
                                    .insert(SlitControl::Input)
                                    .with_children(|parent| {
                                        parent
                                            .spawn(get_txt(
                                                &defaults.toggle_input.to_string(),
                                                BUTTON_TEXT_COLOR,
                                                &asset_server,
                                            ))
                                            .insert(DisplayInfo)
                                            .insert(SlitControl::Input);
                                    });
                            });
                    });

                // asterisk
                parent.spawn(get_asterisk_txt(
                    "*note - colors not exactly to scale",
                    LABEL_TEXT_COLOR,
                    &asset_server,
                ));
            });
        });

    commands.insert_resource(defaults);
}
