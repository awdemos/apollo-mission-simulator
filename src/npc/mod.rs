use bevy::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NpcDialogueEvent>()
            .add_event::<DialogueTrigger>()
            .add_systems(Update, npc_dialogue_system.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, listen_fault_events_for_dialogue.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, listen_health_events_for_dialogue.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, listen_mission_events_for_dialogue.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

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

fn npc_dialogue_system(
    mut triggers: EventReader<DialogueTrigger>,
    mut dialogue_events: EventWriter<NpcDialogueEvent>,
    crew_query: Query<&crate::crew::CrewMember>,
    csm_query: Query<&crate::systems::csm::CommandServiceModule>,
    mission_state: Res<crate::mission::MissionState>,
    fault_manager: Res<crate::faults::FaultManager>,
) {
    for trigger in triggers.read() {
        #[cfg(feature = "llm-npcs")]
        {
            if let Some(dialogue) = generate_llm_dialogue(
                trigger,
                &crew_query,
                &csm_query,
                &mission_state,
                &fault_manager,
            ) {
                dialogue_events.send(dialogue);
                continue;
            }
        }

        if let Some(dialogue) = generate_scripted_dialogue(trigger) {
            dialogue_events.send(dialogue);
        }
    }
}

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
                message: format!("{}'s biometrics are critical. Immediate medical attention required.", crew_name),
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
                message: format!("{}'s heart rate is elevated. Consider workload reduction.", crew_name),
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

#[cfg(feature = "llm-npcs")]
fn generate_llm_dialogue(
    trigger: &DialogueTrigger,
    _crew_query: &Query<&crate::crew::CrewMember>,
    _csm_query: &Query<&crate::systems::csm::CommandServiceModule>,
    _mission_state: &crate::mission::MissionState,
    _fault_manager: &crate::faults::FaultManager,
) -> Option<NpcDialogueEvent> {
    let _context = format!("{:?}", trigger.trigger_type);

    Some(NpcDialogueEvent {
        speaker: NpcCharacter::Capcom,
        message: "[LLM-NPC] Placeholder response - integrate LLM API here".to_string(),
        urgency: DialogueUrgency::Advisory,
        channel: CommChannel::SBandVoice,
    })
}

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
