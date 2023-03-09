use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}
