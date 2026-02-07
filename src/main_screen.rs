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

fn go_to_play(_: On<Activate>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Gameplay);
}

fn setup_help(
    mut commands: Commands,
    known_toolips: Res<TooltipMap>,
    mut stack: ResMut<TooltipStack>,
) {
    commands.spawn((
        DespawnOnExit(Screen::Help),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            row_gap: px(10),
            ..Default::default()
        },
        ThemeBackgroundColor(tokens::WINDOW_BG),
        children![Text::new("Some text to explain how to play the game")],
    ));
    spawn_tooltip(
        commands,
        &known_toolips.tooltips,
        &mut stack.entities,
        "Some text containing clickable words, and non clickable words\nand a line break",
        (px(0), px(0)),
        false,
    );
}

fn quit(_: On<Activate>, mut commands: Commands) {
    commands.write_message(AppExit::Success);
}
