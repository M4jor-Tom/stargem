use bevy::prelude::*;
use crate::world::{LiveWorld, ShotKind, ShotSpawn};

#[derive(Resource)]
pub struct LiveWorldRes(pub LiveWorld);

#[derive(Resource)]
pub struct EventRx(pub crossbeam_channel::Receiver<crate::grpc::SpectatorEvent>);

#[derive(Component)]
pub struct ShipEntity { pub key: String }

#[derive(Resource)]
pub struct ShipMaterials {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct ActiveShots {
    pub items: Vec<ActiveShot>,
}

pub struct ActiveShot {
    pub src: String,
    pub tgt: String,
    pub kind: ShotKind,
    pub age: f32,
    pub life: f32,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ponytail: transparent primitive stands in for a real gltf ship
    let cube = meshes.add(Cuboid::new(4.0, 2.0, 4.0));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.7, 0.7, 0.9, 0.35),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.insert_resource(ShipMaterials { mesh: cube, material: mat });
    commands.init_resource::<ActiveShots>();

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { illuminance: 8000.0, ..default() },
        transform: Transform::from_xyz(50.0, 80.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 80.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn sync_ship_entities(
    mut commands: Commands,
    world_res: Res<LiveWorldRes>,
    mats: Res<ShipMaterials>,
    mut q: Query<(Entity, &ShipEntity, &mut Transform)>,
) {
    let mut seen = std::collections::HashSet::new();
    for (entity, marker, mut tf) in &mut q {
        if let Some(snap) = world_res.0.players.get(&marker.key) {
            tf.translation = Vec3::from_array(snap.position);
            tf.rotation = Quat::from_array(snap.rotation);
            seen.insert(marker.key.clone());
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
    for (key, snap) in &world_res.0.players {
        if seen.contains(key) { continue; }
        commands.spawn((
            PbrBundle {
                mesh: mats.mesh.clone(),
                material: mats.material.clone(),
                transform: Transform::from_translation(Vec3::from_array(snap.position))
                    .with_rotation(Quat::from_array(snap.rotation)),
                ..default()
            },
            ShipEntity { key: key.clone() },
        ));
    }
}

pub fn drain_events(
    rx: Res<EventRx>,
    mut world_res: ResMut<LiveWorldRes>,
    mut shots: ResMut<ActiveShots>,
) {
    while let Ok(ev) = rx.0.try_recv() {
        match ev {
            crate::grpc::SpectatorEvent::Snapshot(s) => {
                eprintln!("spectator: tick={} players={} dmg={}", s.tick_number, s.players.len(), s.damage_events.len());
                crate::world::apply_snapshot(&mut world_res.0, &s);
            }
            crate::grpc::SpectatorEvent::Matches(m) => {
                eprintln!("spectator: found {} match(es)", m.len());
            }
            crate::grpc::SpectatorEvent::Error(e) => {
                eprintln!("spectator: error: {e}");
            }
        }
    }
    for ShotSpawn { src, tgt, kind } in world_res.0.pending_shots.drain(..) {
        let life = match kind { ShotKind::Laser => 0.15, _ => 0.5 };
        shots.items.push(ActiveShot { src, tgt, kind, age: 0.0, life });
    }
}

pub fn draw_shots_system(
    time: Res<Time>,
    world_res: Res<LiveWorldRes>,
    mut shots: ResMut<ActiveShots>,
    mut gizmos: Gizmos,
) {
    let dt = time.delta_seconds();
    for shot in &mut shots.items {
        shot.age += dt;
    }
    shots.items.retain(|s| s.age < s.life);

    for shot in &shots.items {
        let (Some(src), Some(tgt)) = (
            world_res.0.players.get(&shot.src),
            world_res.0.players.get(&shot.tgt),
        ) else { continue };
        let a = Vec3::from_array(src.position);
        let b = Vec3::from_array(tgt.position);
        let (color, is_ray) = match shot.kind {
            ShotKind::Kinetic => (Color::srgb(1.0, 0.85, 0.3), false),
            ShotKind::Electromagnetic => (Color::srgb(0.4, 0.8, 1.0), false),
            ShotKind::Laser => (Color::srgb(1.0, 0.2, 0.2), true),
        };
        if is_ray {
            gizmos.line(a, b, color);
        } else {
            let t = (shot.age / shot.life).clamp(0.0, 1.0);
            let pos = a.lerp(b, t);
            gizmos.sphere(pos, Quat::IDENTITY, 0.6, color);
        }
    }
}
