use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
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

//
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

#[derive(Resource)]
pub struct ParticleTimer(pub Timer);

impl Default for ParticleTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.005, TimerMode::Repeating))
    }
}

// UI

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

// SHADERS
#[derive(Resource, Debug)]
pub struct ParticlesMesh(pub Vec<[f32; 3]>);

impl ParticlesMesh {
    pub fn add_particle(&mut self, coords: [f32; 3]) {
        self.0.push(coords);
        // println!("{:#?}", self.0);
    }

    pub fn reset_mesh(&mut self) {
        self.0 = vec![];
    }
}

impl Default for ParticlesMesh {
    fn default() -> Self {
        ParticlesMesh(vec![])
    }
}

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
pub struct ScreenMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(1)]
    pub border: Color,
}

impl Material2d for ScreenMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screen.wgsl".into()
    }
}
#[derive(AsBindGroup, Debug, TypeUuid, Clone)]
#[uuid = "ff3c172b-415f-41f4-811d-aaca03c5d103"]
pub struct ParticlesMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for ParticlesMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/particles.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/particles.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
