//! Opening experience: animated story crawl + crew introductions.
//!
//! The intro is skippable via any key, mouse button, or gamepad action.

use bevy::prelude::*;

use crate::game_state::{AppState, MissionConfig};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IntroState>()
            .add_systems(OnEnter(AppState::Intro), spawn_intro_cinematic)
            .add_systems(OnEnter(AppState::CrewIntro), spawn_crew_intro)
            .add_systems(OnExit(AppState::Intro), despawn_intro_entities)
            .add_systems(OnExit(AppState::CrewIntro), despawn_intro_entities)
            .add_systems(Update, advance_intro.run_if(in_state(AppState::Intro)))
            .add_systems(Update, advance_crew_intro.run_if(in_state(AppState::CrewIntro)));
    }
}

#[derive(Component)]
struct IntroEntity;

#[derive(Component)]
struct IntroLine {
    target_text: String,
    revealed: usize,
    timer: Timer,
}

#[derive(Resource, Default)]
struct IntroState {
    line_index: usize,
    elapsed: Timer,
}

const INTRO_LINES: &[&str] = &[
    "July 1969. The world holds its breath.",
    "Three astronauts climb atop the Saturn V, the most powerful machine ever built by human hands.",
    "Their goal: not orbit, not a pass, but to walk on another world and return alive.",
    "Every switch, every gauge, every alarm has been rehearsed. Yet space keeps its own secrets.",
    "You are Mission Control, the crew, and the machine. History is waiting.",
];

const INTRO_DURATION_PER_LINE: f32 = 4.0;
const INTRO_FADE_DURATION: f32 = 1.5;

fn spawn_intro_cinematic(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let bg = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage::new(asset_server.load("images/menu-bg.jpg")),
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let overlay = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let content = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(80.0)),
                    ..default()
                },
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let line_text = commands
        .spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgb(0.9, 0.9, 0.95),
                },
            )
            .with_text_justify(JustifyText::Center),
            IntroEntity,
            IntroLine {
                target_text: INTRO_LINES[0].to_string(),
                revealed: 0,
                timer: Timer::from_seconds(0.04, TimerMode::Repeating),
            },
        ))
        .id();

    let skip_hint = commands
        .spawn((
            TextBundle::from_section(
                "Press SPACE / ENTER / CLICK to skip",
                TextStyle {
                    font: font_regular,
                    font_size: 14.0,
                    color: Color::srgba(0.7, 0.7, 0.75, 0.6),
                },
            ),
            IntroEntity,
        ))
        .id();

    commands.entity(content).push_children(&[line_text, skip_hint]);
    commands.entity(root).push_children(&[bg, overlay, content]);

    commands.insert_resource(IntroState {
        line_index: 0,
        elapsed: Timer::from_seconds(INTRO_DURATION_PER_LINE, TimerMode::Once),
    });
}

fn advance_intro(
    mut intro: ResMut<IntroState>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut line_query: Query<&mut Text, With<IntroLine>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let skip = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape)
        || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]);

    if skip {
        next_state.set(AppState::MissionSetup);
        return;
    }

    intro.elapsed.tick(time.delta());

    if let Ok(mut text) = line_query.get_single_mut() {
        let target = INTRO_LINES.get(intro.line_index).copied().unwrap_or("");
        if intro.elapsed.just_finished() || intro.line_index >= INTRO_LINES.len() {
            intro.line_index += 1;
            if intro.line_index >= INTRO_LINES.len() {
                next_state.set(AppState::MissionSetup);
                return;
            }
            intro.elapsed = Timer::from_seconds(INTRO_DURATION_PER_LINE, TimerMode::Once);
            text.sections[0].value = INTRO_LINES[intro.line_index].to_string();
        } else if !target.is_empty() {
            // Simple typewriter effect: reveal full line by end of timer.
            let progress = intro.elapsed.elapsed_secs() / INTRO_DURATION_PER_LINE;
            let reveal = (target.len() as f32 * progress.min(1.0)).ceil() as usize;
            text.sections[0].value = target.chars().take(reveal).collect();
        }
    }
}

fn spawn_crew_intro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mission_config: Res<MissionConfig>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.02, 0.03, 0.08)),
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let content = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(80.0)),
                    ..default()
                },
                ..default()
            },
            IntroEntity,
        ))
        .id();

    let title = commands
        .spawn((
            TextBundle::from_section(
                "CREW ASSIGNMENT",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::srgb(0.5, 0.8, 1.0),
                },
            ),
            IntroEntity,
        ))
        .id();

    let cdr = crew_card(&mut commands, &font, &font_regular, "Commander", &mission_config.commander_name, "Flight commander. Pilot of the Lunar Module and final word on the surface.");
    let cmp = crew_card(&mut commands, &font, &font_regular, "Command Module Pilot", &mission_config.cmp_name, "Orbital watchkeeper. Keeps Columbia alive while the others descend.");
    let lmp = crew_card(&mut commands, &font, &font_regular, "Lunar Module Pilot", &mission_config.lmp_name, "Systems expert. Manages propulsion, power, and the desperate climb home.");

    let hint = commands
        .spawn((
            TextBundle::from_section(
                "Press SPACE / ENTER / CLICK to launch",
                TextStyle {
                    font: font_regular,
                    font_size: 14.0,
                    color: Color::srgba(0.7, 0.7, 0.75, 0.6),
                },
            ),
            IntroEntity,
        ))
        .id();

    commands.entity(content).push_children(&[title, cdr, cmp, lmp, hint]);
    commands.entity(root).push_children(&[content]);

    commands.insert_resource(IntroState {
        line_index: 0,
        elapsed: Timer::from_seconds(6.0, TimerMode::Once),
    });
}

fn crew_card(
    commands: &mut Commands,
    font: &Handle<Font>,
    font_regular: &Handle<Font>,
    role: &str,
    name: &str,
    bio: &str,
) -> Entity {
    let display_name = if name.trim().is_empty() { "[Unassigned]" } else { name };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(520.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.05, 0.07, 0.15, 0.95)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            IntroEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("{} — {}", role, display_name),
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgb(0.0, 0.8, 1.0),
                },
            ));
            parent.spawn(TextBundle::from_section(
                bio,
                TextStyle {
                    font: font_regular.clone(),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.85),
                },
            ));
        })
        .id()
}

fn advance_crew_intro(
    mut intro: ResMut<IntroState>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let skip = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape)
        || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]);

    intro.elapsed.tick(time.delta());
    if skip || intro.elapsed.just_finished() {
        next_state.set(AppState::Loading);
    }
}

fn despawn_intro_entities(mut commands: Commands, query: Query<Entity, With<IntroEntity>>) {
    for entity in query.iter() {
        if let Some(e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}
