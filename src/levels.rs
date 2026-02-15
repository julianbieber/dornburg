use bevy::{
    feathers::controls::{ButtonProps, button},
    prelude::*,
    ui_widgets::{Activate, observe},
};

use crate::{screens::Screen, terrain::RequiredFinishes};

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

fn spawn_intermission(mut commands: Commands, _current_level: Res<CurrentLevel>) {
    let intermission_text_0 = "The candle burned low beside the bed, its flame bending as if \
    listening. Shadows pooled in the corners of the chamber like ink reluctant to dry. \
    Upon the small table rested the skull, pale and patient, as though waiting its turn to speak. \
    He watched it with the weary devotion of a sentinel guarding a silent gate. \
    He had long ago convinced himself that bone could remember. \
    Tonight it seemed to remember too much.\n\nCould laughter survive the earth's compression? \
    The skull did not answer, yet its silence shifted shape.";

    let intermission_text_1 = "He keeps the skull close, as though proximity might quiet the \
    ache that followed the lowering of the coffin. Loss has a way of widening the room at night; \
    every corner feels farther than it should, every silence too deliberate. He tells himself \
    that friendship cannot be misplaced, that what was shared must still cling to the bone like \
    a final warmth. And yet doubt seeps in. What if the earth, indifferent and hurried, returned \
    the wrong relic? What if devotion has fastened itself to a stranger's remains? He studies \
    the ridges and hollows for some private signature, but grief blurs the features of memory \
    . The unknown presses against him like a second darkness behind his eyes, and despair \
    whispers that certainty was buried with the body. Even as he mourns, mistrust coils through \
    his sorrow—of the gravediggers, of the scholars, of his own recollection. He longs to believe \
    he keeps vigil over his friend, yet cannot silence the fear that he has been confiding in an \
    impostor, and that his mourning itself has chosen the wrong companion.";

    let intermission_text_2 = "The number returns to him without invitation: twenty-three. It \
    tolls through his thoughts like a muted bell, appearing in the hour he wakes, in the steps from \
    bed to table, in the restless counting of breaths before dawn. He cannot say why it matters, only \
    that it does, as if some hidden arithmetic governs the fate of bones and names. There had been \
    talk, once, of clarification—of records examined and ledgers opened—but the mayor proved too \
    languid in his office, content with wax seals and half-answers, unwilling to disturb the dust of \
    official certainty. A nobleman, affronted by petitions and propriety alike, had barred the way to \
    a more fitting resting place, citing order while practicing obstruction. And so the dead had been \
    gathered without poetry, consigned to a mass grave in Weimar where distinctions dissolved into \
    soil. Twenty-three bodies, perhaps more. Twenty-three chances for error. He cannot escape the \
    suspicion that somewhere in that careless arithmetic, devotion was miscounted, and that he now \
    keeps vigil not over a friend, but over the consequence of another man’s indolence.";

    let intermission_text_3 = "At last he allows the thought to settle without resistance: this
    skull does not belong to the one he knew. Whatever warmth once animated his friend has withdrawn
    beyond retrieval, and no vigil, however faithful, can summon it back into bone. The features he
    studies so intently yield nothing familiar; they are a geography without memory. The friend is
    gone—not misplaced, not mislabeled, but gone in the only way that admits no correction.\n \nTime, \
    indifferent and meticulous, will sort the matter as it sorts all things. It will peel away
    conjecture and devotion alike, leaving only the stark outline of events. Records will outlive
    intention. Whispers will harden into anecdote, and anecdote into verdict. In that slow unfolding, \
    his motives will matter less than the spectacle.\n\nHe sees already how the story will be told. \
    Not of loyalty strained by doubt, nor of grief driven to desperate custody, but of theft. He will \
    be remembered as the man who stole and kept a skull to which he had no rightful claim, guarding \
    it with a scholar’s obsession and a mourner’s folly. Whatever tenderness once justified his vigil \
    will fade, and in its place will stand the simpler tale: that he clung to a relic that was never \
    his, and made of it both companion and crime.";

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
                children![(Text::new(texts[_current_level.0 as usize]),)],
            ),
            (
                button(ButtonProps::default(), (), Spawn(Text::new("next level"))),
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

#[derive(Component)]
pub struct FinishTextMarker;

fn spawn_missing_finishes(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(LevelScreens::Level),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(100),
            ..Default::default()
        },
        children![(Text::new(""), FinishTextMarker)],
    ));
}

fn update_finish_text(
    mut text: Single<&mut Text, With<FinishTextMarker>>,
    finishes: Res<RequiredFinishes>,
) {
    let i = finishes.0;
    if i == 0 {
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

    // let total_time = timer.1.timer.duration().as_secs_f32();

    timer.0.x = timer.2.content_size.x * timer.1.timer.fraction();

    // timer.0.0 = content;
}
