use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.30196, 0.00000, 0.30196);
pub const PRESSED_BUTTON: Color = Color::rgb(0.30196, 0.00000, 0.30196);
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(1.00000, 0.90196, 1.00000);
pub const LABEL_TEXT_COLOR: Color = Color::rgb(0.30196, 0.00000, 0.30196);

pub fn get_slit(color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(1.0), Val::Px(10.0)),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

pub fn get_separator(sep: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(sep / 12.), Val::Px(10.0)),
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    }
}

pub fn get_base() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgb(0.80000, 1.00000, 0.80000).into(),
        ..default()
    }
}

pub fn get_display_box() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..default()
    }
}

pub fn get_interference_screen() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..default()
    }
}

pub fn get_separation_screen_slit() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.).into(),
        ..default()
    }
}

pub fn get_slit_screen() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..default()
    }
}

pub fn get_slit_controls() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.00000, 0.60000, 1.00000).into(),
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

pub fn get_txt(txt: &str, clr: Color, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        txt,
        TextStyle {
            font: asset_server.load("fonts/BigBlue TerminalPlus Nerd Font Complete Mono.TTF"),
            font_size: 8.0,
            color: clr,
        },
    )
}
