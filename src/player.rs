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
    asset_server: Res<AssetServer>,
) {
    let mut transform = *spawn.into_inner();
    transform.translation.z = 1.0;
    let player_texture: Handle<Image> = asset_server.load("sprites/goethe_paint_head.png");
    let material = materials.add(ColorMaterial {
        texture: Some(player_texture),
        color: Color::WHITE,
        alpha_mode: Default::default(),
        uv_transform: Default::default(),
    });

    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        transform,
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(material),
        Collider::rectangle(20.0, 20.0),
        RigidBody::Dynamic,
        Mass(1.0),
        Friction::new(0.3),
        PlayerMarker,
    ));
}

pub fn sync_camera_to_player(
    player: Single<&Transform, (With<PlayerMarker>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<PlayerMarker>)>,
) {
    camera.translation = player.translation;
}
