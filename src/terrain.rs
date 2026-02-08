use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    color::color_difference::EuclideanDistance,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d},
    sprite_render::Material2d,
};

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
    mut materials: ResMut<Assets<TerrainMaterial>>,
    mut images: ResMut<Assets<Image>>,
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
        MeshMaterial2d(materials.add(TerrainMaterial {
            terrain: images.add(voxels.as_tex()),
        })),
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
                        y: y as i32 * -1 + 63,
                    });
                }
            }
        }
        Collider::voxels(Vec2::new(20.0, 20.0), &coordinates)
    }

    fn as_tex(&self) -> Image {
        let mut height_bytes = Vec::new();
        for x in 0..128 {
            for y in 0..128 {
                height_bytes.extend_from_slice(&(self.get(x, y) as i32 as f32).to_le_bytes());
            }
        }

        let mut i = Image::new(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            height_bytes,
            bevy::render::render_resource::TextureFormat::R32Float,
            RenderAssetUsages::all(),
        );
        i.sampler = ImageSampler::nearest();

        i
    }
}

// Terrain Shader

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TerrainMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub terrain: Handle<Image>,
}

impl Material2d for TerrainMaterial {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        bevy::shader::ShaderRef::Default
    }

    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/terrain.wgsl".into()
    }

    fn depth_bias(&self) -> f32 {
        0.0
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        bevy::sprite_render::AlphaMode2d::Blend
    }
}
