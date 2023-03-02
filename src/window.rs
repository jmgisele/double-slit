//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.
#![allow(clippy::type_complexity)]
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}
