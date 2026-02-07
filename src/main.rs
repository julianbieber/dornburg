use bevy::{feathers::FeathersPlugins, prelude::*};
use clap::Parser;

use crate::{gameplay::GameplayPlugin, screens::ScreenPlugin};

mod gameplay;
mod main_screen;
mod player;
mod screens;
mod terrain;
mod tooltip;

#[derive(Parser, Debug, Resource, Clone, Copy)]
struct Opts {
    #[arg(long)]
    debug_colliders: bool,
}

fn main() -> AppExit {
    let opts = Opts::parse();
    App::new()
        .add_plugins((
            DefaultPlugins,
            FeathersPlugins,
            ScreenPlugin,
            GameplayPlugin { opts },
        ))
        .run()
}
