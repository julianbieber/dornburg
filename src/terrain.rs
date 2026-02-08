use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    color::color_difference::EuclideanDistance,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d},
    sprite_render::Material2d,
};

use crate::{RequiredAssets, player::PlayerMarker};

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
/// The complete level terrain is a single rectangle.
///
/// Modifying the terrain is done by growing or shrinking the islands according to cellular automaton rules and a nosie function.
///
/// Coloring is done via a fragment shader based on the screen coordinates + camera offset.
///
/// We create a collision shape based on the voxels each frame.
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

    if let Some(collider) = voxels.collider() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1280.0 * 2.0, 1280.0 * 2.0))),
            collider,
            RigidBody::Static,
            MeshMaterial2d(materials.add(TerrainMaterial {
                terrain: images.add(voxels.as_tex()),
            })),
            voxels, // Transform::from_translation(Vec3::Y * ),
        ));
    } else {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1280.0 * 2.0, 1280.0 * 2.0))),
            RigidBody::Static,
            MeshMaterial2d(materials.add(TerrainMaterial {
                terrain: images.add(voxels.as_tex()),
            })),
            voxels, // Transform::from_translation(Vec3::Y * ),
        ));
    }
}

pub fn update_terrain(
    mut commands: Commands,
    time: Res<Time>,
    mut terrain: Query<(
        Entity,
        &mut VoxelizedView,
        &mut MeshMaterial2d<TerrainMaterial>,
        &Transform,
    )>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    mut images: ResMut<Assets<Image>>,
    player: Single<&Transform, With<PlayerMarker>>,
) {
    for (entity, mut voxels, mut mat, transform) in &mut terrain {
        for x in 0..128 {
            let x_f = x as f32 / 128.0;
            for y in 0..128 {
                let voxel_position = Vec2::new(
                    x as f32 - 64.0,        // + transform.translation.x,
                    y as f32 * -1.0 + 63.0, // + transform.translation.y,
                ) * 20.0;

                if player.translation.xy().distance_squared(voxel_position) < 300.0 * 300.0 {
                    continue;
                }

                let s = voxels.get_surrounding(x, y) as f32 / 9.0 + 1.0;
                if s < 1.2 {
                    continue;
                }
                let y_f = y as f32 / 128.0;
                let grow_or_shrink = dotnoise(Vec3::new(
                    x_f * 40.0,
                    y_f * 40.0,
                    time.elapsed_secs() * 1.01,
                )) * s;
                if grow_or_shrink < -4.0 {
                    voxels.set(x, y, false);
                } else if grow_or_shrink > 4. {
                    voxels.set(x, y, true)
                }
            }
        }
        if let Some(collider) = voxels.collider() {
            commands
                .get_entity(entity)
                .unwrap()
                .remove::<Collider>()
                .insert(collider);
        } else {
            commands.get_entity(entity).unwrap().remove::<Collider>();
        }
        mat.0 = materials.add(TerrainMaterial {
            terrain: images.add(voxels.as_tex()),
        })
    }
}

fn dotnoise(mut x: Vec3) -> f32 {
    let mut v = 0.0;
    for _ in 0..4 {
        x = x.rotate_x(0.2).rotate_y(0.3).rotate_z(0.4);
        v += Vec3::new(x.x.cos(), x.y.cos(), x.z.cos()).dot(Vec3::new(
            x.y.cos(),
            x.z.cos(),
            x.x.cos(),
        ));
    }
    v
}

#[derive(Component)]
pub struct VoxelizedView {
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

    /// returns how many of the 9 pixels are set;
    fn get_surrounding(&self, x: u32, y: u32) -> u8 {
        let x = x as i32;
        let y = y as i32;
        let mut s = 0;

        for x_o in [-1, 0, 1] {
            for y_o in [-1, 0, 1] {
                s += self.get_checked(x + x_o, y + y_o) as u8;
            }
        }
        s
    }

    fn set(&mut self, x: u32, y: u32, v: bool) {
        assert!(x < 128 && y < 128);
        let v = v as u128;
        let v = v as u128;
        self.voxels[x as usize] = (self.voxels[x as usize] & !(1 << y)) | (v << y);
    }

    fn collider(&self) -> Option<Collider> {
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
        if coordinates.is_empty() {
            None
        } else {
            Some(Collider::voxels(Vec2::new(20.0, 20.0), &coordinates))
        }
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
