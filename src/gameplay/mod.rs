use avian2d::prelude::Gravity;
use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

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
            update_player_position.run_if(in_state(Screen::Gameplay)),
        );
    }
}
