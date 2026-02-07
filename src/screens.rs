use bevy::{
    app::{App, Plugin},
    state::{app::AppExtStates, state::States},
};

use crate::main_screen::MainScreenPlugin;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.add_plugins(MainScreenPlugin);
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Help,
    Gameplay,
}
