use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use interference::InterferencePlugin;
use slit::SlitPlugin;
use ui::BACKDROUND_COLOR;

mod component;
mod interference;
mod slit;
mod ui;

// keep these at a 1:2 ratio or the physics math doesn't work out
// if you change my ratio. change it in the shaders too!

pub const WINDOW_HEIGHT: f32 = 375.;
pub const WINDOW_WIDTH: f32 = 750.;

const TIMESTEP_60_PER_SECOND: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Double Slit Experiment".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(BACKDROUND_COLOR))
        .add_startup_system(setup_camera)
        .add_plugin(SlitPlugin)
        .add_plugin(InterferencePlugin)
        .insert_resource(FixedTime::new_from_secs(TIMESTEP_60_PER_SECOND))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}
