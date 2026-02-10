use avian2d::dynamics::rigid_body::Friction;
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::levels::LevelScreens;
use crate::terrain::SpawnMarker;

#[derive(Component)]
pub struct PlayerMarker;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn: Single<&Transform, With<SpawnMarker>>,
) {
    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        spawn.into_inner().clone(),
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        Collider::rectangle(20.0, 20.0),
        RigidBody::Dynamic,
        Mass(1.0),
        Friction::new(0.8),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.2, 0.5))),
        PlayerMarker,
    ));
}
