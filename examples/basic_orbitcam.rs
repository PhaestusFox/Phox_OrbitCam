use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use rand::Rng;
use phox_orbitcam::{OrbitCam, OrbitCamPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, bevy_editor_pls::EditorPlugin::new()))
    .add_systems(Startup, (spawn_cam, spawn_map))
    .add_plugins(OrbitCamPlugin);
    app.run();
}

fn spawn_cam(
    mut commands: Commands,
) {
    commands.spawn((Camera3dBundle::default(), OrbitCam {
        up: KeyCode::KeyW,
        down: KeyCode::KeyS,
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
        yaw_left: KeyCode::KeyQ,
        yaw_right: KeyCode::KeyE,
        speed: 100.,
        yaw_speed: 1.,
        min_zoom: 5.,
        max_zoom: 100.,
        max_pitch: 80.0f32.to_radians(),
        min_pitch: 10.0f32.to_radians(),
        pitch_up: KeyCode::KeyZ,
        pitch_down: KeyCode::KeyX,
        pitch_speed: 1.,
        mouse_move: phox_orbitcam::MouseControle::Enabled { invert_y: false, active_key: None, active_button: Some(MouseButton::Right), sensitivity: 0.01 },
    }));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE * 1000.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
const CHUNK_SIZE: i32 = 64;
fn spawn_map(
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use noise::NoiseFn;
    let noise: noise::Fbm<noise::OpenSimplex> = noise::Fbm::new(420);
    let mut nodes = Vec::new();
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            nodes.push(Vec3::new(x as f32 * 3., noise.get([x as f64 * 1.14159, z as f64 * 1.14159]) as f32 * 3., z as f32 * 3.));
        }
    }

    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    let mut indices = Vec::new();
    for x in 0..(CHUNK_SIZE as u16 - 1) {
        for z in 0..(CHUNK_SIZE as u16 - 1) {
            indices.push(x + (z * CHUNK_SIZE as u16));
            indices.push(x + (z * CHUNK_SIZE as u16) + 1);
            indices.push(x + (z + 1) * CHUNK_SIZE as u16);
            indices.push(x + (z * CHUNK_SIZE as u16) + 1);
            indices.push(x + (z + 1) * CHUNK_SIZE as u16 + 1);
            indices.push(x + (z + 1) * CHUNK_SIZE as u16);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, nodes);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::new(rand::thread_rng().gen(), 1., rand::thread_rng().gen()); (CHUNK_SIZE * CHUNK_SIZE) as usize]);
    mesh.insert_indices(bevy::render::mesh::Indices::U16(indices));

    commands.spawn(PbrBundle {
        mesh: assets.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..Default::default()
        }),
        ..Default::default()});
}