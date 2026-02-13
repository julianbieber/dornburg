use bevy::{
    feathers::controls::{ButtonProps, button},
    prelude::*,
    ui_widgets::{Activate, observe},
};

use crate::screens::Screen;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(0));
        app.insert_state(LevelScreens::None);
        app.add_systems(OnEnter(Screen::Gameplay), level);
        app.add_systems(OnEnter(LevelScreens::Restart), level);
        app.add_systems(OnEnter(LevelScreens::Intermission), spawn_intermission);
        app.add_systems(OnEnter(LevelScreens::Level), spawn_timer);
        app.add_systems(Update, update_timer.run_if(in_state(LevelScreens::Level)));
    }
}

fn level(mut next: ResMut<NextState<LevelScreens>>) {
    next.set(LevelScreens::Level);
}

#[derive(Resource, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct CurrentLevel(u32);

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum LevelScreens {
    None,
    Restart,
    Level,
    Intermission,
}

fn spawn_intermission(mut commands: Commands, _current_level: Res<CurrentLevel>) {
    commands.spawn((
        DespawnOnExit(LevelScreens::Intermission),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            row_gap: px(10),
            ..Default::default()
        },
        children![(
            button(ButtonProps::default(), (), Spawn(Text::new("next level"))),
            observe(next_level),
        ),],
    ));
}

fn next_level(
    _: On<Activate>,
    mut current_level: ResMut<CurrentLevel>,
    mut next: ResMut<NextState<LevelScreens>>,
) {
    current_level.0 += 1;
    next.set(LevelScreens::Level);
}

const POEM: &str = "Wer reitet so spät durch Nacht und Wind?
Es ist der Vater mit seinem Kind;
Er hat den Knaben wohl in dem Arm,
Er fasst ihn sicher, er hält ihn warm.

Mein Sohn, was birgst du so bang dein Gesicht? 
Siehst, Vater, du den Erlkönig nicht?
Den Erlenkönig mit Kron’ und Schweif? 
Mein Sohn, es ist ein Nebelstreif.

„Du liebes Kind, komm, geh mit mir!
Gar schöne Spiele spiel’ ich mit dir;
Manch’ bunte Blumen sind an dem Strand,
Meine Mutter hat manch gülden Gewand.“ 

Mein Vater, mein Vater, und hörest du nicht,
Was Erlenkönig mir leise verspricht?
Sei ruhig, bleibe ruhig, mein Kind;
In dürren Blättern säuselt der Wind.

„Willst, feiner Knabe, du mit mir gehn?
Meine Töchter sollen dich warten schön;
Meine Töchter führen den nächtlichen Reihn
Und wiegen und tanzen und singen dich ein.“

Mein Vater, mein Vater, und siehst du nicht dort
Erlkönigs Töchter am düstern Ort?
Mein Sohn, mein Sohn, ich seh’ es genau:
Es scheinen die alten Weiden so grau.

„Ich liebe dich, mich reizt deine schöne Gestalt;
Und bist du nicht willig, so brauch’ ich Gewalt.“
Mein Vater, mein Vater, jetzt fasst er mich an!
Erlkönig hat mir ein Leids getan!

Dem Vater grauset’s; er reitet geschwind,
Er hält in Armen das ächzende Kind,
Erreicht den Hof mit Mühe und Not;
In seinen Armen das Kind war tot.";

#[derive(Component)]
struct PoemState {
    timer: Timer,
}

fn spawn_timer(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        Node {
            width: percent(100.0),
            margin: UiRect::all(px(40)),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        children![(
            ScrollPosition(Vec2::ZERO),
            Node {
                max_width: px(200.0),
                max_height: px(10.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                overflow: Overflow::scroll_x(),
                ..Default::default()
            },
            PoemState {
                timer: Timer::from_seconds(120.0, TimerMode::Once)
            },
            children![(
                Text::new(POEM.replace("\n", " ")),
                TextFont::from_font_size(12.0),
                TextLayout::new_with_justify(Justify::Left).with_linebreak(LineBreak::NoWrap)
            )]
        ),],
    ));
}

fn update_timer(
    mut timer: Single<(&mut ScrollPosition, &mut PoemState, &ComputedNode)>,
    time: Res<Time>,
    mut next: ResMut<NextState<LevelScreens>>,
) {
    timer.1.timer.tick(time.delta());
    if timer.1.timer.just_finished() {
        next.set(LevelScreens::Restart);
    }

    // let total_time = timer.1.timer.duration().as_secs_f32();

    timer.0.x = timer.2.content_size.x * timer.1.timer.fraction();

    // timer.0.0 = content;
}
