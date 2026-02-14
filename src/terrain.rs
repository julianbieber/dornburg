use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    color::color_difference::EuclideanDistance,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d},
    sprite_render::Material2d,
};

use crate::{RequiredAssets, levels::LevelScreens, player::PlayerMarker};

/// A level is initialized from an image.
/// Compatible with the default color scale of rx.
/// 1A1C2C: Terrain
/// 566C86: Spawn
/// 73EFF7: End/Checkpoint
/// B13E53: Killzone
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
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    required: Res<RequiredAssets>,
    mut required_finishes: ResMut<RequiredFinishes>,
) {
    let level = images.get(required.levels.first().unwrap()).unwrap();
    if level.width() != 128 {
        panic!("levels must be 128p wide");
    }
    if level.height() != 128 {
        panic!("levels must be 128p high");
    }
    let terrain = Color::Srgba(Srgba::hex("#1A1C2C").unwrap());
    let kill = Color::Srgba(Srgba::hex("#B13E53").unwrap());
    let finish = Color::Srgba(Srgba::hex("#73EFF7").unwrap());
    let spawn_color = Color::Srgba(Srgba::hex("#566C86").unwrap());

    let mut voxels = VoxelizedView::empty();
    let mut killzones = Killzones::empty();
    let mut finishes = Vec::new();
    let mut finish_coords = Vec::new();
    let mut spawn = Vec2::ZERO;

    for y in 0..level.height() {
        for x in 0..level.width() {
            if let Ok(color) = level.get_color_at(x, y) {
                voxels.set(x, y, color.distance(&terrain) <= 0.0001);
                killzones.set(x, y, color.distance(&kill) <= 0.0001);
                if color.distance(&finish) < 0.0001 {
                    finishes.push(voxel_to_world(x, y));
                    finish_coords.push((x, y));
                }
                if color.distance(&spawn_color) <= 0.0001 {
                    spawn = voxel_to_world(x, y);
                }
            }
        }
    }
    voxels.finish_coords = finish_coords;

    let time = TimeDiluationMap::zero();

    let mut spawn_command = commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        Mesh2d(meshes.add(Rectangle::new(1280.0 * 2.0, 1280.0 * 2.0))),
        RigidBody::Static,
        MeshMaterial2d(materials.add(TerrainMaterial {
            terrain: images.add(voxels.as_tex()),
            time: images.add(time.as_tex()),
            kill: images.add(killzones.as_tex()),
            player: Vec4::ZERO,
        })),
        voxels.clone(),
        time,
        UpdateTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));

    if let Some(collider) = voxels.collider() {
        spawn_command.insert(collider);
    }

    let mut kill_spawn = commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        killzones.clone(),
        CollisionEventsEnabled,
    ));
    kill_spawn.observe(player_dies);
    if let Some(collider) = killzones.collider() {
        kill_spawn.insert(collider);
    }

    required_finishes.0 = finishes.len() as u32;
    for finish in finishes {
        commands
            .spawn((
                DespawnOnExit(LevelScreens::Level),
                Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
                MeshMaterial2d(colors.add(Color::srgb(0.6, 0.6, 0.6))),
                CollisionEventsEnabled,
                Collider::rectangle(20.0, 20.0),
                Transform::from_translation(Vec3::new(finish.x + 10.0, finish.y + 10.0, 0.0)),
                FinishMarker,
            ))
            .observe(collect_finish);
    }

    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        Transform::from_translation(Vec3::new(spawn.x + 10.0, spawn.y + 10.0, 0.0)),
        SpawnMarker,
    ));
}

#[derive(Component)]
pub struct SpawnMarker;

/// A circle of radius 8 blocks (160p) should not change
/// the area from 8-10 blocks (160p - 200p) shows crater than 1s/s change
/// further blocks show 1s/s change
/// https://graphtoy.com/?f1(x,t)=clamp((x%5E2/160-160)/45,0,1)&v1=true&f2(x,t)=4/(1+f1(x,t))-1&v2=true&f3(x,t)=min(f1(x,t)*3,f2(x,t))&v3=true&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=165.74778969058985,-0.9241138897409666,12.000000000000151
pub fn update_time(
    player: Single<&Transform, With<PlayerMarker>>,
    mut times: Query<&mut TimeDiluationMap>,
    clock: Res<Time>,
) {
    let p = player.translation.xy();
    let d = clock.delta_secs();
    for mut time in &mut times {
        for x in 0..128 {
            for y in 0..128 {
                let voxel_position = voxel_to_world(x, y);
                let z = p.distance_squared(voxel_position);
                let f1 = (z / 160.0 - 160.0).clamp(0.0, 1.0);
                let f2 = 4.0 / (1.0 + f1) - 1.0;
                let f3 = (f1 * 3.0).min(f2);
                time.set(x, y, d * f3);
            }
        }
    }
}

#[derive(Component)]
pub struct FinishMarker;

#[derive(Resource)]
pub struct RequiredFinishes(pub u32);

fn collect_finish(
    event: On<CollisionStart>,
    player: Single<Entity, With<PlayerMarker>>,
    mut next: ResMut<NextState<LevelScreens>>,
    mut commands: Commands,
    mut required_finishes: ResMut<RequiredFinishes>,
) {
    if event.collider2.entity() == player.into_inner() {
        commands.entity(event.collider1.entity()).despawn();
        if required_finishes.0 > 0 {
            required_finishes.0 -= 1;
        }
        if required_finishes.0 == 0 {
            next.set(LevelScreens::Intermission);
        }
    }
}

fn player_dies(
    event: On<CollisionStart>,
    player: Single<Entity, With<PlayerMarker>>,
    mut next: ResMut<NextState<LevelScreens>>,
) {
    let e = event.body2.unwrap();
    if e == player.into_inner() {
        next.set(LevelScreens::Restart);
    }
}

pub fn out_of_bounds(
    player: Single<&Transform, With<PlayerMarker>>,
    mut next: ResMut<NextState<LevelScreens>>,
) {
    if player.translation.x < -20.0 * 65.0
        || player.translation.x > 20.0 * 65.0
        || player.translation.y < -20.0 * 65.0
        || player.translation.y > 20.0 * 65.0
    {
        next.set(LevelScreens::Restart);
    }
}

fn voxel_to_world(x: u32, y: u32) -> Vec2 {
    Vec2::new(x as f32 - 64.0, -(y as f32) + 63.0) * 20.0
}

pub fn update_terrain(
    mut commands: Commands,
    mut terrain: Query<(
        Entity,
        &mut VoxelizedView,
        &mut MeshMaterial2d<TerrainMaterial>,
        &TimeDiluationMap,
        &Transform,
        &mut UpdateTimer,
    )>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    mut images: ResMut<Assets<Image>>,
    killzones: Single<&Killzones>,
    player: Single<&Transform, With<PlayerMarker>>,
    global_time: Res<Time>,
) {
    let p = player.translation.xy();
    for (entity, mut voxels, mut mat, time, transform, mut timer) in &mut terrain {
        timer.0.tick(global_time.delta());
        if timer.0.just_finished() {
            let total = voxels.total();
            let grow = total < 1638;
            let shrink = total > 4915;
            let mut new_voxels = voxels.clone();
            let mut max_noise: f32 = 0.0;
            for x in 0..128 {
                let x_f = x as f32 / 128.0;
                for y in 0..128 {
                    let voxel_position = voxel_to_world(x, y) + transform.translation.xy();
                    if p.distance_squared(voxel_position) < 300.0 * 300.0 {
                        continue;
                    }
                    let y_f = y as f32 / 128.0;

                    let s = voxels.get_surrounding(x, y, 1);

                    let n = fbm(
                        Vec3::new(x_f, y_f, global_time.elapsed_secs()),
                        5,
                        20.0,
                        1.2,
                        0.6,
                    );
                    max_noise = max_noise.max(n);
                    let c = voxels.get(x, y);
                    if grow {
                        if c || n > 4.3 {
                            new_voxels.set(x, y, s >= 5);
                        } else {
                            new_voxels.set(x, y, (3..4).contains(&s) || s == 1);
                        }
                    } else if shrink {
                        if c && n > 4.3 {
                            new_voxels.set(x, y, s >= 7 || s == 1);
                        } else {
                            new_voxels.set(x, y, (4..6).contains(&s));
                        }
                    } else if c || n > 4.3 {
                        new_voxels.set(x, y, s >= 7);
                    } else {
                        new_voxels.set(x, y, (4..6).contains(&s));
                    }

                    // new_voxels.set(x, y, n > 4.4);
                }
            }
            dbg!(max_noise);
            *voxels = new_voxels;
            if let Some(collider) = voxels.collider() {
                commands
                    .get_entity(entity)
                    .unwrap()
                    .remove::<Collider>()
                    .insert(collider);
            } else {
                commands.get_entity(entity).unwrap().remove::<Collider>();
            }
        }
        mat.0 = materials.add(TerrainMaterial {
            terrain: images.add(voxels.as_tex()),
            time: images.add(time.as_tex()),
            kill: images.add(killzones.as_tex()),
            player: Vec4::new(p.x, p.y, 0.0, 0.0), // probably not rquired, we could get the handle of the exisitng image
        })
    }
}

fn dotnoise(mut x: Vec3) -> f32 {
    let mut v = 0.0;
    for i in 0..4 {
        x = x
            .rotate_x(1.2 * i as f32)
            .rotate_y(0.3 * i as f32)
            .rotate_z(0.4 * i as f32);
        v += Vec3::new(x.x.cos(), x.y.cos(), x.z.cos()).dot(Vec3::new(
            x.y.cos(),
            x.z.cos(),
            x.x.cos(),
        ));
    }
    v.abs() / 4.0
}

fn mirror(p: Vec3, n: Vec3, d: f32) -> Vec3 {
    let dist = p.dot(n) - d;
    p - 2.0 * dist * n
}

pub fn fbm(
    mut point: Vec3,
    octaves: usize,
    frequency: f32,
    lacunarity: f32,
    persistence: f32,
) -> f32 {
    let mut result = 0.0;
    let mut amplitude = 1.0;
    let mut freq = frequency;

    for i in 0..octaves {
        result += 1.0 - dotnoise(point * freq) * amplitude;
        // result *= result * result;
        point = point.rotate_x(0.1).rotate_y(0.2).rotate_z(0.3);
        point += Vec3::ONE;
        point = mirror(
            point,
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            i as f32 * 1.6 * result,
        );
        freq *= lacunarity;
        amplitude *= persistence;
    }

    result
}

#[derive(Component)]
pub struct UpdateTimer(pub Timer);

#[derive(Component, Clone)]
pub struct VoxelizedView {
    voxels: Vec<u128>,
    finish_coords: Vec<(u32, u32)>,
}

impl VoxelizedView {
    fn empty() -> VoxelizedView {
        VoxelizedView {
            voxels: vec![0; 128],
            finish_coords: Vec::new(),
        }
    }

    fn get(&self, x: u32, y: u32) -> bool {
        assert!(x < 128 && y < 128);
        self.voxels[x as usize] & (1u128 << y) > 0
    }

    fn get_checked(&self, mut x: i32, mut y: i32) -> bool {
        x = x.clamp(0, 127);
        y = y.clamp(0, 127);

        self.voxels[x as usize] & (1u128 << y) > 0
    }

    /// returns how many of the 9 pixels are set;
    fn get_surrounding(&self, x: u32, y: u32, size: i32) -> u8 {
        let x = x as i32;
        let y = y as i32;
        let mut s = 0;

        for x_o in -size..=size {
            for y_o in -size..=size {
                s += self.get_checked(x + x_o, y + y_o) as u8;
            }
        }
        s
    }

    fn set(&mut self, x: u32, y: u32, v: bool) {
        assert!(x < 128 && y < 128);
        if self.finish_coords.contains(&(x, y)) {
            return;
        }
        let v = v as u128;
        self.voxels[x as usize] = (self.voxels[x as usize] & !(1 << y)) | (v << y);
    }

    fn total(&self) -> u32 {
        let mut c = 0;
        for x in 0..128 {
            for y in 0..128 {
                c += self.get(x, y) as u32;
            }
        }
        c
    }

    fn collider(&self) -> Option<Collider> {
        let mut coordinates = Vec::new();
        for x in 0..128 {
            for y in 0..128 {
                if self.get(x, y) {
                    coordinates.push(IVec2 {
                        x: x as i32 - 64,
                        y: -(y as i32) + 63,
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

#[derive(Component)]
pub struct TimeDiluationMap {
    time: Vec<f32>,
}

impl TimeDiluationMap {
    fn zero() -> TimeDiluationMap {
        TimeDiluationMap {
            time: vec![0.0; 128 * 128],
        }
    }

    fn set(&mut self, x: u32, y: u32, d: f32) {
        assert!(x < 128 && y < 128);
        let i = x * 128 + y;
        self.time[i as usize] += d;
    }

    fn as_tex(&self) -> Image {
        let height_bytes = self.time.iter().flat_map(|f| f.to_le_bytes()).collect();
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
        i.sampler = ImageSampler::linear();

        i
    }
}

#[derive(Component, Clone)]
pub struct Killzones {
    voxels: Vec<u128>,
}

impl Killzones {
    fn empty() -> Killzones {
        Killzones {
            voxels: vec![0; 128],
        }
    }

    fn get(&self, x: u32, y: u32) -> bool {
        assert!(x < 128 && y < 128);
        self.voxels[x as usize] & 1u128 << y > 0
    }

    fn set(&mut self, x: u32, y: u32, v: bool) {
        assert!(x < 128 && y < 128);
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
                        y: -(y as i32) + 63,
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
    #[texture(2)]
    #[sampler(3)]
    pub time: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub kill: Handle<Image>,
    #[uniform(6)]
    pub player: Vec4,
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
