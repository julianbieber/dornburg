use bevy::{
    feathers::{
        controls::{ButtonProps, button},
        theme::ThemeBackgroundColor,
        tokens,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

use crate::{
    RequiredAssets,
    screens::Screen,
    tooltip::{TooltipPlugin, *},
};
pub struct MainScreenPlugin;

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TooltipPlugin);
        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(Screen::Main), setup_ui);
        app.add_systems(OnEnter(Screen::Help), setup_help);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(main_root());
}

/// 3 Buttons:
/// * Play
/// * Help
/// * Quit
fn main_root() -> impl Bundle {
    (
        DespawnOnExit(Screen::Main),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            row_gap: px(10),
            ..Default::default()
        },
        ThemeBackgroundColor(tokens::WINDOW_BG),
        children![
            (
                button(ButtonProps::default(), (), Spawn(Text::new("Play!"))),
                observe(go_to_play),
            ),
            (
                button(ButtonProps::default(), (), Spawn(Text::new("Help"))),
                observe(go_to_help),
            ),
            (
                button(ButtonProps::default(), (), Spawn(Text::new("Quit"))),
                observe(quit),
            )
        ],
    )
}

fn go_to_help(_: On<Activate>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Help);
}

fn go_to_play(
    _: On<Activate>,
    mut next: ResMut<NextState<Screen>>,
    required: Res<RequiredAssets>,
    asset_server: Res<AssetServer>,
) {
    if required
        .levels
        .iter()
        .all(|l| asset_server.is_loaded_with_dependencies(l.id()))
        && required
            .font
            .clone()
            .is_some_and(|v| asset_server.is_loaded_with_dependencies(v.id()))
    {
        next.set(Screen::Gameplay);
    } else {
        warn!("Not all required levels loaded try again soon");
    }
}

fn setup_help(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Main),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            row_gap: px(10),
            ..Default::default()
        },
        ThemeBackgroundColor(tokens::WINDOW_BG),
        children![
            Text::new("In this little platformer, you collect a number of bones per level.\nIf you touch the 'Lava', go out of bounds, or the timer runs out, the level starts again.\nYou controll the player with:\nA/ArrowLeft: move left\nD/ArrowRight: move right\nSpace: jump\n\nThere are no limits to movement in the air. Go through the levels and enjoy this feaver dream.\n\nGo back to the main menu by pressing ESC from here.")
        ],
    ));
}

fn quit(_: On<Activate>, mut commands: Commands) {
    commands.write_message(AppExit::Success);
}
