use bevy::prelude::*;

pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MissionState>()
            .add_event::<MissionActionEvent>()
            .add_systems(Update, update_mission_timer.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, handle_mission_actions.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct MissionState {
    pub mission_id: String,
    pub phase_index: usize,
    pub step_index: usize,
    pub mission_time: f64,
    pub is_running: bool,
    pub log: Vec<LogEntry>,
}

#[derive(Clone)]
pub struct LogEntry {
    pub time: String,
    pub message: String,
}

#[derive(Event)]
pub struct MissionActionEvent {
    pub action: String,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            mission_id: "apollo11".to_string(),
            phase_index: 0,
            step_index: 0,
            mission_time: 0.0,
            is_running: false,
            log: vec![],
        }
    }
}

#[derive(Clone)]
pub struct MissionPhase {
    pub id: String,
    pub name: String,
    pub procedures: Vec<Procedure>,
}

#[derive(Clone)]
pub struct Procedure {
    pub step: usize,
    pub text: String,
    pub action: String,
}

pub fn get_apollo11() -> Vec<MissionPhase> {
    vec![
        MissionPhase {
            id: "prelaunch".to_string(),
            name: "PRE-LAUNCH".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Verify IMU alignment".to_string(), action: "align-imu".to_string() },
                Procedure { step: 2, text: "Enter VERB 37 to select program".to_string(), action: "select-program".to_string() },
                Procedure { step: 3, text: "Load launch azimuth 072.000".to_string(), action: "load-azimuth".to_string() },
                Procedure { step: 4, text: "Confirm all systems GO".to_string(), action: "confirm-go".to_string() },
                Procedure { step: 5, text: "Initiate countdown T-10 minutes".to_string(), action: "start-countdown".to_string() },
            ],
        },
        MissionPhase {
            id: "launch".to_string(),
            name: "LAUNCH".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "T-0: Saturn V ignition".to_string(), action: "ignition".to_string() },
                Procedure { step: 2, text: "S-IC burnout at T+168s".to_string(), action: "stage1-sep".to_string() },
                Procedure { step: 3, text: "S-II ignition".to_string(), action: "stage2-ignite".to_string() },
                Procedure { step: 4, text: "S-II burnout at T+480s".to_string(), action: "stage2-sep".to_string() },
                Procedure { step: 5, text: "S-IVB ignition for orbit".to_string(), action: "stage3-ignite".to_string() },
                Procedure { step: 6, text: "Orbit insertion confirmed".to_string(), action: "orbit-insertion".to_string() },
            ],
        },
        MissionPhase {
            id: "tli".to_string(),
            name: "TRANS-LUNAR INJECTION".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "S-IVB TLI burn initiation".to_string(), action: "tli-start".to_string() },
                Procedure { step: 2, text: "Monitor burn progress".to_string(), action: "monitor-burn".to_string() },
                Procedure { step: 3, text: "TLI cutoff - 10834 m/s".to_string(), action: "tli-cutoff".to_string() },
                Procedure { step: 4, text: "Transposition and docking".to_string(), action: "transposition".to_string() },
            ],
        },
        MissionPhase {
            id: "landing".to_string(),
            name: "LUNAR DESCENT".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Powered descent initiation P63".to_string(), action: "pdi".to_string() },
                Procedure { step: 2, text: "Braking phase".to_string(), action: "braking".to_string() },
                Procedure { step: 3, text: "Approach phase - manual".to_string(), action: "approach".to_string() },
                Procedure { step: 4, text: "Terminal descent".to_string(), action: "terminal".to_string() },
                Procedure { step: 5, text: "LUNAR CONTACT! Engine cutoff.".to_string(), action: "touchdown".to_string() },
                Procedure { step: 6, text: "Stay/No-Stay decision".to_string(), action: "stay-decision".to_string() },
            ],
        },
        MissionPhase {
            id: "reentry".to_string(),
            name: "RE-ENTRY & RECOVERY".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Service module separation".to_string(), action: "sm-sep".to_string() },
                Procedure { step: 2, text: "CM orient for entry".to_string(), action: "orient-entry".to_string() },
                Procedure { step: 3, text: "Entry interface 121.9km".to_string(), action: "entry-interface".to_string() },
                Procedure { step: 4, text: "Max G-load 6.2 Gs".to_string(), action: "max-g".to_string() },
                Procedure { step: 5, text: "Drogue chute at 10.7km".to_string(), action: "drogue".to_string() },
                Procedure { step: 6, text: "Main chute at 3km".to_string(), action: "main-chute".to_string() },
                Procedure { step: 7, text: "Splashdown!".to_string(), action: "splashdown".to_string() },
            ],
        },
    ]
}

pub fn get_apollo13() -> Vec<MissionPhase> {
    vec![
        MissionPhase {
            id: "prelaunch".to_string(),
            name: "PRE-LAUNCH".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Standard pre-launch checkout".to_string(), action: "prelaunch".to_string() },
            ],
        },
        MissionPhase {
            id: "launch".to_string(),
            name: "LAUNCH".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Normal Saturn V launch".to_string(), action: "launch".to_string() },
            ],
        },
        MissionPhase {
            id: "crisis".to_string(),
            name: "CRISIS!".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Houston, we have a problem!".to_string(), action: "explosion".to_string() },
                Procedure { step: 2, text: "O2 tank 2 exploded".to_string(), action: "main-bus-b".to_string() },
                Procedure { step: 3, text: "Shutdown CM systems".to_string(), action: "power-down".to_string() },
                Procedure { step: 4, text: "Use LM as lifeboat".to_string(), action: "lm-lifeboat".to_string() },
                Procedure { step: 5, text: "Free return trajectory".to_string(), action: "free-return".to_string() },
            ],
        },
        MissionPhase {
            id: "reentry".to_string(),
            name: "CRITICAL RETURN".to_string(),
            procedures: vec![
                Procedure { step: 1, text: "Power up CM from frozen".to_string(), action: "power-up".to_string() },
                Procedure { step: 2, text: "Manual alignment".to_string(), action: "manual-align".to_string() },
                Procedure { step: 3, text: "LM jettison".to_string(), action: "lm-jettison".to_string() },
                Procedure { step: 4, text: "Service module jettison".to_string(), action: "sm-jettison".to_string() },
                Procedure { step: 5, text: "Re-entry and splashdown".to_string(), action: "splashdown".to_string() },
            ],
        },
    ]
}

fn update_mission_timer(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut state: ResMut<MissionState>,
) {
    if state.is_running {
        state.mission_time += time.delta_seconds_f64() * time_scale.multiplier as f64;
    }
}

fn handle_mission_actions(
    mut events: EventReader<MissionActionEvent>,
    mut state: ResMut<MissionState>,
    mut spacecraft_query: Query<&mut crate::spacecraft::Spacecraft>,
    mut agc: Option<ResMut<crate::agc::AgcState>>,
    mut csm_query: Query<&mut crate::systems::csm::CommandServiceModule>,
    mut saturn_query: Query<&mut crate::systems::saturn_v::SaturnVSystems>,
) {
    for event in events.read() {
        let time_str = format_time(state.mission_time);
        
        match event.action.as_str() {
            "align-imu" => {
                state.log.push(LogEntry {
                    time: time_str.clone(),
                    message: "IMU aligned to launch azimuth 072°".to_string(),
                });
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Gimbal angles: X+0.00°, Y+0.00°, Z+0.00°".to_string(),
                });
            }
            "select-program" => {
                if let Some(ref mut agc_state) = agc {
                    agc_state.program = 1;
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Program 01 (Pre-Launch) selected".to_string(),
                });
            }
            "load-azimuth" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Launch azimuth 072.000° loaded".to_string(),
                });
            }
            "confirm-go" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "All systems GO for launch".to_string(),
                });
            }
            "start-countdown" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "T-10 minutes and counting".to_string(),
                });
            }
            "ignition" => {
                for mut saturn in saturn_query.iter_mut() {
                    crate::systems::saturn_v::ignite_s_ic(&mut saturn);
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "IGNITION! Saturn V engines at 100% thrust".to_string(),
                });
            }
            "stage1-sep" => {
                for mut saturn in saturn_query.iter_mut() {
                    crate::systems::saturn_v::stage_separation(&mut saturn, 1);
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "S-IC separation confirmed".to_string(),
                });
            }
            "stage2-ignite" => {
                for mut saturn in saturn_query.iter_mut() {
                    saturn.s_ii.status = crate::systems::SystemStatus::Nominal;
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "S-II ignition confirmed".to_string(),
                });
            }
            "stage2-sep" => {
                for mut saturn in saturn_query.iter_mut() {
                    crate::systems::saturn_v::stage_separation(&mut saturn, 2);
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "S-II separation confirmed".to_string(),
                });
            }
            "stage3-ignite" => {
                for mut saturn in saturn_query.iter_mut() {
                    saturn.s_ivb.status = crate::systems::SystemStatus::Nominal;
                }
                state.log.push(LogEntry {
                    time: time_str,
                    message: "S-IVB ignition for orbit insertion".to_string(),
                });
            }
            "orbit-insertion" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Orbit insertion confirmed - 185x185 km".to_string(),
                });
            }
            "tli-start" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "TLI burn initiated - 5:47 burn time".to_string(),
                });
            }
            "tli-cutoff" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "TLI cutoff - 10,834 m/s velocity".to_string(),
                });
            }
            "transposition" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "CSM/LM transposition and docking complete".to_string(),
                });
            }
            "pdi" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Powered descent initiation - P63".to_string(),
                });
            }
            "touchdown" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Houston, Tranquility Base here. The Eagle has landed!".to_string(),
                });
            }
            "sm-sep" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Service module separation confirmed".to_string(),
                });
            }
            "splashdown" => {
                state.is_running = false;
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Splashdown confirmed! Recovery underway.".to_string(),
                });
            }
            "explosion" => {
                for mut csm in csm_query.iter_mut() {
                    crate::systems::csm::simulate_o2_tank_explosion(&mut csm);
                }
                state.log.push(LogEntry {
                    time: time_str.clone(),
                    message: "MAYDAY! O2 tank 2 has exploded!".to_string(),
                });
                state.log.push(LogEntry {
                    time: time_str,
                    message: "Main Bus B offline - Fuel Cell 1 failed".to_string(),
                });
            }
            "lm-lifeboat" => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: "LM configured as lifeboat - power conservation mode".to_string(),
                });
            }
            _ => {
                state.log.push(LogEntry {
                    time: time_str,
                    message: format!("Executing: {}", event.action),
                });
            }
        }
    }
}

pub fn format_time(seconds: f64) -> String {
    let hrs = (seconds / 3600.0) as u32;
    let mins = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    format!("T+{:02}:{:02}:{:02}", hrs, mins, secs)
}
