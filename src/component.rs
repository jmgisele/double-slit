use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

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
    pub fn add_val(&mut self, opt: &SlitControl, val: f32) {
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
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/my_material.wgsl".into()
    }
}
