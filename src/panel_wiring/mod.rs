use bevy::prelude::*;
use crate::panels::{PanelInteraction, SwitchId, SwitchState, BreakerId};
use crate::systems::csm::CommandServiceModule;
use crate::communications::CommunicationsBus;

pub struct PanelWiringPlugin;

impl Plugin for PanelWiringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MasterAlarmState>()
            .add_systems(Update, handle_panel_system_wiring.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, handle_master_alarm.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

fn handle_panel_system_wiring(
    mut interaction_events: EventReader<PanelInteraction>,
    mut csm_query: Query<&mut CommandServiceModule>,
    mut comms_bus: ResMut<CommunicationsBus>,
) {
    for event in interaction_events.read() {
        match event {
            PanelInteraction::SwitchToggled(_, id, state) => {
                if let Ok(mut csm) = csm_query.get_single_mut() {
                    apply_switch_to_csm(&mut csm, id, state);
                }
                apply_switch_to_comms(&mut comms_bus, id, state);
            }
            PanelInteraction::BreakerToggled(_, id, closed) => {
                if let Ok(mut csm) = csm_query.get_single_mut() {
                    apply_breaker_to_csm(&mut csm, id, closed);
                }
            }
            _ => {}
        }
    }
}

fn apply_switch_to_csm(csm: &mut CommandServiceModule, id: &SwitchId, state: &SwitchState) {
    match id {
        SwitchId::RcsQuadA => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "A") {
                quad.enabled = *state == SwitchState::On;
            }
        }
        SwitchId::RcsQuadB => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "B") {
                quad.enabled = *state == SwitchState::On;
            }
        }
        SwitchId::RcsQuadC => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "C") {
                quad.enabled = *state == SwitchState::On;
            }
        }
        SwitchId::RcsQuadD => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "D") {
                quad.enabled = *state == SwitchState::On;
            }
        }
        SwitchId::MainBusA => {
            csm.electrical.main_bus_a = *state == SwitchState::On;
        }
        SwitchId::MainBusB => {
            csm.electrical.main_bus_b = *state == SwitchState::On;
        }
        SwitchId::O2Fan1 | SwitchId::O2Fan2 => {
            let on = *state == SwitchState::On || *state == SwitchState::Auto;
            csm.environmental_control.suit_circuit.compressor_active = on;
        }
        SwitchId::CabinFan => {
            csm.environmental_control.cabin_fan.enabled = *state == SwitchState::On;
        }
        SwitchId::CryoPumps => {
            let on = *state == SwitchState::On || *state == SwitchState::Auto;
            csm.environmental_control.oxygen_supply.cryo_pumps_active = on;
        }
        _ => {}
    }
}

fn apply_switch_to_comms(comms: &mut CommunicationsBus, id: &SwitchId, state: &SwitchState) {
    match id {
        SwitchId::SBandPower => {
            comms.mode = if *state == SwitchState::On {
                crate::communications::CommMode::TelemetryAndVoice
            } else {
                crate::communications::CommMode::Off
            };
        }
        _ => {}
    }
}

fn apply_breaker_to_csm(csm: &mut CommandServiceModule, id: &BreakerId, closed: &bool) {
    match id {
        BreakerId::FuelCell1MainBusA => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC1") {
                fc.status = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        BreakerId::FuelCell2MainBusA => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC2") {
                fc.status = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        BreakerId::FuelCell3MainBusB => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC3") {
                fc.status = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        BreakerId::Inverter1 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV1") {
                inv.status = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        BreakerId::Inverter2 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV2") {
                inv.status = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        _ => {}
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct MasterAlarmState {
    pub active: bool,
    pub triggered_by: Option<String>,
    pub acknowledged: bool,
}

fn handle_master_alarm(
    master_alarm: Res<crate::game_state::MasterAlarm>,
    mut alarm_state: ResMut<MasterAlarmState>,
    mut mission_state: ResMut<crate::mission::MissionState>,
) {
    if master_alarm.is_changed() {
        if master_alarm.active && !alarm_state.active {
            let time_str = crate::mission::format_time(mission_state.mission_time);
            let msg = if let Some(ref who) = master_alarm.triggered_by {
                format!("MASTER ALARM: {} health critical", who)
            } else {
                "MASTER ALARM: Crew health critical".to_string()
            };
            mission_state.log.push(crate::mission::LogEntry {
                time: time_str,
                message: msg,
            });
        }
        alarm_state.active = master_alarm.active;
        alarm_state.triggered_by = master_alarm.triggered_by.clone();
    }
}
