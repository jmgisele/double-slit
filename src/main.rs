use bevy::{
    prelude::{App, PluginGroup},
    window::{WindowDescriptor, WindowPlugin},
    winit::WinitSettings,
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use slit::SlitPlugin;
use window::setup_camera;

mod slit;
mod ui;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Double Slit Experiment".to_string(),
                width: 800.,
                height: 400.,
                ..Default::default()
            },
            ..Default::default()
        }))
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup_camera)
        .add_plugin(SlitPlugin)
        .run();
}
