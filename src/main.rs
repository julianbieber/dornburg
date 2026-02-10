use bevy::{feathers::FeathersPlugins, prelude::*};
use clap::Parser;

use crate::{gameplay::GameplayPlugin, screens::ScreenPlugin};

mod gameplay;
mod levels;
mod main_screen;
mod player;
mod screens;
mod terrain;
mod tooltip;
mod player_controller;

#[derive(Parser, Debug, Resource, Clone, Copy)]
struct Opts {
    #[arg(long)]
    debug_colliders: bool,
}

fn main() -> AppExit {
    let opts = Opts::parse();
    App::new()
        .insert_resource(RequiredAssets { levels: Vec::new() })
        .add_systems(Startup, load_levels)
        .add_plugins((
            DefaultPlugins,
            FeathersPlugins,
            ScreenPlugin,
            GameplayPlugin { opts },
        ))
        .run()
}

#[derive(Resource)]
pub struct RequiredAssets {
    pub levels: Vec<Handle<Image>>,
}

fn load_levels(asset_server: Res<AssetServer>, mut required: ResMut<RequiredAssets>) {
    let example = asset_server.load("levels/level_example.png");
    required.levels.push(example);
}
