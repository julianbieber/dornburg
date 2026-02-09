use bevy::prelude::*;

use crate::screens::Screen;

///
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(Levels::Level1));
        app.insert_state(LevelScreens::None);
        app.add_systems(OnEnter(Screen::Gameplay), level);
        app.add_systems(OnEnter(LevelScreens::Restart), level);
    }
}

fn level(mut next: ResMut<NextState<LevelScreens>>) {
    next.set(LevelScreens::Level);
}

#[derive(Resource, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct CurrentLevel(Levels);

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum LevelScreens {
    None,
    Restart,
    Level,
    Intermission,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Levels {
    Level1,
}
