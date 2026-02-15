use bevy::{
    feathers::controls::{ButtonProps, button},
    prelude::*,
    ui_widgets::{Activate, observe},
};

use crate::{RequiredAssets, gameplay::RunStartTime, screens::Screen, terrain::RequiredFinishes};
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(0));
        app.insert_state(LevelScreens::None);
        app.add_systems(OnEnter(Screen::Gameplay), level);
        app.add_systems(OnEnter(LevelScreens::Restart), level);
        app.add_systems(OnEnter(LevelScreens::Intermission), spawn_intermission);
        app.add_systems(OnEnter(LevelScreens::Level), spawn_timer);
        app.add_systems(OnEnter(LevelScreens::Level), spawn_missing_finishes);
        app.add_systems(OnEnter(LevelScreens::GameEnd), display_end);
        app.add_systems(
            Update,
            (update_timer, update_finish_text).run_if(in_state(LevelScreens::Level)),
        );
    }
}

fn level(mut next: ResMut<NextState<LevelScreens>>) {
    next.set(LevelScreens::Level);
}

#[derive(Resource, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct CurrentLevel(pub u32);

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum LevelScreens {
    None,
    Restart,
    Level,
    Intermission,
    GameEnd,
}

fn spawn_intermission(
    mut commands: Commands,
    _current_level: Res<CurrentLevel>,
    assets: Res<RequiredAssets>,
) {
    let intermission_text_0 = "Are you the skull I seek? The skull of my deceased friend?\nWhy dont you answer me?\\nWhy do you just look at me with those empty eyes, none of your former wit left.\n\nI must search further.";

    let intermission_text_1 = "A new skull found, I must ask again! Are you the skull I seek? The skull of my deceased friend? I have come to rescue you out of this decaying ruin. What is this place you have been buried in? Why does nothing stay static? What is is ever shifting crypt?\n\nee ahm noht teh skooll yohoo sehehk! lehahveh meh toh rehst, teh kahngeh een tees pahlceh ees cohmfohrteeng fohr meh een my ehtehrnahl rehst.";

    let intermission_text_2 = "Why are there so many skulls? I only paid the grave digger to desecrate a singular grave. Are you haunting me for the sin of needing to speak to you again?\n\nSilence from the three skulls, I feel their discerning stares following me.";

    let intermission_text_3 = "Again three, why three? Please talk to me, is either of you my firend? I have been told he was buried in this crypt. The grave has his name, why are there so many? Why have you forsaken me? How can I go on if I dont find you.\n\nheh whoh yohoo sehehk wahs nehvehr hehreh. ee ahm sohrry boot weh cahn noht lehssehn yohoor boordehn.";

    let texts = [
        intermission_text_0,
        intermission_text_1,
        intermission_text_2,
        intermission_text_3,
    ];

    commands.spawn((
        DespawnOnExit(LevelScreens::Intermission),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(90),
            height: percent(90),
            margin: UiRect::all(Val::Auto),
            row_gap: px(10),
            ..Default::default()
        },
        children![
            // --- TOP TEXT BOX ---
            (
                Node {
                    width: percent(100),
                    height: percent(80),
                    padding: UiRect::all(px(16.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new(texts[_current_level.0 as usize]),
                    TextFont {
                        font: assets.font.clone().unwrap(),
                        ..Default::default()
                    },
                )],
            ),
            (
                button(
                    ButtonProps::default(),
                    (),
                    Spawn((
                        Text::new("next level"),
                        TextFont {
                            font: assets.font.clone().unwrap(),
                            ..Default::default()
                        },
                    ))
                ),
                observe(next_level),
            ),
        ],
    ));
}

fn next_level(
    _: On<Activate>,
    mut current_level: ResMut<CurrentLevel>,
    mut next: ResMut<NextState<LevelScreens>>,
) {
    current_level.0 += 1;
    if current_level.0 < 4 {
        next.set(LevelScreens::Level);
    } else {
        next.set(LevelScreens::GameEnd);
    }
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

fn spawn_timer(mut commands: Commands, assets: Res<RequiredAssets>) {
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
                timer: Timer::from_seconds(90.0, TimerMode::Once)
            },
            children![(
                Text::new(POEM.replace("\n", " ")),
                TextFont {
                    font: assets.font.clone().unwrap(),
                    font_size: 15.0,
                    ..Default::default()
                },
                TextLayout::new_with_justify(Justify::Left).with_linebreak(LineBreak::NoWrap)
            )]
        ),],
    ));
}

#[derive(Component)]
pub struct FinishTextMarker;

fn spawn_missing_finishes(mut commands: Commands, assets: Res<RequiredAssets>) {
    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(100),
            ..Default::default()
        },
        children![(
            Text::new(""),
            TextFont {
                font: assets.font.clone().unwrap(),
                ..Default::default()
            },
            FinishTextMarker
        )],
    ));
}

fn update_finish_text(
    mut text: Single<&mut Text, With<FinishTextMarker>>,
    finishes: Res<RequiredFinishes>,
) {
    let i = finishes.0;
    if i == 1 {
        text.0 = format!("Collect {i} bone!");
    } else {
        text.0 = format!("Collect {i} bones!");
    }
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

    timer.0.x = timer.2.content_size.x * timer.1.timer.fraction();
}

fn display_end(
    mut commands: Commands,
    start: Res<RunStartTime>,
    assets: Res<RequiredAssets>,
    time: Res<Time>,
) {
    let i = time.elapsed_secs() - start.0;
    commands.spawn((
        DespawnOnExit(LevelScreens::GameEnd),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(90),
            height: percent(90),
            margin: UiRect::all(Val::Auto),
            row_gap: px(10),
            ..Default::default()
        },
        children![
            (
                Node {
                    width: percent(100),
                    height: percent(80),
                    padding: UiRect::all(px(16.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new(format!("You woke up after: {i:?}s")),
                    TextFont {
                        font: assets.font.clone().unwrap(),
                        ..Default::default()
                    },
                )],
            ),
            (
                button(
                    ButtonProps::default(),
                    (),
                    Spawn((
                        Text::new("Back to Main Menu"),
                        TextFont {
                            font: assets.font.clone().unwrap(),
                            ..Default::default()
                        },
                    ))
                ),
                observe(go_to_main),
            ),
        ],
    ));
}
fn go_to_main(
    _: On<Activate>,
    mut current_level: ResMut<CurrentLevel>,
    mut next: ResMut<NextState<LevelScreens>>,
    mut next_main: ResMut<NextState<Screen>>,
) {
    current_level.0 = 0;
    next.set(LevelScreens::None);
    next_main.set(Screen::Main);
}
