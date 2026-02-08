use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::{
    Opts,
    player::spawn_player,
    screens::Screen,
    terrain::{TerrainMaterial, spawn_level},
};

pub struct GameplayPlugin {
    pub opts: Opts,
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0));
        app.add_plugins(Material2dPlugin::<TerrainMaterial>::default());
        if self.opts.debug_colliders {
            app.add_plugins(PhysicsDebugPlugin);
        }
        app.add_systems(
            OnEnter(Screen::Gameplay),
            (spawn_level, spawn_player).chain(),
        );
    }
}
