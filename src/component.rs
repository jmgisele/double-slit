use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Debug, Component)]
pub enum InputType {
    Light,
    Particles,
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Resource, Debug)]
pub struct SlitStructure {
    pub separation: f32,
    pub slit_width: f32,
    pub wavelength: f32,
    pub screen_distance: f32,
    pub toggle_input: InputType,
}

impl Default for SlitStructure {
    fn default() -> Self {
        SlitStructure {
            separation: 50.,       // micrometers
            slit_width: 5.,        // micrometers
            wavelength: 500.,      // nanometers
            screen_distance: 100., // centimeters
            toggle_input: InputType::Light,
        }
    }
}

pub const MAX_WAVELENGTH: f32 = 800.;
pub const MIN_WAVELENGTH: f32 = 200.;

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
            SlitControl::Input => {
                self.toggle_input = match self.toggle_input {
                    InputType::Light => InputType::Particles,
                    InputType::Particles => InputType::Light,
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
    Input,
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

#[derive(AsBindGroup, Debug, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct LightMaterial {
    // have to be vec4s so they're all padded correctly or wasm won't compile
    #[uniform(0)]
    pub separation: Vec4,
    #[uniform(1)]
    pub slit_width: Vec4,
    #[uniform(2)]
    pub wavelength: Vec4,
    #[uniform(3)]
    pub screen_distance: Vec4,
    //
    #[uniform(4)]
    pub background_color: Color,
    #[uniform(5)]
    pub light_color: Color,
    #[uniform(6)]
    pub border_color: Color,
}

impl Material2d for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/light.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, TypeUuid, Clone)]
#[uuid = "2d1e7208-7dd0-4834-8e0a-163ed6b06aa8"]
pub struct ParticlesMaterial {
    // have to be vec4s so they're all padded correctly or wasm won't compile
    #[uniform(0)]
    pub separation: Vec4,
    #[uniform(1)]
    pub slit_width: Vec4,
    #[uniform(2)]
    pub wavelength: Vec4,
    #[uniform(3)]
    pub screen_distance: Vec4,
    //
    #[uniform(4)]
    pub background_color: Color,
    #[uniform(5)]
    pub light_color: Color,
    #[uniform(6)]
    pub border_color: Color,
}

impl Material2d for ParticlesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles.wgsl".into()
    }
}
