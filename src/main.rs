use bevy::{
    prelude::{App, ClearColor, Color, PluginGroup},
    window::{WindowDescriptor, WindowPlugin},
    winit::WinitSettings,
    DefaultPlugins,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use interference::InterferencePlugin;
use slit::SlitPlugin;
use window::setup_camera;

mod component;
mod interference;
mod slit;
mod ui;
mod window;

pub const WINDOW_HEIGHT: f32 = 400.;
pub const WINDOW_WIDTH: f32 = 800.;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Double Slit Experiment".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..Default::default()
            },
            ..Default::default()
        }))
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::rgb(0.80000, 1.00000, 0.80000)))
        .add_startup_system(setup_camera)
        .add_plugin(SlitPlugin)
        .add_plugin(InterferencePlugin)
        .run();
}
