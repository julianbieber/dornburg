use avian2d::prelude::*;
use bevy::prelude::*;

/// A level is initialized from a greyscale image.
/// 0: Terrain
/// 255: No Terrain
/// 60: Start
/// 180: End/Checkpoint
///
/// The image is transformed into a mesh, with 1 vertex per pixel
/// adjacent vertices are conencted into triangles
/// Vertices without adjacent vertices are expanded to a quad
///
/// The complete level terrain is a single mesh.
///
/// Modifying the mesh is done by moving the vertices along their normal vector.
///
/// Coloring is done via a fragment shader based on the screen coordinates + camera offset, so that vertex overlaps dont create z fighting.
///
/// We can recreate a collider from the mesh for each frame, if too expensive recreate in background once the previous has been calculated. So that the collider stays somewhat close to the visuals.
pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(2000.0, 20.0))),
        Collider::rectangle(2000.0, 20.0),
        RigidBody::Static,
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
        Transform::from_translation(Vec3::Y * -500.0),
    ));
}
