use bevy::{input::ButtonState, input::keyboard::{KeyboardInput, Key}, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuFocus>()
            .init_resource::<MenuInputState>()
            .add_systems(OnEnter(crate::game_state::AppState::MainMenu), spawn_main_menu)
            .add_systems(OnEnter(crate::game_state::AppState::MainMenu), spawn_menu_camera)
            .add_systems(OnExit(crate::game_state::AppState::MainMenu), despawn_menu_entities)
            .add_systems(OnExit(crate::game_state::AppState::MainMenu), despawn_menu_camera)
            .add_systems(OnEnter(crate::game_state::AppState::MissionSetup), spawn_mission_setup)
            .add_systems(OnEnter(crate::game_state::AppState::MissionSetup), spawn_menu_camera)
            .add_systems(OnExit(crate::game_state::AppState::MissionSetup), despawn_menu_entities)
            .add_systems(OnExit(crate::game_state::AppState::MissionSetup), despawn_menu_camera)
            .add_systems(OnEnter(crate::game_state::AppState::Paused), spawn_pause_menu)
            .add_systems(OnEnter(crate::game_state::AppState::Paused), spawn_menu_camera)
            .add_systems(OnExit(crate::game_state::AppState::Paused), despawn_menu_entities)
            .add_systems(OnExit(crate::game_state::AppState::Paused), despawn_menu_camera)
            .add_systems(OnEnter(crate::game_state::AppState::GameOver), spawn_game_over)
            .add_systems(OnEnter(crate::game_state::AppState::GameOver), spawn_menu_camera)
            .add_systems(OnExit(crate::game_state::AppState::GameOver), despawn_menu_entities)
            .add_systems(OnExit(crate::game_state::AppState::GameOver), despawn_menu_camera)
            .add_systems(Update, menu_button_interaction.run_if(in_menu_state))
            .add_systems(Update, menu_keyboard_navigation.run_if(in_menu_state))
            .add_systems(Update, menu_keyboard_activation.run_if(in_menu_state))
            .add_systems(Update, menu_text_input.run_if(in_state(crate::game_state::AppState::MissionSetup)))
            .add_systems(Update, menu_action_handler.run_if(in_menu_state).after(menu_keyboard_activation))
            .add_systems(Update, update_text_input_display.run_if(in_state(crate::game_state::AppState::MissionSetup)))
            .add_systems(Update, update_launch_button_state.run_if(in_state(crate::game_state::AppState::MissionSetup)))
            .add_systems(Update, update_mission_selector_state.run_if(in_state(crate::game_state::AppState::MissionSetup)))
            .add_systems(Update, update_difficulty_selector_state.run_if(in_state(crate::game_state::AppState::MissionSetup)))
        .add_systems(Update, update_houston_selector_state.run_if(in_state(crate::game_state::AppState::MissionSetup)));
    }
}

fn in_menu_state(state: Res<State<crate::game_state::AppState>>) -> bool {
    matches!(
        state.get(),
        crate::game_state::AppState::MainMenu
            | crate::game_state::AppState::MissionSetup
            | crate::game_state::AppState::Paused
            | crate::game_state::AppState::GameOver
    )
}

#[derive(Component)]
struct MenuEntity;

#[derive(Component)]
struct MenuCamera;

#[derive(Component)]
struct MenuOverlayPanel;

#[derive(Component)]
struct FocusedButton;

#[derive(Component)]
struct MenuButton {
    action: MenuAction,
}

#[derive(Component)]
struct TextInput {
    field: InputField,
    value: String,
    focused: bool,
    cursor_blink: f32,
}

#[derive(Component)]
struct TextInputDisplay;

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputField {
    Commander,
    Cmp,
    Lmp,
}

#[derive(Clone)]
enum MenuAction {
    NewMission,
    Settings,
    Quit,
    Launch,
    Back,
    Resume,
    ReturnToMenu,
    SelectMission(String),
    SelectDifficulty(crate::game_state::Difficulty),
    SelectHoustonMode(crate::game_state::HoustonMode),
    ShowSavedGames,
    ShowComputerOptions,
    CloseOverlay,
}

#[derive(Resource, Default, Debug, Clone)]
struct MenuFocus {
    focus_index: usize,
    total_focusable: usize,
}

#[derive(Resource, Default, Debug, Clone)]
struct MenuInputState {
    just_tabbed: bool,
}

const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.08);
const PANEL_BG: Color = Color::srgba(0.05, 0.07, 0.15, 0.95);
const BUTTON_BG: Color = Color::srgb(0.1, 0.15, 0.25);
const BUTTON_HOVER: Color = Color::srgb(0.2, 0.3, 0.5);
const BUTTON_FOCUS: Color = Color::srgb(0.25, 0.4, 0.6);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.95);
const TITLE_COLOR: Color = Color::srgb(0.5, 0.8, 1.0);
const ACCENT_COLOR: Color = Color::srgb(0.0, 0.8, 1.0);
const DISABLED_COLOR: Color = Color::srgb(0.3, 0.3, 0.35);

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                ..default()
            },
            MenuEntity,
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
            MenuEntity,
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
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ..default()
            },
            MenuEntity,
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
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let title = spawn_text(
        &mut commands,
        &font,
        "APOLLO MISSION SIMULATOR",
        48.0,
        TITLE_COLOR,
    );

    let subtitle = spawn_text(
        &mut commands,
        &font_regular,
        "A Historically Accurate Spaceflight Experience",
        18.0,
        Color::srgb(0.6, 0.7, 0.8),
    );

    let btn_new = spawn_menu_button(&mut commands, &font, "NEW GAME", MenuAction::NewMission, true);
    let btn_saved = spawn_menu_button(&mut commands, &font, "SAVED GAMES", MenuAction::ShowSavedGames, false);
    let btn_options = spawn_menu_button(&mut commands, &font, "COMPUTER OPTIONS", MenuAction::ShowComputerOptions, false);
    let btn_quit = spawn_menu_button(&mut commands, &font, "QUIT", MenuAction::Quit, false);

    let author = spawn_text(
        &mut commands,
        &font_regular,
        "by Andrew White",
        14.0,
        Color::srgb(0.5, 0.6, 0.7),
    );

    let copyright = spawn_text(
        &mut commands,
        &font_regular,
        "Copyright 2026 Andrew White  |  Licensed under GPL v2+",
        12.0,
        Color::srgb(0.4, 0.45, 0.5),
    );

    commands.entity(content).push_children(&[title, subtitle, btn_new, btn_saved, btn_options, btn_quit, author, copyright]);
    commands.entity(root).push_children(&[bg, overlay, content]);
}

fn spawn_mission_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mission_config: Res<crate::game_state::MissionConfig>,
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
                ..default()
            },
            MenuEntity,
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
            MenuEntity,
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
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ..default()
            },
            MenuEntity,
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
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let title = spawn_text(&mut commands, &font, "MISSION SETUP", 36.0, TITLE_COLOR);

    let panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(500.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BG),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let mission_label = spawn_text(&mut commands, &font_regular, "Mission", 16.0, ACCENT_COLOR);
    let mission_row = spawn_mission_selector(&mut commands, &font, &font_regular, &mission_config.mission_id);

    let difficulty_label = spawn_text(&mut commands, &font_regular, "Difficulty", 16.0, ACCENT_COLOR);
    let difficulty_row = spawn_difficulty_selector(&mut commands, &font, &font_regular, mission_config.difficulty);

    let houston_label = spawn_text(&mut commands, &font_regular, "Houston Radio", 16.0, ACCENT_COLOR);
    let houston_row = spawn_houston_selector(&mut commands, &font, &font_regular, mission_config.houston_mode);

    let crew_label = spawn_text(&mut commands, &font_regular, "Crew", 16.0, ACCENT_COLOR);

    let cdr_input = spawn_text_input(
        &mut commands,
        &font_regular,
        InputField::Commander,
        "Commander",
        &mission_config.commander_name,
    );
    let cmp_input = spawn_text_input(
        &mut commands,
        &font_regular,
        InputField::Cmp,
        "Command Module Pilot",
        &mission_config.cmp_name,
    );
    let lmp_input = spawn_text_input(
        &mut commands,
        &font_regular,
        InputField::Lmp,
        "Lunar Module Pilot",
        &mission_config.lmp_name,
    );

    let btn_launch = spawn_menu_button(&mut commands, &font, "LAUNCH", MenuAction::Launch, true);
    let btn_back = spawn_menu_button(&mut commands, &font, "Back", MenuAction::Back, false);

    commands
        .entity(panel)
        .push_children(&[
            mission_label,
            mission_row,
            difficulty_label,
            difficulty_row,
            houston_label,
            houston_row,
            crew_label,
            cdr_input,
            cmp_input,
            lmp_input,
            btn_launch,
            btn_back,
        ]);

    commands.entity(content).push_children(&[title, panel]);
    commands.entity(root).push_children(&[bg, overlay, content]);
}

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let title = spawn_text(&mut commands, &font, "PAUSED", 48.0, TITLE_COLOR);
    let btn_resume = spawn_menu_button(&mut commands, &font, "Resume", MenuAction::Resume, true);
    let btn_menu = spawn_menu_button(&mut commands, &font, "Return to Menu", MenuAction::ReturnToMenu, false);

    commands.entity(root).push_children(&[title, btn_resume, btn_menu]);
}

fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_stats: Res<crate::game_state::GameStats>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    spawn_starfield(&mut commands);

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(BG_COLOR),
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let result_text = match game_stats.mission_result {
        crate::game_state::MissionResult::Success => "MISSION SUCCESS",
        _ => "MISSION FAILURE",
    };
    let result_color = match game_stats.mission_result {
        crate::game_state::MissionResult::Success => Color::srgb(0.2, 1.0, 0.3),
        _ => Color::srgb(1.0, 0.2, 0.2),
    };

    let title = spawn_text(&mut commands, &font, result_text, 48.0, result_color);

    let stats_panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BG),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let hrs = (game_stats.mission_time_seconds / 3600.0) as u32;
    let mins = ((game_stats.mission_time_seconds % 3600.0) / 60.0) as u32;
    let time_str = format!("Mission Time: {:02}:{:02}", hrs, mins);

    let time_text = spawn_text(&mut commands, &font_regular, &time_str, 20.0, TEXT_COLOR);
    let faults_text = spawn_text(
        &mut commands,
        &font_regular,
        &format!("Faults Encountered: {}", game_stats.faults_encountered),
        20.0,
        TEXT_COLOR,
    );
    let repairs_text = spawn_text(
        &mut commands,
        &font_regular,
        &format!("Repairs Performed: {}", game_stats.repairs_performed),
        20.0,
        TEXT_COLOR,
    );

    let btn_menu = spawn_menu_button(&mut commands, &font, "Return to Menu", MenuAction::ReturnToMenu, true);

    commands
        .entity(stats_panel)
        .push_children(&[time_text, faults_text, repairs_text]);
    commands.entity(root).push_children(&[title, stats_panel, btn_menu]);
}

fn spawn_saved_games_overlay(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    let overlay = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BG),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let title = spawn_text(commands, &font, "SAVED GAMES", 28.0, TITLE_COLOR);
    let msg = spawn_text(commands, &font_regular, "Saved games feature coming soon", 18.0, TEXT_COLOR);
    let btn_back = spawn_menu_button(commands, &font, "BACK", MenuAction::CloseOverlay, true);

    commands.entity(panel).push_children(&[title, msg, btn_back]);
    commands.entity(overlay).push_children(&[panel]);
}

fn spawn_error_overlay(commands: &mut Commands, asset_server: &AssetServer, message: &str) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    let overlay = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.05, 0.05)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let title = spawn_text(commands, &font, "CANNOT LAUNCH", 24.0, Color::srgb(1.0, 0.3, 0.3));
    let msg = spawn_text(commands, &font_regular, message, 16.0, TEXT_COLOR);
    let btn_back = spawn_menu_button(commands, &font, "OK", MenuAction::CloseOverlay, true);

    commands.entity(panel).push_children(&[title, msg, btn_back]);
    commands.entity(overlay).push_children(&[panel]);
}

fn spawn_computer_options_overlay(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_regular = asset_server.load("fonts/FiraSans-Regular.ttf");

    let overlay = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BG),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            MenuOverlayPanel,
            MenuEntity,
        ))
        .id();

    let title = spawn_text(commands, &font, "COMPUTER OPTIONS", 28.0, TITLE_COLOR);
    let msg1 = spawn_text(commands, &font_regular, "Graphics and audio settings", 18.0, TEXT_COLOR);
    let msg2 = spawn_text(commands, &font_regular, "Fullscreen toggle: placeholder", 16.0, DISABLED_COLOR);
    let btn_back = spawn_menu_button(commands, &font, "BACK", MenuAction::CloseOverlay, true);

    commands.entity(panel).push_children(&[title, msg1, msg2, btn_back]);
    commands.entity(overlay).push_children(&[panel]);
}

fn despawn_menu_entities(
    mut commands: Commands,
    query: Query<Entity, With<MenuEntity>>,
) {
    for entity in query.iter() {
        if let Some(mut e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}

fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        MenuCamera,
    ));
}

fn despawn_menu_camera(
    mut commands: Commands,
    query: Query<Entity, With<MenuCamera>>,
) {
    for entity in query.iter() {
        if let Some(mut e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}

fn spawn_starfield(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    use rand::Rng;

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(BG_COLOR),
                ..default()
            },
            MenuEntity,
        ))
        .id();

    for _ in 0..200 {
        let x: f32 = rng.gen_range(0.0..100.0);
        let y: f32 = rng.gen_range(0.0..100.0);
        let size: f32 = rng.gen_range(1.0..3.0);
        let brightness: f32 = rng.gen_range(0.5..1.0);

        let star = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(brightness, brightness, brightness * 0.95)),
                    border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                    ..default()
                },
                MenuEntity,
            ))
            .id();

        commands.entity(root).add_child(star);
    }
}

fn spawn_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
) -> Entity {
    commands
        .spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size,
                    color,
                },
            ),
            MenuEntity,
        ))
        .id()
}

fn spawn_menu_button(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: &str,
    action: MenuAction,
    focused: bool,
) -> Entity {
    let bg = if focused { BUTTON_FOCUS } else { BUTTON_BG };

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(240.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(bg),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            MenuButton { action },
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: TEXT_COLOR,
                },
            ));
        })
        .id()
}

fn spawn_mission_selector(
    commands: &mut Commands,
    font: &Handle<Font>,
    font_regular: &Handle<Font>,
    selected: &str,
) -> Entity {
    let row = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let missions = [("apollo11", "Apollo 11 (Landing)"), ("apollo13", "Apollo 13 (Survival)"), ("freeflight", "Free Flight")];

    let mut children = Vec::new();
    for (id, label) in missions {
        let is_selected = selected == id;
        let bg = if is_selected { ACCENT_COLOR } else { BUTTON_BG };
        let text_color = if is_selected { Color::BLACK } else { TEXT_COLOR };

        let btn = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(bg),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                MenuButton {
                    action: MenuAction::SelectMission(id.to_string()),
                },
                MenuEntity,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: if is_selected { font.clone() } else { font_regular.clone() },
                        font_size: 14.0,
                        color: text_color,
                    },
                ));
            })
            .id();
        children.push(btn);
    }

    commands.entity(row).push_children(&children);
    row
}

fn spawn_difficulty_selector(
    commands: &mut Commands,
    font: &Handle<Font>,
    font_regular: &Handle<Font>,
    selected: crate::game_state::Difficulty,
) -> Entity {
    let row = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let difficulties = [
        (crate::game_state::Difficulty::Normal, "Normal"),
        (crate::game_state::Difficulty::Hard, "Hard"),
        (crate::game_state::Difficulty::Realistic, "Realistic"),
    ];

    let mut children = Vec::new();
    for (diff, label) in difficulties {
        let is_selected = selected == diff;
        let bg = if is_selected { ACCENT_COLOR } else { BUTTON_BG };
        let text_color = if is_selected { Color::BLACK } else { TEXT_COLOR };

        let btn = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(bg),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                MenuButton {
                    action: MenuAction::SelectDifficulty(diff),
                },
                MenuEntity,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: if is_selected { font.clone() } else { font_regular.clone() },
                        font_size: 14.0,
                        color: text_color,
                    },
                ));
            })
            .id();
        children.push(btn);
    }

    commands.entity(row).push_children(&children);
    row
}

fn spawn_houston_selector(
    commands: &mut Commands,
    font: &Handle<Font>,
    font_regular: &Handle<Font>,
    selected: crate::game_state::HoustonMode,
) -> Entity {
    let row = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let enhanced_available = crate::game_state::HoustonMode::enhanced_available();
    let modes = [
        (crate::game_state::HoustonMode::Classic, "Classic", true),
        (crate::game_state::HoustonMode::Enhanced, "Enhanced", enhanced_available),
    ];

    let mut children = Vec::new();
    for (mode, label, enabled) in modes {
        let is_selected = selected == mode;
        let bg = if !enabled {
            DISABLED_COLOR
        } else if is_selected {
            ACCENT_COLOR
        } else {
            BUTTON_BG
        };
        let text_color = if !enabled {
            DISABLED_COLOR
        } else if is_selected {
            Color::BLACK
        } else {
            TEXT_COLOR
        };

        let btn = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(bg),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                MenuButton {
                    action: MenuAction::SelectHoustonMode(mode),
                },
                MenuEntity,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: if is_selected && enabled { font.clone() } else { font_regular.clone() },
                        font_size: 14.0,
                        color: text_color,
                    },
                ));
            })
            .id();
        children.push(btn);
    }

    commands.entity(row).push_children(&children);
    row
}

fn spawn_text_input(
    commands: &mut Commands,
    font: &Handle<Font>,
    field: InputField,
    label: &str,
    value: &str,
) -> Entity {
    let row = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .id();

    let label_entity = commands
        .spawn((
            TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: TEXT_COLOR,
                },
            ),
            MenuEntity,
        ))
        .id();

    let input_box = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(220.0),
                    height: Val::Px(36.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.08, 0.1, 0.18)),
                border_color: BorderColor(Color::srgb(0.3, 0.4, 0.5)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            TextInput {
                field,
                value: value.to_string(),
                focused: false,
                cursor_blink: 0.0,
            },
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    value,
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: TEXT_COLOR,
                    },
                ),
                TextInputDisplay,
                MenuEntity,
            ));
        })
        .id();

    commands.entity(row).push_children(&[label_entity, input_box]);
    row
}

fn menu_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton, Option<&FocusedButton>),
        (With<Button>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg, _button, focused) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(BUTTON_HOVER);
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(ACCENT_COLOR);
            }
            Interaction::None => {
                if focused.is_some() {
                    *bg = BackgroundColor(BUTTON_FOCUS);
                } else {
                    *bg = BackgroundColor(BUTTON_BG);
                }
            }
        }
    }
}

fn menu_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut focus: ResMut<MenuFocus>,
    mut menu_state: ResMut<MenuInputState>,
    mut text_input_query: Query<(Entity, &mut TextInput)>,
    button_query: Query<(Entity, Option<&FocusedButton>), With<Button>>,
    mut commands: Commands,
) {
    let count = button_query.iter().count() + text_input_query.iter().count();
    focus.total_focusable = count;

    if count == 0 {
        return;
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

        if shift {
            focus.focus_index = focus.focus_index.saturating_sub(1);
            if focus.focus_index == 0 && !menu_state.just_tabbed {
                focus.focus_index = count.saturating_sub(1);
            }
        } else {
            focus.focus_index = (focus.focus_index + 1) % count;
        }

        menu_state.just_tabbed = true;
    } else {
        menu_state.just_tabbed = false;
    }

    let mut idx = 0;
    for (_entity, mut text_input) in text_input_query.iter_mut() {
        text_input.focused = idx == focus.focus_index;
        idx += 1;
    }
    for (entity, focused) in button_query.iter() {
        let is_focused = idx == focus.focus_index;
        if is_focused && focused.is_none() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(FocusedButton);
            }
        } else if !is_focused && focused.is_some() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<FocusedButton>();
            }
        }
        idx += 1;
    }
}

fn menu_keyboard_activation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut focused_button_query: Query<&mut Interaction, (With<Button>, With<FocusedButton>)>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        for mut interaction in focused_button_query.iter_mut() {
            *interaction = Interaction::Pressed;
        }
    }
}

fn menu_text_input(
    mut key_events: EventReader<KeyboardInput>,
    mut text_input_query: Query<&mut TextInput>,
    mut mission_config: ResMut<crate::game_state::MissionConfig>,
) {
    for mut text_input in text_input_query.iter_mut() {
        if !text_input.focused {
            continue;
        }

        for event in key_events.read() {
            if event.state != ButtonState::Pressed {
                continue;
            }
            match &event.logical_key {
                Key::Character(ch) => {
                    if ch.is_ascii() && ch.len() == 1 && !ch.chars().next().unwrap().is_control() {
                        text_input.value.push_str(ch);
                    }
                }
                Key::Space => {
                    text_input.value.push(' ');
                }
                Key::Backspace => {
                    text_input.value.pop();
                }
                _ => {}
            }
        }

        match text_input.field {
            InputField::Commander => mission_config.commander_name = text_input.value.clone(),
            InputField::Cmp => mission_config.cmp_name = text_input.value.clone(),
            InputField::Lmp => mission_config.lmp_name = text_input.value.clone(),
        }
    }
}

fn update_text_input_display(
    text_input_query: Query<(&TextInput, &Children)>,
    mut text_display_query: Query<&mut Text, With<TextInputDisplay>>,
) {
    for (text_input, children) in text_input_query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_display_query.get_mut(*child) {
                let display = if text_input.focused {
                    format!("{}|", text_input.value)
                } else {
                    text_input.value.clone()
                };
                for section in text.sections.iter_mut() {
                    section.value = display.clone();
                }
            }
        }
    }
}

fn update_launch_button_state(
    mission_config: Res<crate::game_state::MissionConfig>,
    mut button_query: Query<(&MenuButton, &mut BorderColor, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (button, mut border, children) in button_query.iter_mut() {
        if let MenuAction::Launch = &button.action {
            let enabled = mission_config.all_crew_named();
            if enabled {
                *border = BorderColor(ACCENT_COLOR);
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        for section in text.sections.iter_mut() {
                            section.style.color = TEXT_COLOR;
                        }
                    }
                }
            } else {
                *border = BorderColor(Color::srgb(0.15, 0.2, 0.25));
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        for section in text.sections.iter_mut() {
                            section.style.color = DISABLED_COLOR;
                    }
                }
            }
        }
    }
}

fn menu_mouse_click_handler(
    mouse_button: Res<ButtonInput<MouseButton>>,
    button_query: Query<(&Interaction, &MenuButton)>,
    mut next_state: ResMut<NextState<crate::game_state::AppState>>,
    mut mission_config: ResMut<crate::game_state::MissionConfig>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_stats: ResMut<crate::game_state::GameStats>,
    game_world_query: Query<Entity, With<crate::game_state::GameWorldEntity>>,
    mut commands: Commands,
    overlay_query: Query<Entity, With<MenuOverlayPanel>>,
    asset_server: Res<AssetServer>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            match &button.action {
                MenuAction::NewMission => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    mission_config.commander_name.clear();
                    mission_config.cmp_name.clear();
                    mission_config.lmp_name.clear();
                    next_state.set(crate::game_state::AppState::MissionSetup);
                }
                MenuAction::Settings => {}
                MenuAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuAction::Launch => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    next_state.set(crate::game_state::AppState::Loading);
                }
                MenuAction::Back => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    next_state.set(crate::game_state::AppState::MainMenu);
                }
                MenuAction::Resume => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    next_state.set(crate::game_state::AppState::InGame);
                }
                MenuAction::ReturnToMenu => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    for entity in game_world_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    *game_stats = crate::game_state::GameStats::default();
                    next_state.set(crate::game_state::AppState::MainMenu);
                }
                MenuAction::SelectMission(id) => {
                    mission_config.mission_id = id.clone();
                }
                MenuAction::SelectDifficulty(diff) => {
                    mission_config.difficulty = *diff;
                }
                MenuAction::SelectHoustonMode(mode) => {
                    let can_use = match mode {
                        crate::game_state::HoustonMode::Classic => true,
                        crate::game_state::HoustonMode::Enhanced => {
                            crate::game_state::HoustonMode::enhanced_available()
                        }
                    };
                    if can_use {
                        mission_config.houston_mode = *mode;
                    }
                }
                MenuAction::ShowSavedGames => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    spawn_saved_games_overlay(&mut commands, &asset_server);
                }
                MenuAction::ShowComputerOptions => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                    spawn_computer_options_overlay(&mut commands, &asset_server);
                }
                MenuAction::CloseOverlay => {
                    for entity in overlay_query.iter() {
                        if let Some(mut e) = commands.get_entity(entity) {
                            e.despawn_recursive();
                        }
                    }
                }
            }
            break;
        }
    }
}
}

fn update_mission_selector_state(
    mission_config: Res<crate::game_state::MissionConfig>,
    button_query: Query<(&MenuButton, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if !mission_config.is_changed() {
        return;
    }
    for (button, children) in button_query.iter() {
        if let MenuAction::SelectMission(id) = &button.action {
            let is_selected = mission_config.mission_id == *id;
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = if is_selected { Color::BLACK } else { TEXT_COLOR };
                    }
                }
            }
        }
    }
}

fn update_difficulty_selector_state(
    mission_config: Res<crate::game_state::MissionConfig>,
    button_query: Query<(&MenuButton, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if !mission_config.is_changed() {
        return;
    }
    for (button, children) in button_query.iter() {
        if let MenuAction::SelectDifficulty(diff) = &button.action {
            let is_selected = mission_config.difficulty == *diff;
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = if is_selected { Color::BLACK } else { TEXT_COLOR };
                    }
                }
            }
        }
    }
}

fn update_houston_selector_state(
    mission_config: Res<crate::game_state::MissionConfig>,
    button_query: Query<(&MenuButton, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if !mission_config.is_changed() {
        return;
    }
    let enhanced_available = crate::game_state::HoustonMode::enhanced_available();
    for (button, children) in button_query.iter() {
        if let MenuAction::SelectHoustonMode(mode) = &button.action {
            let is_selected = mission_config.houston_mode == *mode;
            let enabled = match mode {
                crate::game_state::HoustonMode::Classic => true,
                crate::game_state::HoustonMode::Enhanced => enhanced_available,
            };
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = if !enabled {
                            DISABLED_COLOR
                        } else if is_selected {
                            Color::BLACK
                        } else {
                            TEXT_COLOR
                        };
                    }
                }
            }
        }
    }
}

fn menu_action_handler(
    mouse_button: Res<ButtonInput<MouseButton>>,
    button_query: Query<(&Interaction, &MenuButton)>,
    mut next_state: ResMut<NextState<crate::game_state::AppState>>,
    mut mission_config: ResMut<crate::game_state::MissionConfig>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_stats: ResMut<crate::game_state::GameStats>,
    game_world_query: Query<Entity, With<crate::game_state::GameWorldEntity>>,
    mut commands: Commands,
    overlay_query: Query<Entity, With<MenuOverlayPanel>>,
    asset_server: Res<AssetServer>,
) {
    let mouse_clicked = mouse_button.just_pressed(MouseButton::Left);
    for (interaction, button) in button_query.iter() {
        if *interaction != Interaction::Pressed && !(mouse_clicked && *interaction == Interaction::Hovered) {
            continue;
        }

        match &button.action {
            MenuAction::NewMission => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                mission_config.commander_name.clear();
                mission_config.cmp_name.clear();
                mission_config.lmp_name.clear();
                next_state.set(crate::game_state::AppState::MissionSetup);
            }
            MenuAction::Settings => {}
            MenuAction::Quit => {
                app_exit_events.send(AppExit::Success);
            }
            MenuAction::Launch => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                if mission_config.all_crew_named() {
                    next_state.set(crate::game_state::AppState::Loading);
                } else {
                    spawn_error_overlay(&mut commands, &asset_server, "Please name all three crew members before launching.");
                }
            }
            MenuAction::Back => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                next_state.set(crate::game_state::AppState::MainMenu);
            }
            MenuAction::Resume => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                next_state.set(crate::game_state::AppState::InGame);
            }
            MenuAction::ReturnToMenu => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                for entity in game_world_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                *game_stats = crate::game_state::GameStats::default();
                next_state.set(crate::game_state::AppState::MainMenu);
            }
            MenuAction::SelectMission(id) => {
                mission_config.mission_id = id.clone();
            }
            MenuAction::SelectDifficulty(diff) => {
                mission_config.difficulty = *diff;
            }
            MenuAction::SelectHoustonMode(mode) => {
                let can_use = match mode {
                    crate::game_state::HoustonMode::Classic => true,
                    crate::game_state::HoustonMode::Enhanced => {
                        crate::game_state::HoustonMode::enhanced_available()
                    }
                };
                if can_use {
                    mission_config.houston_mode = *mode;
                }
            }
            MenuAction::ShowSavedGames => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                spawn_saved_games_overlay(&mut commands, &asset_server);
            }
            MenuAction::ShowComputerOptions => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
                spawn_computer_options_overlay(&mut commands, &asset_server);
            }
            MenuAction::CloseOverlay => {
                for entity in overlay_query.iter() {
                    if let Some(mut e) = commands.get_entity(entity) {
                        e.despawn_recursive();
                    }
                }
            }
        }
    }
}
