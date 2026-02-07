use bevy::{
    app::{App, Plugin},
    state::{app::AppExtStates, state::States},
};

use crate::{gameplay::GameplayPlugin, main_screen::MainScreenPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.add_plugins((MainScreenPlugin, GameplayPlugin));
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Help,
    Gameplay,
}
