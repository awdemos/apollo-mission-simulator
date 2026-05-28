use bevy::prelude::*;

pub struct LvdcPlugin;

impl Plugin for LvdcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LvdcState>()
            .add_event::<ManualStagingEvent>()
            .add_systems(Update, update_lvdc_guidance.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, execute_staging_sequence.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, process_manual_staging.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, check_staging_readiness.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct LvdcState {
    pub computers: [LvdcComputer; 3],
    pub active_computer: usize,
    pub mission_time: f64,
    pub guidance_mode: GuidanceMode,
    pub staging_commands: Vec<StagingCommand>,
    pub current_stage: u32,
    pub attitude: Quat,
    pub angular_velocity: Vec3,
    pub target_inclination: f32,
    pub target_altitude: f32,
}

impl Default for LvdcState {
    fn default() -> Self {
        Self {
            computers: [
                LvdcComputer::new("LVDC-A"),
                LvdcComputer::new("LVDC-B"),
                LvdcComputer::new("LVDC-C"),
            ],
            active_computer: 0,
            mission_time: 0.0,
            guidance_mode: GuidanceMode::PreLaunch,
            staging_commands: vec![
                StagingCommand::new(168.0, 1, StagingAction::CutoffAndSeparate),
                StagingCommand::new(170.0, 2, StagingAction::Ignite),
                StagingCommand::new(480.0, 2, StagingAction::CutoffAndSeparate),
                StagingCommand::new(482.0, 3, StagingAction::Ignite),
                StagingCommand::new(680.0, 3, StagingAction::Cutoff),
            ],
            current_stage: 0,
            attitude: Quat::IDENTITY,
            angular_velocity: Vec3::ZERO,
            target_inclination: 32.5f32.to_radians(),
            target_altitude: 185.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LvdcComputer {
    pub name: String,
    pub status: ComputerStatus,
    pub memory_words: [u16; 5120],
    pub cycle_time_us: u32,
    pub last_vote: bool,
}

impl LvdcComputer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: ComputerStatus::Standby,
            memory_words: [0u16; 5120],
            cycle_time_us: 715,
            last_vote: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputerStatus {
    Standby,
    Active,
    Failed,
    Voting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidanceMode {
    PreLaunch,
    FirstStage,
    SecondStage,
    ThirdStage,
    OrbitInsertion,
    Coast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingAction {
    CutoffAndSeparate,
    Ignite,
    Cutoff,
}

#[derive(Event, Debug, Clone)]
pub struct ManualStagingEvent {
    pub stage: u32,
    pub action: StagingAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingReadiness {
    NotReady,
    Ready,
    Executed,
}

#[derive(Debug, Clone)]
pub struct StagingCommand {
    pub time: f64,
    pub stage: u32,
    pub action: StagingAction,
    pub readiness: StagingReadiness,
    pub manual_confirmation_required: bool,
}

impl StagingCommand {
    pub fn new(time: f64, stage: u32, action: StagingAction) -> Self {
        Self {
            time,
            stage,
            action,
            readiness: StagingReadiness::NotReady,
            manual_confirmation_required: true,
        }
    }
}

fn update_lvdc_guidance(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut lvdc: ResMut<LvdcState>,
    mut spacecraft_query: Query<(&mut crate::spacecraft::Spacecraft, &mut crate::spacecraft::LaunchController)>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;
    lvdc.mission_time += dt as f64;

    for computer in &mut lvdc.computers {
        if computer.status != ComputerStatus::Failed {
            computer.status = ComputerStatus::Active;
        }
    }

    let votes: Vec<bool> = lvdc.computers.iter()
        .filter(|c| c.status != ComputerStatus::Failed)
        .map(|c| c.last_vote)
        .collect();

    let consensus = if votes.len() >= 2 {
        votes.iter().filter(|&&v| v).count() >= votes.len() / 2
    } else {
        false
    };

    if let Ok((mut spacecraft, mut launch)) = spacecraft_query.get_single_mut() {
        match launch.state {
            crate::spacecraft::LaunchState::OnPad => {
            }
            crate::spacecraft::LaunchState::Ignition | crate::spacecraft::LaunchState::Liftoff => {
                lvdc.guidance_mode = GuidanceMode::FirstStage;
            }
            crate::spacecraft::LaunchState::InFlight => {
                if lvdc.current_stage == 0 {
                    lvdc.guidance_mode = GuidanceMode::FirstStage;
                } else if lvdc.current_stage == 1 {
                    lvdc.guidance_mode = GuidanceMode::SecondStage;
                } else if lvdc.current_stage == 2 {
                    lvdc.guidance_mode = GuidanceMode::ThirdStage;
                }
            }
            _ => {}
        }

        if consensus && launch.state == crate::spacecraft::LaunchState::InFlight {
            let pitch_rate = match lvdc.guidance_mode {
                GuidanceMode::FirstStage => 0.3 * dt,
                GuidanceMode::SecondStage => 0.15 * dt,
                GuidanceMode::ThirdStage => 0.05 * dt,
                _ => 0.0,
            };
            lvdc.angular_velocity.x = pitch_rate;
        }
    }
}

fn execute_staging_sequence(
    mut lvdc: ResMut<LvdcState>,
    mut spacecraft_query: Query<(&mut crate::spacecraft::LaunchController, &GlobalTransform)>,
    mut separate_events: EventWriter<crate::staging_animation::StageSeparateEvent>,
    mut ignite_events: EventWriter<crate::staging_animation::EngineIgnitionEvent>,
) {
    if let Ok((mut launch, transform)) = spacecraft_query.get_single_mut() {
        let current_stage = lvdc.current_stage;
        let mut new_stage = current_stage;
        let mut new_guidance_mode = lvdc.guidance_mode;
        let rocket_pos = transform.translation();
        let rocket_vel = Vec3::ZERO;
        
        for cmd in &mut lvdc.staging_commands {
            if cmd.readiness == StagingReadiness::Ready && current_stage < cmd.stage {
                match cmd.action {
                    StagingAction::CutoffAndSeparate => {
                        new_stage = cmd.stage;
                        cmd.readiness = StagingReadiness::Executed;
                        launch.stage = match cmd.stage {
                            1 => crate::spacecraft::LaunchStage::SII_Burn,
                            2 => crate::spacecraft::LaunchStage::SIVB_Burn1,
                            _ => launch.stage,
                        };
                        let stage_id = match cmd.stage {
                            1 => crate::staging_animation::StageIdentifier::SIC,
                            2 => crate::staging_animation::StageIdentifier::SII,
                            _ => crate::staging_animation::StageIdentifier::SIVB,
                        };
                        separate_events.send(crate::staging_animation::StageSeparateEvent {
                            stage: stage_id,
                            separation_point: rocket_pos,
                            rocket_velocity: rocket_vel,
                        });
                        info!("STAGING: Stage {} cutoff and separation executed", cmd.stage);
                    }
                    StagingAction::Ignite => {
                        cmd.readiness = StagingReadiness::Executed;
                        launch.state = crate::spacecraft::LaunchState::InFlight;
                        let (radius, count) = match cmd.stage {
                            1 => (1.83 * crate::config::SATURN_V_SCALE, 5u32),
                            2 => (1.0 * crate::config::SATURN_V_SCALE, 5u32),
                            _ => (1.0 * crate::config::SATURN_V_SCALE, 1u32),
                        };
                        ignite_events.send(crate::staging_animation::EngineIgnitionEvent {
                            position: rocket_pos,
                            engine_radius: radius,
                            engine_count: count,
                        });
                        info!("STAGING: Stage {} ignition", cmd.stage);
                    }
                    StagingAction::Cutoff => {
                        cmd.readiness = StagingReadiness::Executed;
                        launch.state = crate::spacecraft::LaunchState::OrbitInsertion;
                        new_guidance_mode = GuidanceMode::OrbitInsertion;
                        info!("STAGING: Stage {} cutoff - orbit insertion", cmd.stage);
                    }
                }
            }
        }
        
        lvdc.current_stage = new_stage;
        lvdc.guidance_mode = new_guidance_mode;
    }
}

fn process_manual_staging(
    mut staging_events: EventReader<ManualStagingEvent>,
    mut lvdc: ResMut<LvdcState>,
) {
    for event in staging_events.read() {
        let mission_time = lvdc.mission_time;
        for cmd in &mut lvdc.staging_commands {
            if cmd.stage == event.stage && cmd.action == event.action && cmd.readiness == StagingReadiness::NotReady {
                if !cmd.manual_confirmation_required || mission_time >= cmd.time {
                    cmd.readiness = StagingReadiness::Ready;
                    info!("STAGING COMMAND: Stage {} {:?} confirmed by operator", cmd.stage, cmd.action);
                } else {
                    info!("STAGING COMMAND: Stage {} {:?} premature - waiting for T+{}", cmd.stage, cmd.action, cmd.time);
                }
            }
        }
    }
}

fn check_staging_readiness(
    mut lvdc: ResMut<LvdcState>,
) {
    let mission_time = lvdc.mission_time;
    for cmd in &mut lvdc.staging_commands {
        if cmd.readiness == StagingReadiness::NotReady && !cmd.manual_confirmation_required && mission_time >= cmd.time {
            cmd.readiness = StagingReadiness::Ready;
        }
    }
}
