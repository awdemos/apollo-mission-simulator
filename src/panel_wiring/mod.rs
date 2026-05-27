use bevy::prelude::*;
use crate::panels::{PanelInteraction, SwitchId, SwitchState, BreakerId};
use crate::systems::csm::CommandServiceModule;
use crate::communications::CommunicationsBus;

pub struct PanelWiringPlugin;

impl Plugin for PanelWiringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_panel_system_wiring);
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
        _ => {}
    }
}
