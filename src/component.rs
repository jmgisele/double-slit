use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Resource, Debug)]
pub struct SlitStructure {
    pub separation: f32,
    pub slit_width: f32,
    pub wavelength: f32,
    pub screen_distance: f32,
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

pub const MAX_WAVELENGTH: f32 = 750.;
pub const MIN_WAVELENGTH: f32 = 250.;

impl SlitStructure {
    pub fn add_val(&mut self, opt: &SlitControl, val: f32) {
        match opt {
            SlitControl::Separation => {
                let new = self.separation + val;
                if new >= 1. && new <= 100. {
                    self.separation += val
                }
            }
            SlitControl::ScreenDistance => {
                let new = self.screen_distance + val;
                if new >= 20. && new <= 200. {
                    self.screen_distance += val
                }
            }
            SlitControl::Wavelength => {
                let new = self.wavelength + val;
                if new <= MAX_WAVELENGTH && new >= MIN_WAVELENGTH {
                    self.wavelength += val
                }
            }
            SlitControl::Width => {
                let new = self.slit_width + val;
                if new >= 1. && new <= 15. {
                    self.slit_width += val
                }
            }
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
pub struct InterferenceMaterial {
    #[uniform(0)]
    pub separation: f32,
    #[uniform(0)]
    pub _wasm_padding_0: Vec3,
    //
    #[uniform(1)]
    pub slit_width: f32,
    #[uniform(1)]
    pub _wasm_padding_1: Vec3,
    //
    #[uniform(2)]
    pub wavelength: f32,
    #[uniform(2)]
    pub _wasm_padding_2: Vec3,
    //
    #[uniform(3)]
    pub screen_distance: f32,
    #[uniform(3)]
    pub _wasm_padding_3: Vec3,
    //
    #[uniform(4)]
    pub background_color: Color,
    //
    #[uniform(5)]
    pub border_color: Color,
}

impl Default for InterferenceMaterial {
    fn default() -> Self {
        InterferenceMaterial {
            separation: 0.,
            _wasm_padding_0: Vec3::ZERO,
            slit_width: 0.,
            _wasm_padding_1: Vec3::ZERO,
            wavelength: 0.,
            _wasm_padding_2: Vec3::ZERO,
            screen_distance: 0.,
            _wasm_padding_3: Vec3::ZERO,
            background_color: Color::WHITE,
            border_color: Color::WHITE,
        }
    }
}

impl Material2d for InterferenceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/interference.wgsl".into()
    }
}
