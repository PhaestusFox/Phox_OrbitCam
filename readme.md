# Phox Orbit Camera
this is an orbit camera made for the [Phox Plugin Jam](https://itch.io/jam/phoxs-bevy-plugin-jam)
this is a fairly simple orbit camera that is intended to give people a plugin to use
during the [Phox Game Jam](https://itch.io/jam/phoxs-bevy-game-jam), the functionality is fairly simple but may be expanded in the future

you can watch the [livestrems](https://www.youtube.com/playlist?list=PL6uRoaCCw7GOrAUOrfBXZESKbdRc-PhS9) of me making the plugin the whole thing was live-streamed, yes even this bit right here, yes me Writing documentation is as boring as you think it is, you should definitely watch me Writing this bit specifically ;P

# How to use
add this repo to your cargo.toml
```toml
phox_orbitcam = {git = "https://github.com/PhaestusFox/Phox_OrbitCam.git"}
```
then add the plgin to your App
```rust
App::new().add_plugins(phox_orbitcam::OrbitCamPlugin)
```
and finally add the OrbitCam Component to your camera entity
```rust
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
```
you can configure each camera individually with this component
GLHF in the jam this weekend
