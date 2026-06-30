use bevy::prelude::*;
use crate::render::LiveWorldRes;

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

pub fn follow_camera_system(
    world_res: Res<LiveWorldRes>,
    mut q: Query<&mut Transform, With<Camera3d>>,
) {
    let Some(snap) = world_res.0.players.get_index(world_res.0.follow_idx).map(|(_, s)| s) else { return };
    let target = Vec3::from_array(snap.position);
    if let Ok(mut tf) = q.get_single_mut() {
        let offset = Vec3::new(0.0, 15.0, 20.0);
        tf.translation = target + offset;
        tf.look_at(target, Vec3::Y);
    }
}