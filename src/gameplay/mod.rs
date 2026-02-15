use avian2d::prelude::Gravity;
use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::main_screen::camera_intro_zoom;
use crate::player::sync_camera_to_player;
use crate::terrain::out_of_bounds;
use crate::{
    Opts,
    levels::{LevelPlugin, LevelScreens},
    player::spawn_player,
    player_controller::update_player_position,
    screens::Screen,
    terrain::{RequiredFinishes, TerrainMaterial, spawn_level, update_terrain, update_time},
};

pub struct GameplayPlugin {
    pub opts: Opts,
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
            .insert_resource(Gravity(Vec2::NEG_Y * (9.81 * 50.0)));

        app.add_plugins(Material2dPlugin::<TerrainMaterial>::default());
        app.add_plugins(LevelPlugin);
        if self.opts.debug_colliders {
            app.add_plugins(PhysicsDebugPlugin);
        }
        app.insert_resource(RequiredFinishes(0));
        app.add_systems(
            OnEnter(LevelScreens::Level),
            (spawn_level, spawn_player).chain(),
        );
        app.add_systems(
            Update,
            (update_time, update_terrain)
                .chain()
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            (update_player_position, sync_camera_to_player)
                .chain()
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(Update, camera_intro_zoom.run_if(in_state(Screen::Gameplay)));
        app.add_systems(Update, out_of_bounds.run_if(in_state(Screen::Gameplay)));
        app.insert_resource(RunStartTime(0.0));
        app.add_systems(OnEnter(Screen::Gameplay), set_start);
    }
}

#[derive(Resource)]
pub struct RunStartTime(pub f32);

fn set_start(mut start: ResMut<RunStartTime>, time: Res<Time>) {
    start.0 = time.elapsed_secs();
}
