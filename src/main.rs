use bevy::{feathers::FeathersPlugins, prelude::*};
use clap::Parser;

use crate::{gameplay::GameplayPlugin, screens::ScreenPlugin};

mod gameplay;
mod levels;
mod main_screen;
mod player;
mod player_controller;
mod screens;
mod terrain;

#[derive(Parser, Debug, Resource, Clone, Copy)]
struct Opts {
    #[arg(long)]
    debug_colliders: bool,
}

fn main() -> AppExit {
    let opts = Opts::parse();
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(RequiredAssets {
            levels: Vec::new(),
            font: None,
        })
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
    font: Option<Handle<Font>>,
}

fn load_levels(asset_server: Res<AssetServer>, mut required: ResMut<RequiredAssets>) {
    required
        .levels
        .push(asset_server.load("levels/level_1.png"));
    required
        .levels
        .push(asset_server.load("levels/level_2.png"));
    required
        .levels
        .push(asset_server.load("levels/level_3.png"));
    required
        .levels
        .push(asset_server.load("levels/level_4.png"));

    required.font = Some(asset_server.load("fonts/CinzelDecorative-Regular.ttf"));
}
