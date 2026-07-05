use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::render::LiveWorldRes;

#[derive(Resource)]
pub struct CameraOrbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    /// Where the camera is currently looking (interpolated during a switch).
    pub current_target: Vec3,
    /// Target we're blending toward (the focused ship's position).
    pub goal_target: Vec3,
    /// Previous ship's index; when it differs from LiveWorld.follow_idx we start a blend.
    pub last_follow_idx: usize,
    /// Blend progress 0..1 for the ongoing player switch.
    pub blend: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.35,
            distance: 30.0,
            current_target: Vec3::ZERO,
            goal_target: Vec3::ZERO,
            last_follow_idx: usize::MAX,
            blend: 1.0,
        }
    }
}

const SWITCH_DURATION: f32 = 0.5;
const MOUSE_SENSITIVITY: f32 = 0.005;

pub fn switch_target_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut world_res: ResMut<LiveWorldRes>,
) {
    let n = world_res.0.players.len();
    if n == 0 { return; }
    if keys.just_pressed(KeyCode::Tab) {
        world_res.0.follow_idx = (world_res.0.follow_idx + 1) % n;
    }
    for (i, code) in [
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4,
        KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
    ].iter().enumerate() {
        if keys.just_pressed(*code) && i < n {
            world_res.0.follow_idx = i;
        }
    }
}

pub fn orbit_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut orbit: ResMut<CameraOrbit>,
) {
    if !buttons.pressed(MouseButton::Right) {
        motion.clear();
        return;
    }
    let mut delta = Vec2::ZERO;
    for ev in motion.read() { delta += ev.delta; }
    if delta == Vec2::ZERO { return; }
    orbit.yaw -= delta.x * MOUSE_SENSITIVITY;
    orbit.pitch = (orbit.pitch - delta.y * MOUSE_SENSITIVITY)
        .clamp(-1.5, 1.5);
}

pub fn follow_camera_system(
    time: Res<Time>,
    world_res: Res<LiveWorldRes>,
    mut orbit: ResMut<CameraOrbit>,
    mut q: Query<&mut Transform, With<Camera3d>>,
) {
    let Some(snap) = world_res.0.players
        .get_index(world_res.0.follow_idx)
        .map(|(_, s)| s) else { return };
    let ship_pos = Vec3::from_array(snap.position);

    // Kick off a blend when the focused player changes.
    if orbit.last_follow_idx != world_res.0.follow_idx {
        if orbit.last_follow_idx == usize::MAX {
            orbit.current_target = ship_pos;
        }
        orbit.last_follow_idx = world_res.0.follow_idx;
        orbit.blend = 0.0;
    }
    orbit.goal_target = ship_pos;

    if orbit.blend < 1.0 {
        orbit.blend = (orbit.blend + time.delta_seconds() / SWITCH_DURATION).min(1.0);
        // smoothstep for a nicer ease
        let t = orbit.blend * orbit.blend * (3.0 - 2.0 * orbit.blend);
        let curr = orbit.current_target;
        orbit.current_target = curr.lerp(orbit.goal_target, t);
    } else {
        orbit.current_target = orbit.goal_target;
    }

    let (yaw, pitch, dist) = (orbit.yaw, orbit.pitch, orbit.distance);
    let offset = Vec3::new(
        yaw.sin() * pitch.cos() * dist,
        pitch.sin() * dist,
        yaw.cos() * pitch.cos() * dist,
    );
    if let Ok(mut tf) = q.get_single_mut() {
        tf.translation = orbit.current_target + offset;
        tf.look_at(orbit.current_target, Vec3::Y);
    }
}
