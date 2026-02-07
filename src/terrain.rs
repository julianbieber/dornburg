use avian2d::prelude::*;
use bevy::{color::color_difference::EuclideanDistance, prelude::*};

use crate::RequiredAssets;

/// A level is initialized from an image.
/// Compatible with the default color scale of rx.
/// 1A1C2C: Terrain
/// 566C86: Spawn
/// 333C57: End/Checkpoint
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
    images: Res<Assets<Image>>,
    required: Res<RequiredAssets>,
) {
    let level = images.get(required.levels.first().unwrap()).unwrap();
    if level.width() != 128 {
        panic!("levels must be 128p wide");
    }
    if level.height() != 128 {
        panic!("levels must be 128p high");
    }
    let terrain = Color::Srgba(Srgba::hex("#1A1C2C").unwrap());
    let mut voxels = VoxelizedView::empty();

    for y in 0..level.height() {
        for x in 0..level.width() {
            if let Ok(color) = level.get_color_at(x, y) {
                if color.distance(&terrain) <= 0.0001 {
                    voxels.set(x, y, true);
                }
            }
        }
    }

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0 * 2.0, 1280.0 * 2.0))),
        voxels.collider(),
        RigidBody::Static,
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
        // Transform::from_translation(Vec3::Y * ),
    ));
}

struct VoxelizedView {
    voxels: Vec<u128>,
}

impl VoxelizedView {
    fn empty() -> VoxelizedView {
        VoxelizedView {
            voxels: vec![0; 128],
        }
    }

    fn get(&self, x: u32, y: u32) -> bool {
        assert!(x < 128 && y < 128);
        self.voxels[x as usize] & 1u128 << y > 0
    }

    fn get_checked(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= 128 || y >= 128 {
            return false;
        }

        self.voxels[x as usize] & 1u128 << y > 0
    }

    fn set(&mut self, x: u32, y: u32, v: bool) {
        assert!(x < 128 && y < 128);
        let v = v as u128;
        let v = v << y;
        self.voxels[x as usize] |= v;
    }

    fn collider(&self) -> Collider {
        let mut coordinates = Vec::new();
        for x in 0..128 {
            for y in 0..128 {
                if self.get(x, y) {
                    coordinates.push(IVec2 {
                        x: x as i32 - 64,
                        y: y as i32 * -1 + 64,
                    });
                }
            }
        }
        Collider::voxels(Vec2::new(20.0, 20.0), &coordinates)
    }
}
