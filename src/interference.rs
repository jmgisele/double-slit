use crate::component::{
    InputType, ParticleTimer, ParticlesMaterial, ParticlesMesh, ScreenMaterial, SlitStructure,
};
use crate::particles::{add_particle, add_particles_criteria};
use crate::slit::wavelength_to_rgb;
use crate::{component::LightMaterial, WINDOW_HEIGHT};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct InterferencePlugin;

impl Plugin for InterferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<LightMaterial>::default())
            .add_plugin(Material2dPlugin::<ScreenMaterial>::default())
            .add_plugin(Material2dPlugin::<ParticlesMaterial>::default())
            .init_resource::<ParticleTimer>()
            .add_startup_system(setup_screen.in_base_set(StartupSet::PostStartup))
            .add_system(output_light.run_if(light_criteria))
            .add_system(
                add_particle
                    .run_if(add_particles_criteria)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(output_particles.run_if(output_particles_criteria))
            .add_system(reset_particles.run_if(reset_particles_criteria));
    }
}

pub const BASELINE_Y_SLITS: f32 =
    (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2. - SLIT_SCREEN_HEIGHT * 2.;
pub const BASELINE_X_SLITS: f32 = 150. - SLIT_SCREEN_WIDTH / 2.;
pub const SLIT_SCREEN_WIDTH: f32 = 500.;
pub const SLIT_SCREEN_HEIGHT: f32 = 100.;

pub const SCREEN_COLOR: Color = Color::rgba_linear(0.3, 0.1, 0., 1.0);
pub const BORDER_COLOR: Color = Color::rgba(0.27843, 0.18039, 0.12157, 1.0);

fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_material: ResMut<Assets<LightMaterial>>,
    slit_structure: Res<SlitStructure>,
) {
    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: light_material.add(LightMaterial {
                screen_distance: Vec4::new(slit_structure.screen_distance, 0., 0., 0.),
                separation: Vec4::new(slit_structure.separation, 0., 0., 0.),
                slit_width: Vec4::new(slit_structure.slit_width, 0., 0., 0.),
                wavelength: Vec4::new(slit_structure.wavelength, 0., 0., 0.),
                background_color: SCREEN_COLOR,
                light_color: wavelength_to_rgb(&slit_structure.wavelength),
                border_color: BORDER_COLOR,
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
            ..default()
        })
        .insert(InputType::Light);

    let mesh = ParticlesMesh::default();
    commands.insert_resource(mesh);
}

fn output_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut light_material: ResMut<Assets<LightMaterial>>,
    slit_structure: Res<SlitStructure>,
    mut particles_mesh: ResMut<ParticlesMesh>,
    particles_query: Query<Entity, With<InputType>>,
) {
    for entity in particles_query.iter() {
        commands.entity(entity).despawn();
    }

    particles_mesh.reset_mesh();

    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: light_material.add(LightMaterial {
                screen_distance: Vec4::new(slit_structure.screen_distance, 0., 0., 0.),
                separation: Vec4::new(slit_structure.separation, 0., 0., 0.),
                slit_width: Vec4::new(slit_structure.slit_width, 0., 0., 0.),
                wavelength: Vec4::new(slit_structure.wavelength, 0., 0., 0.),
                background_color: SCREEN_COLOR,
                light_color: wavelength_to_rgb(&slit_structure.wavelength),
                border_color: BORDER_COLOR,
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
            ..default()
        })
        .insert(InputType::Light);
}

pub fn light_criteria(slit_structure: Res<SlitStructure>) -> bool {
    if slit_structure.is_changed() && matches!(slit_structure.toggle_input, InputType::Light) {
        true
    } else {
        false
    }
}

fn output_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut screen_material: ResMut<Assets<ScreenMaterial>>,
    mut particles_material: ResMut<Assets<ParticlesMaterial>>,
    slit_structure: Res<SlitStructure>,
    particles_mesh: Res<ParticlesMesh>,
    light_query: Query<Entity, With<InputType>>,
) {
    for entity in light_query.iter() {
        commands.entity(entity).despawn();
    }
    let y = (WINDOW_HEIGHT - SLIT_SCREEN_HEIGHT) / 2.;

    // screen
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SLIT_SCREEN_WIDTH, SLIT_SCREEN_HEIGHT, 0.).into())
                .into(),
            material: screen_material.add(ScreenMaterial {
                color: SCREEN_COLOR,
                border: BORDER_COLOR,
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.)),
            ..default()
        })
        .insert(InputType::Particles);

    let particles_mesh = get_particles_mesh(&particles_mesh, &slit_structure);
    // particles
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(particles_mesh.into()).into(),
            material: particles_material.add(ParticlesMaterial {
                color: wavelength_to_rgb(&slit_structure.wavelength),
            }),
            transform: Transform::from_translation(Vec3::new(BASELINE_X_SLITS, y, 0.1)),
            ..default()
        })
        .insert(InputType::Particles);
}

pub fn output_particles_criteria(
    slit_structure: Res<SlitStructure>,
    mesh: Res<ParticlesMesh>,
) -> bool {
    if mesh.is_changed() && matches!(slit_structure.toggle_input, InputType::Particles) {
        true
    } else {
        false
    }
}

pub fn get_particles_mesh(particles_mesh: &ParticlesMesh, slit: &SlitStructure) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::PointList);

    let _old_mesh = vec![[-20., -20., 0.], [1., 1., 0.], [0., 1., 0.], [1., 0., 0.]];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, particles_mesh.0.clone());
    // y: 50 to -50
    // x : -250 to 250

    // normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0., 1., 0.]; particles_mesh.0.len()],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; particles_mesh.0.len()]);

    // color comes from wavelength
    let color = wavelength_to_rgb(&slit.wavelength);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[color.r(), color.g(), color.b(), 1.]; particles_mesh.0.len()],
    );

    mesh
}

pub fn reset_particles(mut particles_mesh: ResMut<ParticlesMesh>) {
    particles_mesh.reset_mesh();
}

pub fn reset_particles_criteria(slit_structure: Res<SlitStructure>) -> bool {
    if slit_structure.is_changed() && matches!(slit_structure.toggle_input, InputType::Particles) {
        true
    } else {
        false
    }
}
