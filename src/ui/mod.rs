pub mod menu;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiSet};
use crate::mission::{MissionState, format_time, get_apollo11, get_apollo13, MissionActionEvent};
use crate::agc::AgcState;
use crate::spacecraft::Spacecraft;
use crate::communications::{CommunicationsBus, GroundControlState, CommMode, voice_loop_name, ground_station_name, DataRate};
use crate::planning::{MissionPlan, PlanStatus, validate_launch_plan, validate_flight_path, validate_return_plan};
use crate::npc::{NpcDialogueEvent, NpcCharacter, DialogueUrgency, PlayerRadioMessage};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .init_resource::<RadioMessageLog>()
            .add_systems(Update, top_menu_bar.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, telemetry_panel.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, mission_panel.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, radio_panel.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, planning_panel.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, log_panel.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, collect_radio_messages.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, camera_mode_keyboard.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct UiState {
    pub show_telemetry: bool,
    pub show_mission: bool,
    pub show_radio: bool,
    pub show_planning: bool,
    pub show_log: bool,
    pub selected_mission: String,
    pub radio_input: String,
}

#[derive(Resource)]
pub struct RadioMessageLog {
    pub messages: Vec<RadioMessage>,
    pub max_messages: usize,
}

pub struct RadioMessage {
    pub speaker: String,
    pub text: String,
    pub urgency: DialogueUrgency,
    pub mission_time: f64,
}

impl Default for RadioMessageLog {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            max_messages: 50,
        }
    }
}

fn collect_radio_messages(
    mut dialogue_events: EventReader<NpcDialogueEvent>,
    mut log: ResMut<RadioMessageLog>,
    mission_state: Res<MissionState>,
) {
    for event in dialogue_events.read() {
        let speaker = match &event.speaker {
            NpcCharacter::Capcom => "CAPCOM".to_string(),
            NpcCharacter::FlightDirector => "FLIGHT".to_string(),
            NpcCharacter::Fido => "FIDO".to_string(),
            NpcCharacter::Guido => "GUIDO".to_string(),
            NpcCharacter::Eecom => "EECOM".to_string(),
            NpcCharacter::Surgeon => "SURGEON".to_string(),
            NpcCharacter::CrewMember(role) => format!("{:?}", role),
        };
        log.messages.push(RadioMessage {
            speaker,
            text: event.message.clone(),
            urgency: event.urgency.clone(),
            mission_time: mission_state.mission_time,
        });
        if log.messages.len() > log.max_messages {
            log.messages.remove(0);
        }
    }
}

fn top_menu_bar(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut mission_state: ResMut<MissionState>,
    mut camera_mode: ResMut<crate::CameraMode>,
    mut time_scale: ResMut<crate::TimeScale>,
    mut next_state: ResMut<NextState<crate::game_state::AppState>>,
) {
    let time_str = format_time(mission_state.mission_time);
    let mission_id = mission_state.mission_id.clone();

    egui::TopBottomPanel::top("top_menu").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            let toggle = |ui: &mut egui::Ui, label: &str, active: &mut bool| {
                if ui.selectable_label(*active, label).clicked() {
                    *active = !*active;
                }
            };

            toggle(ui, "Telemetry", &mut ui_state.show_telemetry);
            toggle(ui, "Mission", &mut ui_state.show_mission);
            toggle(ui, "Radio", &mut ui_state.show_radio);
            toggle(ui, "Planning", &mut ui_state.show_planning);
            toggle(ui, "Log", &mut ui_state.show_log);

            ui.separator();

            egui::ComboBox::from_id_source("mission_select")
                .selected_text(&mission_state.mission_id)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(mission_state.mission_id == "apollo11", "Apollo 11").clicked() {
                        mission_state.mission_id = "apollo11".to_string();
                        mission_state.phase_index = 0;
                        mission_state.step_index = 0;
                        mission_state.mission_time = 0.0;
                        mission_state.log.clear();
                    }
                    if ui.selectable_label(mission_state.mission_id == "apollo13", "Apollo 13").clicked() {
                        mission_state.mission_id = "apollo13".to_string();
                        mission_state.phase_index = 0;
                        mission_state.step_index = 0;
                        mission_state.mission_time = 0.0;
                        mission_state.log.clear();
                    }
                });

            ui.separator();

            if mission_state.is_running {
                if ui.button("⏹ Stop Timer").clicked() {
                    mission_state.is_running = false;
                }
            } else {
                if ui.button("▶ Start Timer").clicked() {
                    mission_state.is_running = true;
                    mission_state.log.push(crate::mission::LogEntry {
                        time: time_str.clone(),
                        message: format!("Mission {} timer started", mission_id),
                    });
                }
            }

            if ui.button("⏸ Pause Game").clicked() {
                mission_state.is_running = false;
                next_state.set(crate::game_state::AppState::Paused);
            }

            ui.separator();

            if ui.button(format!("CAM: {}", camera_mode.name())).clicked() {
                *camera_mode = camera_mode.next();
            }

            ui.separator();

            ui.label(egui::RichText::new(&time_str).strong().color(egui::Color32::YELLOW));

            ui.separator();

            ui.label("Time:");
            if ui.button("◄").clicked() {
                time_scale.decrease();
            }
            ui.label(format!("{:.1}x", time_scale.multiplier));
            if ui.button("►").clicked() {
                time_scale.increase();
            }
            if ui.button("Reset").clicked() {
                time_scale.multiplier = 1.0;
            }
        });
    });
}

fn telemetry_panel(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    spacecraft_query: Query<&Spacecraft>,
) {
    if !ui_state.show_telemetry {
        return;
    }

    egui::Window::new("Telemetry")
        .default_pos([10.0, 240.0])
        .default_size([300.0, 200.0])
        .show(contexts.ctx_mut(), |ui| {
            for spacecraft in spacecraft_query.iter() {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.label(format!("{:?}", spacecraft.vessel_type));
                });
                ui.horizontal(|ui| {
                    ui.label("Velocity:");
                    ui.label(format!("{:.1} m/s", spacecraft.velocity.length()));
                });
                ui.horizontal(|ui| {
                    ui.label("Altitude:");
                    ui.label(format!("{:.1} km", spacecraft.altitude));
                });
            }
        });
}

fn camera_mode_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_mode: ResMut<crate::CameraMode>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        *camera_mode = camera_mode.next();
    }
}

fn mission_panel(
    mut contexts: EguiContexts,
    mut mission_state: ResMut<MissionState>,
    ui_state: Res<UiState>,
    mut action_events: EventWriter<MissionActionEvent>,
) {
    if !ui_state.show_mission {
        return;
    }

    let phases = if mission_state.mission_id == "apollo11" {
        get_apollo11()
    } else {
        get_apollo13()
    };

    egui::Window::new("Mission Control")
        .default_pos([10.0, 450.0])
        .default_size([400.0, 300.0])
        .show(contexts.ctx_mut(), |ui| {
            if let Some(phase) = phases.get(mission_state.phase_index) {
                ui.heading(&phase.name);
                ui.separator();

                for (i, proc) in phase.procedures.iter().enumerate() {
                    let is_active = i == mission_state.step_index;
                    let is_completed = i < mission_state.step_index;

                    let color = if is_active {
                        egui::Color32::YELLOW
                    } else if is_completed {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::GRAY
                    };

                    ui.horizontal(|ui| {
                        ui.colored_label(color, format!("{}.", proc.step));
                        ui.colored_label(color, &proc.text);
                        if is_active && ui.button("EXECUTE").clicked() {
                            action_events.send(MissionActionEvent {
                                action: proc.action.clone(),
                            });
                            mission_state.step_index += 1;
                            if mission_state.step_index >= phase.procedures.len() {
                                mission_state.step_index = 0;
                                mission_state.phase_index += 1;
                            }
                        }
                    });
                }
            } else {
                ui.heading("MISSION COMPLETE");
            }
        });
}


fn radio_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    comms: Res<CommunicationsBus>,
    ground: Res<GroundControlState>,
    radio_log: Res<RadioMessageLog>,
    mut radio_events: EventWriter<PlayerRadioMessage>,
) {
    if !ui_state.show_radio {
        return;
    }

    egui::Window::new("Radio Interface")
        .default_pos([340.0, 200.0])
        .default_size([350.0, 450.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Ground Control");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Station:");
                ui.label(ground_station_name(ground.station));
            });
            
            ui.horizontal(|ui| {
                ui.label("CAPCOM:");
                ui.label(&ground.capcom);
            });
            
            ui.horizontal(|ui| {
                ui.label("Flight Director:");
                ui.label(&ground.flight_director);
            });
            
            ui.separator();
            ui.heading("Signal");
            
            ui.horizontal(|ui| {
                ui.label("Strength:");
                let strength_color = if comms.signal_strength > 0.7 {
                    egui::Color32::GREEN
                } else if comms.signal_strength > 0.3 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };
                ui.colored_label(strength_color, format!("{:.1}%", comms.signal_strength * 100.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("SNR:");
                ui.label(format!("{:.1} dB", comms.snr_db));
            });
            
            ui.horizontal(|ui| {
                ui.label("Frequency:");
                ui.label(format!("{:.1} MHz", comms.frequency_mhz));
            });
            
            ui.separator();
            ui.heading("Data");
            
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.label(format!("{:?}", comms.mode));
            });
            
            ui.horizontal(|ui| {
                ui.label("Downlink:");
                ui.label(format!("{:.1} bps", comms.downlink_rate.bits_per_second()));
            });
            
            ui.horizontal(|ui| {
                ui.label("Uplink:");
                ui.label(format!("{:.1} bps", comms.uplink_rate.bits_per_second()));
            });
            
            ui.horizontal(|ui| {
                ui.label("BER:");
                ui.label(format!("{:.2e}", comms.bit_error_rate));
            });
            
            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Voice Loop");
                ui.label(voice_loop_name(ground.active_loop));
            });

            ui.separator();
            ui.heading("Houston Radio");

            egui::ScrollArea::vertical()
                .max_height(180.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for msg in &radio_log.messages {
                        let time_str = format_time(msg.mission_time);
                        let color = match &msg.urgency {
                            DialogueUrgency::Routine => egui::Color32::from_rgb(180, 200, 220),
                            DialogueUrgency::Advisory => egui::Color32::from_rgb(220, 220, 140),
                            DialogueUrgency::Urgent => egui::Color32::from_rgb(255, 180, 80),
                            DialogueUrgency::Emergency => egui::Color32::from_rgb(255, 80, 80),
                        };
                        ui.horizontal(|ui| {
                            ui.colored_label(egui::Color32::GRAY, format!("[{}]", time_str));
                            ui.colored_label(
                                egui::Color32::from_rgb(140, 180, 255),
                                format!("{}:", msg.speaker),
                            );
                            ui.colored_label(color, &msg.text);
                        });
                    }
                });

            ui.separator();
            let response = ui.horizontal(|ui| {
                ui.label("TX:");
                let text_edit = egui::TextEdit::singleline(&mut ui_state.radio_input)
                    .desired_width(ui.available_width() - 60.0)
                    .hint_text("Type message to Houston...");
                ui.add(text_edit)
            });

            if ui.button("SEND").clicked() || (response.inner.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                if !ui_state.radio_input.is_empty() {
                    radio_events.send(PlayerRadioMessage {
                        text: ui_state.radio_input.clone(),
                    });
                    ui_state.radio_input.clear();
                }
            }
        });
}

fn planning_panel(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    plan: Res<MissionPlan>,
) {
    if !ui_state.show_planning {
        return;
    }

    egui::Window::new("Mission Planning")
        .default_pos([660.0, 300.0])
        .default_size([400.0, 400.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Status: {}", plan.status.as_str()));
            ui.separator();
            
            ui.collapsing("Launch Plan", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Window Start:");
                    ui.label(&plan.launch.launch_window_start);
                });
                ui.horizontal(|ui| {
                    ui.label("Window End:");
                    ui.label(&plan.launch.launch_window_end);
                });
                ui.horizontal(|ui| {
                    ui.label("Azimuth:");
                    ui.label(format!("{:.1}°", plan.launch.launch_azimuth));
                });
                ui.horizontal(|ui| {
                    ui.label("Inclination:");
                    ui.label(format!("{:.1}°", plan.launch.target_inclination));
                });
                ui.horizontal(|ui| {
                    ui.label("Orbit Altitude:");
                    ui.label(format!("{:.0} km", plan.launch.target_orbit_altitude_km));
                });
                
                let issues = validate_launch_plan(&plan.launch);
                if issues.is_empty() {
                    ui.colored_label(egui::Color32::GREEN, "Launch plan valid");
                } else {
                    for issue in issues {
                        ui.colored_label(egui::Color32::RED, issue);
                    }
                }
            });
            
            ui.collapsing("Flight Path", |ui| {
                ui.horizontal(|ui| {
                    ui.label("TLI Time:");
                    ui.label(format!("T+{:.0} s", plan.flight_path.tli_burn_time_met));
                });
                ui.horizontal(|ui| {
                    ui.label("TLI Duration:");
                    ui.label(format!("{:.0} s", plan.flight_path.tli_duration_seconds));
                });
                ui.horizontal(|ui| {
                    ui.label("Landing Site:");
                    ui.label(&plan.flight_path.landing_site_name);
                });
                ui.horizontal(|ui| {
                    ui.label("Coordinates:");
                    ui.label(format!("{:.2}°, {:.2}°", 
                        plan.flight_path.landing_site_latitude,
                        plan.flight_path.landing_site_longitude));
                });
                
                let issues = validate_flight_path(&plan.flight_path);
                if issues.is_empty() {
                    ui.colored_label(egui::Color32::GREEN, "Flight path valid");
                } else {
                    for issue in issues {
                        ui.colored_label(egui::Color32::RED, issue);
                    }
                }
            });
            
            ui.collapsing("Return Plan", |ui| {
                ui.horizontal(|ui| {
                    ui.label("TEI Time:");
                    ui.label(format!("T+{:.0} s", plan.return_trajectory.tei_burn_time_met));
                });
                ui.horizontal(|ui| {
                    ui.label("Entry Interface:");
                    ui.label(format!("{:.1} km", plan.return_trajectory.entry_interface_altitude_km));
                });
                ui.horizontal(|ui| {
                    ui.label("Splashdown:");
                    ui.label(format!("{:.1}°, {:.1}°",
                        plan.return_trajectory.splashdown_latitude,
                        plan.return_trajectory.splashdown_longitude));
                });
                ui.horizontal(|ui| {
                    ui.label("Recovery:");
                    ui.label(&plan.return_trajectory.recovery_ship);
                });
                
                let issues = validate_return_plan(&plan.return_trajectory);
                if issues.is_empty() {
                    ui.colored_label(egui::Color32::GREEN, "Return plan valid");
                } else {
                    for issue in issues {
                        ui.colored_label(egui::Color32::RED, issue);
                    }
                }
            });
        });
}

fn log_panel(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    mission_state: Res<MissionState>,
) {
    if !ui_state.show_log {
        return;
    }

    egui::Window::new("Mission Log")
        .default_pos([660.0, 40.0])
        .default_size([400.0, 300.0])
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in mission_state.log.iter().rev() {
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::YELLOW, &entry.time);
                        ui.label(&entry.message);
                    });
                }
            });
        });
}
