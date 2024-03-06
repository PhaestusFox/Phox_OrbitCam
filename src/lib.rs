use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};

#[derive(Component)]
pub struct OrbitCam {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub yaw_left: KeyCode,
    pub yaw_right: KeyCode,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub pitch_up: KeyCode,
    pub pitch_down: KeyCode,
    pub pitch_speed: f32,
    pub speed: f32,
    pub yaw_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    /// Some is enabled bool is invert Y, MouseButton is *Active* Button
    pub mouse_move: MouseControle,
}

#[derive(Component, Reflect)]
struct CameraZoom {
    zoom: f32,
    last: Vec3,
    offset: Vec3,
}

#[derive(Component)]
struct CameraPitch(f32);

#[derive(Component)]
struct CameraYaw(f32);

pub struct OrbitCamPlugin;

impl Plugin for OrbitCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_cameras, zoom_camera, yaw_camera, pitch_camera, mouse_move).before(update_camera_postition))
        .add_systems(Update, update_camera_postition)
        .add_systems(First, (add_zoom, add_pitch, add_yaw))
        .add_systems(Startup, spawn_debug_cube)
        .register_type::<CameraZoom>();
    }
}

fn add_zoom(
    new_cameras: Query<(Entity, &OrbitCam), (Added<OrbitCam>, Without<CameraZoom>)>,
    mut commands: Commands,
) {
    for (entity, cam) in &new_cameras {
        commands.entity(entity).insert(CameraZoom {
            zoom: (cam.max_zoom - cam.min_zoom) / 2. + cam.min_zoom,
            last: Vec3::ZERO,
            offset: Vec3::ZERO,
        });
    }
}

fn add_pitch(
    new_cameras: Query<Entity, (Added<OrbitCam>, Without<CameraPitch>)>,
    mut commands: Commands,
) {
    for entity in &new_cameras {
        commands.entity(entity).insert(CameraPitch(-3.1415/4.));
    }
}

fn add_yaw(
    new_cameras: Query<Entity, (Added<OrbitCam>, Without<CameraYaw>)>,
    mut commands: Commands,
) {
    for entity in &new_cameras {
        commands.entity(entity).insert(CameraYaw(0.));
    }
}

fn move_cameras(
    mut cameras: Query<(&mut Transform, &OrbitCam)>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut camera, config) in &mut cameras {
        let mut delta = Vec3::ZERO;
        let mut right = *camera.right();
        right.y = 0.;
        right = right.normalize();
        let mut forward = *camera.forward();
        forward.y = 0.;
        forward = forward.normalize();
        if inputs.pressed(config.up) {
            delta += forward;
        }
        if inputs.pressed(config.down) {
            delta -= forward;
        }
        if inputs.pressed(config.right) {
            delta += right;
        }
        if inputs.pressed(config.left) {
            delta -= right;
        }

        camera.translation += delta * time.delta_seconds() * config.speed;
    }
}

fn zoom_camera(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut cameras: Query<(&mut CameraZoom, &OrbitCam)>
) {
    let delta: f32 = mouse_wheel.read().map(|e| e.y).sum();
    for (mut zoom, config) in &mut cameras {
        zoom.zoom += delta;
        zoom.zoom = zoom.zoom.clamp(config.min_zoom, config.max_zoom);
    }
}

#[derive(Component)]
struct DebugCube;

fn update_camera_postition(
    mut cameras: Query<(&mut Transform, &mut CameraZoom, &CameraPitch, &CameraYaw, &OrbitCam)>,
    mut debug: Query<&mut Transform, (With<DebugCube>, Without<CameraPitch>)>
) {
    for (mut pos, mut zoom, pitch, yaw, config) in &mut cameras {
        let moved = pos.translation - zoom.last;
        zoom.offset += moved;

        let mut start = Transform::from_translation(zoom.offset + Vec3::Z * zoom.zoom);
        let rotation = Quat::from_rotation_y(yaw.0) * Quat::from_rotation_x(-pitch.0);
        start.rotate_around(zoom.offset, rotation);
        start.look_at(zoom.offset, Vec3::Y);
        for mut cube in &mut debug {
            cube.translation = zoom.offset;
        }
        zoom.last = start.translation;

        *pos = start;
    }
}

fn yaw_camera(
    inputs: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&mut CameraYaw, &OrbitCam)>,
    time: Res<Time>,
) {
    for (mut yaw, config) in &mut cameras {
        if inputs.pressed(config.yaw_left) {
            yaw.0 += config.yaw_speed * time.delta_seconds();
        }
        if inputs.pressed(config.yaw_right) {
            yaw.0 -= config.yaw_speed * time.delta_seconds();
        }
    }
}

fn pitch_camera(
    inputs: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&mut CameraPitch, &OrbitCam)>,
    time: Res<Time>,
) {
    for (mut pitch, config) in &mut cameras {
        if inputs.pressed(config.pitch_up) {
            pitch.0 += config.pitch_speed * time.delta_seconds();
        }
        if inputs.pressed(config.pitch_down) {
            pitch.0 -= config.pitch_speed * time.delta_seconds();
        }
        pitch.0 = pitch.0.clamp(config.min_pitch, config.max_pitch);
    }
}


fn spawn_debug_cube(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshs.add(Sphere::new(1.)),
        ..Default::default()
    }, DebugCube));
}

fn mouse_move(
    mut mouse_movement: EventReader<MouseMotion>,
    mut cameras: Query<(&mut CameraPitch, &mut CameraYaw, &OrbitCam)>,
    input: Res<ButtonInput<MouseButton>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut delta = Vec2::ZERO;
    for movement in mouse_movement.read() {
        delta += movement.delta;
    }

    for (mut pitch, mut yaw, config) in &mut cameras {
        let MouseControle::Enabled {
            invert_y,
            active_key,
            active_button,
            sensitivity,
        } = config.mouse_move else {continue;};
        let mut run = false;
        if let Some(mouse) = active_button {
            run = input.pressed(mouse);
        }
        if let Some(key) = active_key {
            run |= input_keyboard.pressed(key);
        }
        if !run {continue;}
        pitch.0 += if invert_y {-delta.y} else {delta.y} * sensitivity;
        yaw.0 += delta.x * sensitivity;
    }
}

pub enum MouseControle {
    Disabled,
    Enabled {
        invert_y: bool,
        active_key: Option<KeyCode>,
        active_button: Option<MouseButton>,
        sensitivity: f32,
    }
}