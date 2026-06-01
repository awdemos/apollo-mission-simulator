use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;

/// Application state machine for the Apollo Mission Simulator.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    MissionSetup,
    Loading,
    InGame,
    Paused,
    GameOver,
}

/// Plugin that wires up the state machine and global state-related systems.
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<MissionConfig>()
            .init_resource::<GameStats>()
            .init_resource::<MasterAlarm>()
            .add_systems(Update, handle_pause_toggle)
            .add_systems(Update, check_game_over.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::Paused), show_cursor)
            .add_systems(OnEnter(AppState::MainMenu), show_cursor)
            .add_systems(OnEnter(AppState::MissionSetup), show_cursor)
            .add_systems(OnEnter(AppState::Loading), spawn_game_world)
            .add_systems(Update, transition_loading_to_ingame.run_if(in_state(AppState::Loading)))
            .add_systems(OnEnter(AppState::GameOver), show_cursor);
    }
}

/// Configuration chosen during MissionSetup.
#[derive(Resource, Debug, Clone)]
pub struct MissionConfig {
    pub mission_id: String,
    pub difficulty: Difficulty,
    pub houston_mode: HoustonMode,
    pub commander_name: String,
    pub cmp_name: String,
    pub lmp_name: String,
}

impl Default for MissionConfig {
    fn default() -> Self {
        Self {
            mission_id: "apollo11".to_string(),
            difficulty: Difficulty::Normal,
            houston_mode: HoustonMode::default(),
            commander_name: String::new(),
            cmp_name: String::new(),
            lmp_name: String::new(),
        }
    }
}

impl MissionConfig {
    pub fn all_crew_named(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Normal,
    Hard,
    Realistic,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Normal
    }
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Normal => "NORMAL",
            Difficulty::Hard => "HARD",
            Difficulty::Realistic => "REALISTIC",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Difficulty::Normal => "More forgiving faults, full ground control assistance",
            Difficulty::Hard => "Realistic fault rates, partial ground control",
            Difficulty::Realistic => "Historical accuracy, comm blackouts, no HUD assistance",
        }
    }
}

/// Houston Radio mode for LLM-enhanced ground control dialogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HoustonMode {
    /// Scripted dialogue only (no LLM).
    #[default]
    Classic,
    /// LLM-enhanced Houston responses via OpenAI-compatible API.
    Enhanced,
}

impl HoustonMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoustonMode::Classic => "CLASSIC",
            HoustonMode::Enhanced => "ENHANCED",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            HoustonMode::Classic => "Classic",
            HoustonMode::Enhanced => "Enhanced",
        }
    }

    pub fn enhanced_available() -> bool {
        std::env::var("HOUSTON_LLM_API_KEY").is_ok()
    }
}

/// Statistics tracked during a mission.
#[derive(Resource, Default, Debug, Clone)]
pub struct GameStats {
    pub mission_time_seconds: f64,
    pub faults_encountered: u32,
    pub repairs_performed: u32,
    pub mission_result: MissionResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissionResult {
    #[default]
    InProgress,
    Success,
    Failure,
}

/// Master alarm triggered when crew health reaches Critical.
#[derive(Resource, Debug, Clone, Default)]
pub struct MasterAlarm {
    pub active: bool,
    pub triggered_by: Option<String>,
}

/// Marker component for entities that belong to the active game world.
/// Used to despawn everything when returning to menu.
#[derive(Component)]
pub struct GameWorldEntity;

fn handle_pause_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::InGame => {
                next_state.set(AppState::Paused);
            }
            AppState::Paused => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.cursor.visible = true;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
    }
}

fn check_game_over(
    crew_query: Query<&crate::crew::CrewMember>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_stats: ResMut<GameStats>,
    mission_state: Res<crate::mission::MissionState>,
    fault_manager: Res<crate::faults::FaultManager>,
) {
    let total_crew = crew_query.iter().count();
    let deceased_count = crew_query
        .iter()
        .filter(|c| matches!(c.health.status, crate::crew::HealthStatus::Deceased))
        .count();

    if total_crew > 0 && deceased_count == total_crew {
        game_stats.mission_time_seconds = mission_state.mission_time;
        game_stats.faults_encountered = fault_manager.fault_history.len() as u32;
        game_stats.repairs_performed = fault_manager
            .fault_history
            .iter()
            .filter(|r| r.resolved_at.is_some())
            .count() as u32;
        game_stats.mission_result = MissionResult::Failure;
        next_state.set(AppState::GameOver);
        return;
    }

    let catastrophic = fault_manager
        .active_faults
        .iter()
        .any(|f| matches!(f.severity, crate::faults::FaultSeverity::Catastrophic));
    if catastrophic {
        game_stats.mission_time_seconds = mission_state.mission_time;
        game_stats.faults_encountered = fault_manager.fault_history.len() as u32;
        game_stats.repairs_performed = fault_manager
            .fault_history
            .iter()
            .filter(|r| r.resolved_at.is_some())
            .count() as u32;
        game_stats.mission_result = MissionResult::Failure;
        next_state.set(AppState::GameOver);
    }
}

fn spawn_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mission_config: Res<MissionConfig>,
    mut mission_state: ResMut<crate::mission::MissionState>,
    mut fault_manager: ResMut<crate::faults::FaultManager>,
    cache: Res<crate::spacecraft::CmMeshCache>,
    existing_camera: Query<Entity, With<Camera3d>>,
) {
    mission_state.mission_id = mission_config.mission_id.clone();
    mission_state.phase_index = 0;
    mission_state.step_index = 0;
    mission_state.mission_time = 0.0;
    mission_state.is_running = true;
    mission_state.log.clear();

    fault_manager.load_scripted_faults(&mission_config.mission_id);
    fault_manager.set_difficulty(match mission_config.difficulty {
        Difficulty::Normal => 1,
        Difficulty::Hard => 2,
        Difficulty::Realistic => 3,
    });

    crate::spacecraft::spawn_apollo_stack(&mut commands, &mut meshes, &mut materials, &cache);

    let lat = crate::world::LAUNCH_SITE_LAT;
    let lon = crate::world::LAUNCH_SITE_LON;
    let r = crate::world::EARTH_RADIUS;
    let rocket_x = r * lat.cos() * lon.cos();
    let rocket_y = r * lat.sin();
    let rocket_z = r * lat.cos() * lon.sin();
    let rocket_pos = Vec3::new(rocket_x, rocket_y, rocket_z);
        let up = rocket_pos.normalize();
        let world_up = Vec3::Y;

    if existing_camera.iter().next().is_none() {
        let east = up.cross(world_up).normalize();
        let north = east.cross(up);
        let cam_offset = up * 8.0 + north * 45.0 + east * 30.0;
        let cam_pos = rocket_pos + cam_offset;

        commands.spawn((
            Camera3dBundle {
                camera: Camera {
                    order: 0,
                    clear_color: ClearColorConfig::Default,
                    ..default()
                },
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 45f32.to_radians(),
                    near: 0.001,
                    far: 10000.0,
                    ..default()
                }),
                exposure: bevy::render::camera::Exposure::INDOOR,
                transform: Transform::from_translation(cam_pos).looking_at(rocket_pos + world_up * 4.0, world_up),
                ..default()
            },
            crate::OrbitCamera {
                radius: 18.0,
                theta: 30f32.to_radians(),
                phi: 95f32.to_radians(),
                target: rocket_pos + world_up * 5.5,
                exterior_offset: Vec3::new(12.0, 2.0, 8.0),
            },
            crate::InteriorCamera {
                yaw: std::f32::consts::PI,
                pitch: -0.05,
                position_offset: Vec3::new(0.0, 0.09, 0.06),
            },
            GameWorldEntity,
        ));
    }

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.65, 0.70, 0.80),
        brightness: 1.2,
    });

    let sun_dir = crate::sky::sun_direction();
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 120000.0,
                color: Color::srgb(1.0, 0.98, 0.95),
                shadows_enabled: true,
                shadow_depth_bias: 0.005,
                shadow_normal_bias: 0.05,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 15.0,
                maximum_distance: 150.0,
                ..default()
            }
            .build(),
            transform: Transform::from_translation(sun_dir * 2000.0)
                .with_rotation(Quat::from_rotation_arc(Vec3::NEG_Z, sun_dir)),
            ..default()
        },
        GameWorldEntity,
    ));

    use crate::systems::*;
    use crate::systems::csm::*;

    commands.spawn((
        CommandServiceModule {
            serial_number: "CSM-107".to_string(),
            mass_kg: 28801.0,
            electrical: ElectricalSystem::default(),
            sps: PropulsionSystem {
                engine: Engine {
                    name: "AJ10-137".to_string(),
                    thrust_sl_newtons: 0.0,
                    thrust_vac_newtons: 91000.0,
                    chamber_pressure_psi: 100.0,
                    mixture_ratio: 1.6,
                    status: SystemStatus::Nominal,
                },
                fuel_tanks: vec![PropellantTank::new("SPS Ox", 18413.0, 29300.0)],
                thrust_newtons: 0.0,
                isp_seconds: 314.0,
            },
            rcs: RcsSystem {
                quads: vec![
                    create_rcs_quad("A", true),
                    create_rcs_quad("B", true),
                    create_rcs_quad("C", true),
                    create_rcs_quad("D", true),
                ],
                propellant_kg: 118.0,
                pressure_psi: 270.0,
            },
            environmental_control: EnvironmentalControlSystem::default(),
            thermal: ThermalControlSystem::default(),
            gnc: GuidanceNavigationControl::default(),
            abort_mode_active: false,
            dse_powered: true,
            interior_lights_on: true,
            flood_lights_on: false,
            panel_lights_on: true,
            uv_lights_on: false,
            event_timer_powered: true,
            tape_recorder_active: false,
        },
        Name::new("CSM-107 Systems"),
        GameWorldEntity,
    ));

    let crew_names = [
        (if mission_config.commander_name.is_empty() { "Neil Armstrong".to_string() } else { mission_config.commander_name.clone() }, crate::crew::CrewRole::Commander),
        (if mission_config.cmp_name.is_empty() { "Michael Collins".to_string() } else { mission_config.cmp_name.clone() }, crate::crew::CrewRole::CommandModulePilot),
        (if mission_config.lmp_name.is_empty() { "Buzz Aldrin".to_string() } else { mission_config.lmp_name.clone() }, crate::crew::CrewRole::LunarModulePilot),
    ];

    for (name, role) in crew_names {
        let is_player = role == crate::crew::CrewRole::CommandModulePilot;
        let mut entity = commands.spawn((
            crate::crew::CrewMember {
                name,
                role,
                health: crate::crew::CrewHealth::default(),
            },
            GameWorldEntity,
        ));
        if is_player {
            entity.insert(crate::crew::PlayerCharacter);
        }
    }
}

fn create_rcs_quad(id: &str, enabled: bool) -> crate::systems::RcsQuad {
    crate::systems::RcsQuad {
        id: id.to_string(),
        thrusters: [
            crate::systems::RcsThruster { thrust_n: 445.0, status: crate::systems::SystemStatus::Nominal, fired_count: 0 },
            crate::systems::RcsThruster { thrust_n: 445.0, status: crate::systems::SystemStatus::Nominal, fired_count: 0 },
            crate::systems::RcsThruster { thrust_n: 445.0, status: crate::systems::SystemStatus::Nominal, fired_count: 0 },
            crate::systems::RcsThruster { thrust_n: 445.0, status: crate::systems::SystemStatus::Nominal, fired_count: 0 },
        ],
        enabled,
    }
}

fn transition_loading_to_ingame(
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_seconds();
    if *timer >= 0.5 {
        next_state.set(AppState::InGame);
        *timer = 0.0;
    }
}
