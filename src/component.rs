use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Resource, Debug)]
pub struct SlitStructure {
    pub separation: f32,      // centimeters
    pub slit_width: f32,      // milimeters
    pub wavelength: f32,      // nanometers
    pub screen_distance: f32, // centimeters
}

impl Default for SlitStructure {
    fn default() -> Self {
        SlitStructure {
            separation: 50.,       // micrometers
            slit_width: 5.,        // micrometers
            wavelength: 500.,      // nanometers
            screen_distance: 100., // centimeters
        }
    }
}

impl SlitStructure {
    pub fn add_val(&mut self, opt: &SlitControl, val: f32) {
        println!("{:#?}", self);
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
pub enum Slit {
    LeftSlit,
    RightSlit,
}

#[derive(Component)]
pub struct SlitScreen;

#[derive(Component)]
pub struct Light;

#[derive(Component)]
pub struct DisplayInfo;

#[derive(Component)]
pub struct Increment(pub f32);

// BASIC SHADER
#[derive(AsBindGroup, Debug, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub separation: f32,
    #[uniform(1)]
    pub slit_width: f32,
    #[uniform(2)]
    pub wavelength: f32,
    #[uniform(3)]
    pub screen_distance: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/my_material.wgsl".into()
    }
}
