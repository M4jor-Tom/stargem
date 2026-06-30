use bevy::prelude::*;
use crate::world::{LiveWorld, ShipSnap};

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

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(4.0, 2.0, 4.0));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.9),
        ..default()
    });
    commands.insert_resource(ShipMaterials { mesh: cube, material: mat });

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
) {
    while let Ok(ev) = rx.0.try_recv() {
        match ev {
            crate::grpc::SpectatorEvent::Snapshot(s) => crate::world::apply_snapshot(&mut world_res.0, &s),
            crate::grpc::SpectatorEvent::Matches(_) => {}
            crate::grpc::SpectatorEvent::Error(e) => tracing::warn!("spectator stream: {e}"),
        }
    }
}