use bevy::prelude::*;

#[cfg(feature = "llm-npcs")]
mod context;
#[cfg(feature = "llm-npcs")]
mod history;
#[cfg(feature = "llm-npcs")]
mod llm;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NpcDialogueEvent>()
            .add_event::<DialogueTrigger>()
            .add_event::<PlayerRadioMessage>()
            .init_resource::<RadioMessageQueue>()
            .add_systems(
                Update,
                npc_dialogue_system.run_if(in_state(crate::game_state::AppState::InGame)),
            )
            .add_systems(
                Update,
                listen_fault_events_for_dialogue
                    .run_if(in_state(crate::game_state::AppState::InGame)),
            )
            .add_systems(
                Update,
                listen_health_events_for_dialogue
                    .run_if(in_state(crate::game_state::AppState::InGame)),
            )
            .add_systems(
                Update,
                listen_mission_events_for_dialogue
                    .run_if(in_state(crate::game_state::AppState::InGame)),
            )
            .add_systems(
                Update,
                process_radio_queue.run_if(in_state(crate::game_state::AppState::InGame)),
            )
            .add_systems(
                Update,
                handle_player_radio_message.run_if(in_state(crate::game_state::AppState::InGame)),
            );

        #[cfg(feature = "llm-npcs")]
        {
            app.init_resource::<HoustonState>()
                .add_systems(
                    Update,
                    poll_llm_responses.run_if(in_state(crate::game_state::AppState::InGame)),
                );
        }
    }
}

// =============================================================================
// Shared types (always compiled)
// =============================================================================

#[derive(Event, Debug, Clone)]
pub struct NpcDialogueEvent {
    pub speaker: NpcCharacter,
    pub message: String,
    pub urgency: DialogueUrgency,
    pub channel: CommChannel,
}

#[derive(Event, Debug, Clone)]
pub struct DialogueTrigger {
    pub trigger_type: DialogueTriggerType,
}

#[derive(Debug, Clone)]
pub enum DialogueTriggerType {
    FaultTriggered(crate::faults::FaultId),
    FaultResolved(crate::faults::FaultId),
    HealthChanged(String, crate::crew::HealthStatus),
    MissionPhaseStarted(String),
    MissionPhaseCompleted(String),
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcCharacter {
    Capcom,
    FlightDirector,
    Fido,
    Guido,
    Eecom,
    Surgeon,
    CrewMember(crate::crew::CrewRole),
}

impl NpcCharacter {
    pub fn display_name(&self) -> String {
        match self {
            NpcCharacter::Capcom => "CAPCOM".to_string(),
            NpcCharacter::FlightDirector => "FLIGHT DIRECTOR".to_string(),
            NpcCharacter::Fido => "FIDO".to_string(),
            NpcCharacter::Guido => "GUIDO".to_string(),
            NpcCharacter::Eecom => "EECOM".to_string(),
            NpcCharacter::Surgeon => "SURGEON".to_string(),
            NpcCharacter::CrewMember(role) => format!("Crew - {:?}", role),
        }
    }

    pub fn mocr_tag(&self) -> &'static str {
        match self {
            NpcCharacter::Capcom => "CAPCOM",
            NpcCharacter::FlightDirector => "FLIGHT",
            NpcCharacter::Fido => "FIDO",
            NpcCharacter::Guido => "GUIDO",
            NpcCharacter::Eecom => "EECOM",
            NpcCharacter::Surgeon => "SURGEON",
            NpcCharacter::CrewMember(_) => "CREW",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueUrgency {
    Routine,
    Advisory,
    Urgent,
    Emergency,
}

impl DialogueUrgency {
    pub fn color(&self) -> Color {
        match self {
            DialogueUrgency::Routine => Color::srgb(0.7, 0.7, 0.7),
            DialogueUrgency::Advisory => Color::srgb(0.3, 0.7, 1.0),
            DialogueUrgency::Urgent => Color::srgb(1.0, 0.8, 0.0),
            DialogueUrgency::Emergency => Color::srgb(1.0, 0.2, 0.2),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommChannel {
    SBandVoice,
    VhfVoice,
    Teleprinter,
}

impl CommChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommChannel::SBandVoice => "S-Band Voice",
            CommChannel::VhfVoice => "VHF Voice",
            CommChannel::Teleprinter => "Teleprinter",
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct PlayerRadioMessage {
    pub text: String,
}

#[derive(Debug, Clone)]
struct DelayedMessage {
    text: String,
    deliver_at: f64,
}

#[derive(Resource, Default)]
pub struct RadioMessageQueue {
    outbound: Vec<DelayedMessage>,
    inbound: Vec<DelayedMessage>,
}

pub fn signal_delay_seconds(distance_to_earth_km: f32) -> f32 {
    distance_to_earth_km / 299_792.458
}

pub fn estimated_earth_distance_km(mission_phase_id: &str, mission_time_seconds: f64) -> f32 {
    match mission_phase_id {
        "prelaunch" | "launch" => 0.0,
        "orbit" => 400.0,
        "tli" => {
            let tli_duration = 3600.0 * 3.0;
            let fraction = (mission_time_seconds / tli_duration).min(1.0) as f32;
            fraction * 200_000.0
        }
        "coast" | "transposition" | "docking" => 200_000.0,
        "landing" | "lunar_orbit" | "descent" | "ascent" => 384_400.0,
        "reentry" => {
            let reentry_duration = 3600.0 * 3.0;
            let fraction = (mission_time_seconds / reentry_duration).min(1.0) as f32;
            384_400.0 * (1.0 - fraction)
        }
        _ => 100_000.0,
    }
}

// =============================================================================
// LLM-backed Houston (feature-gated)
// =============================================================================

#[cfg(feature = "llm-npcs")]
#[derive(Resource)]
pub struct HoustonState {
    config: llm::HoustonConfig,
    system_prompt: String,
    history: history::ConversationHistory,
    last_call_time: f64,
    pending: Option<bevy::tasks::Task<()>>,
    rx: Option<tokio::sync::oneshot::Receiver<Result<String, String>>>,
    pending_trigger: Option<DialogueTriggerType>,
}

#[cfg(feature = "llm-npcs")]
impl Default for HoustonState {
    fn default() -> Self {
        let config = llm::HoustonConfig::from_env();
        let system_prompt = llm::build_system_prompt();
        Self {
            config,
            system_prompt,
            history: history::ConversationHistory::new(),
            last_call_time: -999.0,
            pending: None,
            rx: None,
            pending_trigger: None,
        }
    }
}

#[cfg(feature = "llm-npcs")]
fn npc_dialogue_system(
    mut triggers: EventReader<DialogueTrigger>,
    mut dialogue_events: EventWriter<NpcDialogueEvent>,
    mut houston: ResMut<HoustonState>,
    crew_query: Query<&crate::crew::CrewMember>,
    csm_query: Query<&crate::systems::csm::CommandServiceModule>,
    mission_state: Res<crate::mission::MissionState>,
    fault_manager: Res<crate::faults::FaultManager>,
    comms: Res<crate::communications::CommunicationsBus>,
) {
    for trigger in triggers.read() {
        if comms.signal_strength < 0.05 {
            continue;
        }

        let urgency = trigger_urgency(&trigger.trigger_type);

        if houston.pending.is_some() {
            if let Some(fallback) = generate_scripted_dialogue(trigger) {
                dialogue_events.send(fallback);
            }
            continue;
        }

        let current_time = mission_state.mission_time;
        if current_time - houston.last_call_time < houston.config.min_call_interval_secs {
            if let Some(fallback) = generate_scripted_dialogue(trigger) {
                dialogue_events.send(fallback);
            }
            continue;
        }

        let situation = context::SituationReport::build(
            current_time,
            &format!("Phase {}", mission_state.phase_index),
            "Normal",
            comms.signal_strength,
            &fault_manager,
            &crew_query,
            &csm_query,
        );

        let context_text = situation.to_prompt_text();
        let trigger_desc = format!("{:?}", trigger.trigger_type);
        let full_context = format!("TRIGGER: {}\n\n{}", trigger_desc, context_text);

        let history_snapshot: Vec<history::HistoryEntry> =
            houston.history.recent(10).to_vec();
        let system_prompt = houston.system_prompt.clone();
        let config = houston.config.clone();

        houston.last_call_time = current_time;
        houston.pending_trigger = Some(trigger.trigger_type.clone());

        let (tx, rx) = tokio::sync::oneshot::channel();

        let handle = bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
            let client = llm::HoustonLlmClient::new(config);
            let result = match client.complete(&system_prompt, &full_context, &history_snapshot).await {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("LLM error: {}", e)),
            };
            let _ = tx.send(result);
        });

        houston.pending = Some(handle);
        houston.rx = Some(rx);

        if let Some(fallback) = generate_scripted_dialogue(trigger) {
            dialogue_events.send(fallback);
        }
    }
}

#[cfg(feature = "llm-npcs")]
fn poll_llm_responses(
    mut houston: ResMut<HoustonState>,
    mut dialogue_events: EventWriter<NpcDialogueEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if let Some(mut rx) = houston.rx.take() {
        match rx.try_recv() {
            Ok(Ok(response_text)) => {
                let (speaker, message) = parse_llm_response(&response_text);

                houston.history.push(history::HistoryEntry {
                    text: message.clone(),
                    from_ground: true,
                    mission_time: mission_state.mission_time,
                    speaker: speaker.display_name(),
                });

                let urgency = if let Some(ref trigger_type) = houston.pending_trigger {
                    trigger_urgency(trigger_type)
                } else {
                    DialogueUrgency::Advisory
                };

                dialogue_events.send(NpcDialogueEvent {
                    speaker,
                    message,
                    urgency,
                    channel: CommChannel::SBandVoice,
                });

                houston.pending_trigger = None;
                houston.pending = None;
            }
            Ok(Err(_)) => {
                houston.pending_trigger = None;
                houston.pending = None;
            }
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {
                houston.rx = Some(rx);
            }
            Err(_) => {
                houston.pending_trigger = None;
                houston.pending = None;
            }
        }
    }
}

#[cfg(feature = "llm-npcs")]
fn parse_llm_response(raw: &str) -> (NpcCharacter, String) {
    let trimmed = raw.trim();

    let (speaker, message) = if let Some(rest) = trimmed.strip_prefix("[CAPCOM]") {
        (NpcCharacter::Capcom, rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("[FLIGHT]") {
        (NpcCharacter::FlightDirector, rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("[EECOM]") {
        (NpcCharacter::Eecom, rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("[GUIDO]") {
        (NpcCharacter::Guido, rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("[FIDO]") {
        (NpcCharacter::Fido, rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("[SURGEON]") {
        (NpcCharacter::Surgeon, rest.trim())
    } else {
        (NpcCharacter::Capcom, trimmed)
    };

    let message = if message.len() > 500 {
        message.chars().take(500).collect()
    } else {
        message.to_string()
    };

    (speaker, message)
}

#[cfg(feature = "llm-npcs")]
fn trigger_urgency(trigger_type: &DialogueTriggerType) -> DialogueUrgency {
    match trigger_type {
        DialogueTriggerType::FaultTriggered(id) => match id.default_severity() {
            crate::faults::FaultSeverity::Catastrophic => DialogueUrgency::Emergency,
            crate::faults::FaultSeverity::Critical => DialogueUrgency::Emergency,
            crate::faults::FaultSeverity::Major => DialogueUrgency::Urgent,
            crate::faults::FaultSeverity::Minor => DialogueUrgency::Advisory,
        },
        DialogueTriggerType::FaultResolved(_) => DialogueUrgency::Routine,
        DialogueTriggerType::HealthChanged(_, status) => match status {
            crate::crew::HealthStatus::Critical | crate::crew::HealthStatus::Deceased => {
                DialogueUrgency::Emergency
            }
            crate::crew::HealthStatus::Impaired => DialogueUrgency::Urgent,
            crate::crew::HealthStatus::Stressed | crate::crew::HealthStatus::Fatigued => {
                DialogueUrgency::Advisory
            }
            _ => DialogueUrgency::Routine,
        },
        DialogueTriggerType::MissionPhaseStarted(_) | DialogueTriggerType::MissionPhaseCompleted(_) => {
            DialogueUrgency::Routine
        }
        DialogueTriggerType::Custom(_) => DialogueUrgency::Advisory,
    }
}

// =============================================================================
// Non-LLM path (always compiled)
// =============================================================================

#[cfg(not(feature = "llm-npcs"))]
fn npc_dialogue_system(
    mut triggers: EventReader<DialogueTrigger>,
    mut dialogue_events: EventWriter<NpcDialogueEvent>,
) {
    for trigger in triggers.read() {
        if let Some(dialogue) = generate_scripted_dialogue(trigger) {
            dialogue_events.send(dialogue);
        }
    }
}

// =============================================================================
// Scripted dialogue fallback (always compiled)
// =============================================================================

fn generate_scripted_dialogue(trigger: &DialogueTrigger) -> Option<NpcDialogueEvent> {
    match &trigger.trigger_type {
        DialogueTriggerType::FaultTriggered(fault_id) => match fault_id {
            crate::faults::FaultId::O2Tank2_Rupture => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Capcom,
                message: "Apollo, we've had a problem here.".to_string(),
                urgency: DialogueUrgency::Emergency,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::O2Tank1_Rupture => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Eecom,
                message: "O2 Tank 1 pressure is dropping fast. Check your cryo stir.".to_string(),
                urgency: DialogueUrgency::Emergency,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::CabinPressure_Loss => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Eecom,
                message: "Cabin pressure is dropping. Verify suit integrity and check for leaks.".to_string(),
                urgency: DialogueUrgency::Urgent,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::Co2Scrubber_Saturation => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Eecom,
                message: "CO2 levels are climbing. You need to replace that canister.".to_string(),
                urgency: DialogueUrgency::Urgent,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::MainBusA_Failure | crate::faults::FaultId::MainBusB_Failure => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Eecom,
                message: "We've lost a main bus. Check your fuel cell connections.".to_string(),
                urgency: DialogueUrgency::Urgent,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::FuelCell1_Failure | crate::faults::FaultId::FuelCell2_Failure | crate::faults::FaultId::FuelCell3_Failure => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Eecom,
                message: "Fuel cell failure confirmed. Power management is critical.".to_string(),
                urgency: DialogueUrgency::Urgent,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::Sps_Engine_Failure => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Fido,
                message: "SPS engine failure. We'll need to work alternate trajectories.".to_string(),
                urgency: DialogueUrgency::Emergency,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::Imu_GimbalLock => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Guido,
                message: "IMU is approaching gimbal lock. Recommend coarse alignment.".to_string(),
                urgency: DialogueUrgency::Advisory,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::Agc_ProgramAlarm => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Guido,
                message: "AGC program alarm detected. Stand by for analysis.".to_string(),
                urgency: DialogueUrgency::Advisory,
                channel: CommChannel::SBandVoice,
            }),
            crate::faults::FaultId::SBand_Amplifier_Failure | crate::faults::FaultId::HighGainAntenna_Stuck => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Capcom,
                message: "We're losing signal strength. Check your comm configuration.".to_string(),
                urgency: DialogueUrgency::Advisory,
                channel: CommChannel::SBandVoice,
            }),
            _ => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Capcom,
                message: format!("We've detected an anomaly: {}", fault_id.name()),
                urgency: DialogueUrgency::Advisory,
                channel: CommChannel::SBandVoice,
            }),
        },
        DialogueTriggerType::FaultResolved(fault_id) => Some(NpcDialogueEvent {
            speaker: NpcCharacter::Capcom,
            message: format!("Good work. {} is resolved.", fault_id.name()),
            urgency: DialogueUrgency::Routine,
            channel: CommChannel::SBandVoice,
        }),
        DialogueTriggerType::HealthChanged(crew_name, status) => match status {
            crate::crew::HealthStatus::Critical => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Surgeon,
                message: format!(
                    "{}'s biometrics are critical. Immediate medical attention required.",
                    crew_name
                ),
                urgency: DialogueUrgency::Emergency,
                channel: CommChannel::SBandVoice,
            }),
            crate::crew::HealthStatus::Impaired => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Surgeon,
                message: format!("{} is showing signs of impairment. Monitor closely.", crew_name),
                urgency: DialogueUrgency::Urgent,
                channel: CommChannel::SBandVoice,
            }),
            crate::crew::HealthStatus::Stressed => Some(NpcDialogueEvent {
                speaker: NpcCharacter::Surgeon,
                message: format!(
                    "{}'s heart rate is elevated. Consider workload reduction.",
                    crew_name
                ),
                urgency: DialogueUrgency::Advisory,
                channel: CommChannel::SBandVoice,
            }),
            crate::crew::HealthStatus::Deceased => Some(NpcDialogueEvent {
                speaker: NpcCharacter::FlightDirector,
                message: format!("This is Flight. We have lost {}. Godspeed.", crew_name),
                urgency: DialogueUrgency::Emergency,
                channel: CommChannel::SBandVoice,
            }),
            _ => None,
        },
        DialogueTriggerType::MissionPhaseStarted(phase) => Some(NpcDialogueEvent {
            speaker: NpcCharacter::Capcom,
            message: format!("Proceeding to {}. Good luck.", phase),
            urgency: DialogueUrgency::Routine,
            channel: CommChannel::SBandVoice,
        }),
        DialogueTriggerType::MissionPhaseCompleted(phase) => Some(NpcDialogueEvent {
            speaker: NpcCharacter::Capcom,
            message: format!("{} complete. Stand by for next phase.", phase),
            urgency: DialogueUrgency::Routine,
            channel: CommChannel::SBandVoice,
        }),
        DialogueTriggerType::Custom(msg) => Some(NpcDialogueEvent {
            speaker: NpcCharacter::Capcom,
            message: msg.clone(),
            urgency: DialogueUrgency::Routine,
            channel: CommChannel::SBandVoice,
        }),
    }
}

// =============================================================================
// Player radio input + signal delay (always compiled)
// =============================================================================

fn handle_player_radio_message(
    mut radio_events: EventReader<PlayerRadioMessage>,
    mut queue: ResMut<RadioMessageQueue>,
    mission_state: Res<crate::mission::MissionState>,
    #[cfg(feature = "llm-npcs")] mut houston: ResMut<HoustonState>,
    #[cfg(feature = "llm-npcs")] crew_query: Query<&crate::crew::CrewMember>,
    #[cfg(feature = "llm-npcs")] csm_query: Query<&crate::systems::csm::CommandServiceModule>,
    #[cfg(feature = "llm-npcs")] fault_manager: Res<crate::faults::FaultManager>,
    #[cfg(feature = "llm-npcs")] comms: Res<crate::communications::CommunicationsBus>,
    #[cfg(feature = "llm-npcs")] mut dialogue_triggers: EventWriter<DialogueTrigger>,
) {
    for msg in radio_events.read() {
        #[cfg(feature = "llm-npcs")]
        {
            if comms.signal_strength < 0.05 {
                continue;
            }

            houston.history.push(history::HistoryEntry {
                text: msg.text.clone(),
                from_ground: false,
                mission_time: mission_state.mission_time,
                speaker: "Crew".to_string(),
            });

            if houston.pending.is_some() {
                continue;
            }

            let current_time = mission_state.mission_time;
            if current_time - houston.last_call_time < houston.config.min_call_interval_secs {
                continue;
            }

            let situation = context::SituationReport::build(
                current_time,
                &format!("Phase {}", mission_state.phase_index),
                "Normal",
                comms.signal_strength,
                &fault_manager,
                &crew_query,
                &csm_query,
            );

            let context_text = situation.to_prompt_text();
            let full_context = format!(
                "CREW MESSAGE: {}\n\n{}",
                msg.text, context_text
            );

            let history_snapshot: Vec<history::HistoryEntry> =
                houston.history.recent(10).to_vec();
            let system_prompt = houston.system_prompt.clone();
            let config = houston.config.clone();

            houston.last_call_time = current_time;
            houston.pending_trigger =
                Some(DialogueTriggerType::Custom(msg.text.clone()));

            let (tx, rx) = tokio::sync::oneshot::channel();

            let handle = bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
                let client = llm::HoustonLlmClient::new(config);
                let result = match client
                    .complete(&system_prompt, &full_context, &history_snapshot)
                    .await
                {
                    Ok(response) => Ok(response),
                    Err(e) => Err(format!("LLM error: {}", e)),
                };
                let _ = tx.send(result);
            });

            houston.pending = Some(handle);
            houston.rx = Some(rx);
        }

        #[cfg(not(feature = "llm-npcs"))]
        {
            let _ = &msg;
            let _ = &mut queue;
        }
    }
}

fn process_radio_queue(
    time: Res<Time>,
    mut queue: ResMut<RadioMessageQueue>,
    mut dialogue_events: EventWriter<NpcDialogueEvent>,
) {
    let now = time.elapsed_seconds_f64();

    queue.outbound.retain(|msg| {
        if now >= msg.deliver_at {
            dialogue_events.send(NpcDialogueEvent {
                speaker: NpcCharacter::CrewMember(crate::crew::CrewRole::Commander),
                message: format!("→ Houston: {}", msg.text),
                urgency: DialogueUrgency::Routine,
                channel: CommChannel::SBandVoice,
            });
            false
        } else {
            true
        }
    });

    queue.inbound.retain(|msg| {
        if now >= msg.deliver_at {
            true
        } else {
            true
        }
    });
}

// =============================================================================
// Event listeners (always compiled)
// =============================================================================

fn listen_fault_events_for_dialogue(
    mut fault_events: EventReader<crate::faults::FaultTriggeredEvent>,
    mut dialogue_triggers: EventWriter<DialogueTrigger>,
) {
    for event in fault_events.read() {
        dialogue_triggers.send(DialogueTrigger {
            trigger_type: DialogueTriggerType::FaultTriggered(event.fault_id),
        });
    }
}

fn listen_health_events_for_dialogue(
    crew_query: Query<(Entity, &crate::crew::CrewMember), Changed<crate::crew::CrewMember>>,
    mut dialogue_triggers: EventWriter<DialogueTrigger>,
) {
    for (_entity, crew) in crew_query.iter() {
        dialogue_triggers.send(DialogueTrigger {
            trigger_type: DialogueTriggerType::HealthChanged(
                crew.name.clone(),
                crew.health.status,
            ),
        });
    }
}

fn listen_mission_events_for_dialogue(
    mut action_events: EventReader<crate::mission::MissionActionEvent>,
    mut dialogue_triggers: EventWriter<DialogueTrigger>,
) {
    for event in action_events.read() {
        dialogue_triggers.send(DialogueTrigger {
            trigger_type: DialogueTriggerType::Custom(format!("Mission action: {}", event.action)),
        });
    }
}
