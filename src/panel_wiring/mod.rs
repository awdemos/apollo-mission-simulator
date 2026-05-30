use bevy::prelude::*;
use crate::panels::{PanelInteraction, SwitchId, SwitchState, BreakerId};
use crate::systems::csm::CommandServiceModule;
use crate::communications::{CommunicationsBus, SignalConditioningEquipment, SceStatus};

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
    mut sce_query: Query<&mut SignalConditioningEquipment>,
) {
    for event in interaction_events.read() {
        match event {
            PanelInteraction::SwitchToggled(_, id, state) => {
                if let Ok(mut csm) = csm_query.get_single_mut() {
                    apply_switch_to_csm(&mut csm, id, state);
                }
                apply_switch_to_comms(&mut comms_bus, id, state);
                if let Ok(mut sce) = sce_query.get_single_mut() {
                    apply_switch_to_sce(&mut sce, id, state);
                }
            }
            PanelInteraction::BreakerToggled(_, id, closed) => {
                if let Ok(mut csm) = csm_query.get_single_mut() {
                    apply_breaker_to_csm(&mut csm, id, closed);
                }
                apply_breaker_to_comms(&mut comms_bus, id, closed);
                if let Ok(mut sce) = sce_query.get_single_mut() {
                    apply_breaker_to_sce(&mut sce, id, closed);
                }
            }
            _ => {}
        }
    }
}

fn apply_switch_to_csm(csm: &mut CommandServiceModule, id: &SwitchId, state: &SwitchState) {
    let on = *state == SwitchState::On;
    let on_or_auto = on || *state == SwitchState::Auto;

    match id {
        // === RCS System ===
        SwitchId::RcsQuadA => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "A") {
                quad.enabled = on;
            }
        }
        SwitchId::RcsQuadB => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "B") {
                quad.enabled = on;
            }
        }
        SwitchId::RcsQuadC => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "C") {
                quad.enabled = on;
            }
        }
        SwitchId::RcsQuadD => {
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == "D") {
                quad.enabled = on;
            }
        }
        SwitchId::CmRcsHeaters => {
            for quad in &mut csm.rcs.quads {
                for thruster in &mut quad.thrusters {
                    if thruster.status == crate::systems::SystemStatus::Off {
                        thruster.status = crate::systems::SystemStatus::Nominal;
                    }
                }
            }
        }
        SwitchId::SmRcsHeaters => {
            for quad in &mut csm.rcs.quads {
                for thruster in &mut quad.thrusters {
                    if thruster.status == crate::systems::SystemStatus::Off {
                        thruster.status = crate::systems::SystemStatus::Nominal;
                    }
                }
            }
        }

        // === Electrical Power System ===
        SwitchId::MainBusA => {
            csm.electrical.main_bus_a = on;
        }
        SwitchId::MainBusB => {
            csm.electrical.main_bus_b = on;
        }
        SwitchId::FuelCell1 => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC1") {
                if on {
                    fc.status = crate::systems::SystemStatus::Nominal;
                } else {
                    fc.status = crate::systems::SystemStatus::Off;
                }
            }
        }
        SwitchId::FuelCell2 => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC2") {
                if on {
                    fc.status = crate::systems::SystemStatus::Nominal;
                } else {
                    fc.status = crate::systems::SystemStatus::Off;
                }
            }
        }
        SwitchId::FuelCell3 => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC3") {
                if on {
                    fc.status = crate::systems::SystemStatus::Nominal;
                } else {
                    fc.status = crate::systems::SystemStatus::Off;
                }
            }
        }
        SwitchId::BatteryA => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT A") {
                bat.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        SwitchId::BatteryB => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT B") {
                bat.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        SwitchId::BatteryC => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT C") {
                bat.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        SwitchId::Inverter1 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV1") {
                inv.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        SwitchId::Inverter2 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV2") {
                inv.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }
        SwitchId::Inverter3 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV3") {
                inv.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
            }
        }

        // === Environmental Control System ===
        SwitchId::O2Fan1 => {
            csm.environmental_control.suit_circuit.compressor_active = on_or_auto;
        }
        SwitchId::O2Fan2 => {
            csm.environmental_control.suit_circuit.compressor_active = on_or_auto;
        }
        SwitchId::H2Fan1 | SwitchId::H2Fan2 => {
            csm.environmental_control.oxygen_supply.cryo_pumps_active = on_or_auto;
        }
        SwitchId::CabinFan => {
            csm.environmental_control.cabin_fan.enabled = on;
        }
        SwitchId::CryoPumps => {
            csm.environmental_control.oxygen_supply.cryo_pumps_active = on_or_auto;
        }

        // === SPS Engine Control ===
        SwitchId::EngineArm => {
            csm.sps.engine.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
        }
        SwitchId::SpsEnable => {
            csm.sps.engine.status = if on { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };
        }
        SwitchId::SpsThrustOn => {
            if csm.sps.engine.status != crate::systems::SystemStatus::Off {
                csm.sps.engine.status = crate::systems::SystemStatus::Firing;
                csm.sps.thrust_newtons = csm.sps.engine.thrust_vac_newtons;
            }
        }
        SwitchId::SpsThrustOff => {
            if csm.sps.engine.status == crate::systems::SystemStatus::Firing {
                csm.sps.engine.status = crate::systems::SystemStatus::Nominal;
                csm.sps.thrust_newtons = 0.0;
            }
        }
        SwitchId::UllageThrust => {
            for quad in &mut csm.rcs.quads {
                if quad.enabled {
                    for thruster in &mut quad.thrusters {
                        if on {
                            thruster.status = crate::systems::SystemStatus::Firing;
                        } else if thruster.status == crate::systems::SystemStatus::Firing {
                            thruster.status = crate::systems::SystemStatus::Nominal;
                        }
                    }
                }
            }
        }

        // === Guidance, Navigation & Control ===
        SwitchId::ImuAlign => {
            csm.gnc.imu_aligned = on;
        }
        SwitchId::ImuCage => {
            csm.gnc.imu_aligned = false;
            csm.gnc.gimbal_angles = [0.0, 0.0, 0.0];
        }
        SwitchId::GncMode => {
            csm.gnc.standby_mode = !on;
        }
        SwitchId::RhcPower => {
            csm.gnc.rhc_powered = on;
        }
        SwitchId::ThcPower => {
            csm.gnc.thc_powered = on;
        }

        // === Entry & Abort ===
        SwitchId::EntryMonitor => {
            csm.gnc.standby_mode = !on;
        }
        SwitchId::AbortMode => {
            csm.abort_mode_active = on;
        }

        // === SCE (Signal Conditioning Equipment) ===
        SwitchId::ScePower => {
        }

        // === Stabilization Control System ===
        SwitchId::ScsMode => {
            csm.gnc.standby_mode = !on;
        }
        SwitchId::TvCEnable => {
            csm.gnc.tvc_enabled = on;
        }

        // === Data Storage Equipment ===
        SwitchId::DsePower => {
            csm.dse_powered = on;
        }
        SwitchId::TapeRecorder => {
            csm.tape_recorder_active = on;
        }

        _ => {}
    }
}

fn apply_switch_to_comms(comms: &mut CommunicationsBus, id: &SwitchId, state: &SwitchState) {
    let on = *state == SwitchState::On;

    match id {
        SwitchId::SBandPower => {
            comms.mode = if on {
                crate::communications::CommMode::TelemetryAndVoice
            } else {
                crate::communications::CommMode::Off
            };
        }
        SwitchId::VhfPower => {
            if on {
                comms.signal_strength = comms.signal_strength.max(0.5);
            }
        }
        SwitchId::HighGainAntenna => {
            if on {
                comms.transmitter_power_watts = 20.0;
                comms.signal_strength = comms.signal_strength.max(0.8);
            } else {
                comms.transmitter_power_watts = 2.8;
            }
        }
        _ => {}
    }
}

fn apply_switch_to_sce(sce: &mut SignalConditioningEquipment, id: &SwitchId, state: &SwitchState) {
    let on = *state == SwitchState::On;

    match id {
        SwitchId::ScePower => {
            if on {
                sce.signal_conditioner_status = SceStatus::Primary;
                sce.primary_power = true;
            } else {
                sce.signal_conditioner_status = SceStatus::Off;
                sce.primary_power = false;
                sce.auxiliary_power = false;
            }
        }
        _ => {}
    }
}

fn apply_breaker_to_csm(csm: &mut CommandServiceModule, id: &BreakerId, closed: &bool) {
    let nominal = if *closed { crate::systems::SystemStatus::Nominal } else { crate::systems::SystemStatus::Off };

    match id {
        // === Fuel Cell Breakers ===
        BreakerId::FuelCell1MainBusA => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC1") {
                fc.status = nominal;
            }
        }
        BreakerId::FuelCell2MainBusA => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC2") {
                fc.status = nominal;
            }
        }
        BreakerId::FuelCell3MainBusB => {
            if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC3") {
                fc.status = nominal;
            }
        }

        // === Battery Breakers ===
        BreakerId::BatteryRelayBus => {
            for bat in &mut csm.electrical.batteries {
                if !closed { bat.status = crate::systems::SystemStatus::Off; }
            }
        }
        BreakerId::BatteryAEntry => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT A") {
                bat.status = nominal;
            }
        }
        BreakerId::BatteryBEntry => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT B") {
                bat.status = nominal;
            }
        }
        BreakerId::BatteryCEntry => {
            if let Some(bat) = csm.electrical.batteries.iter_mut().find(|b| b.id == "BAT C") {
                bat.status = nominal;
            }
        }

        // === Inverter Breakers ===
        BreakerId::Inverter1 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV1") {
                inv.status = nominal;
            }
        }
        BreakerId::Inverter2 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV2") {
                inv.status = nominal;
            }
        }
        BreakerId::Inverter3 => {
            if let Some(inv) = csm.electrical.inverters.iter_mut().find(|i| i.id == "INV3") {
                inv.status = nominal;
            }
        }

        // === RCS Breakers ===
        BreakerId::RcsQuadAHeater | BreakerId::RcsQuadBHeater
        | BreakerId::RcsQuadCHeater | BreakerId::RcsQuadDHeater => {
            if !closed {
                let quad_id = match id {
                    BreakerId::RcsQuadAHeater => "A",
                    BreakerId::RcsQuadBHeater => "B",
                    BreakerId::RcsQuadCHeater => "C",
                    BreakerId::RcsQuadDHeater => "D",
                    _ => unreachable!(),
                };
                if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == quad_id) {
                    for thruster in &mut quad.thrusters {
                        thruster.status = crate::systems::SystemStatus::Off;
                    }
                }
            }
        }
        BreakerId::RcsQuadAProp | BreakerId::RcsQuadBProp
        | BreakerId::RcsQuadCProp | BreakerId::RcsQuadDProp => {
            let quad_id = match id {
                BreakerId::RcsQuadAProp => "A",
                BreakerId::RcsQuadBProp => "B",
                BreakerId::RcsQuadCProp => "C",
                BreakerId::RcsQuadDProp => "D",
                _ => unreachable!(),
            };
            if let Some(quad) = csm.rcs.quads.iter_mut().find(|q| q.id == quad_id) {
                quad.enabled = *closed;
            }
        }

        // === SPS Breakers ===
        BreakerId::SpsPropellant | BreakerId::SpsHelium | BreakerId::SpsPilotValve => {
            if !closed {
                csm.sps.engine.status = crate::systems::SystemStatus::Off;
                csm.sps.thrust_newtons = 0.0;
            } else if csm.sps.engine.status == crate::systems::SystemStatus::Off {
                csm.sps.engine.status = crate::systems::SystemStatus::Nominal;
            }
        }

        // === ECS Breakers ===
        BreakerId::O2Tank1 | BreakerId::O2Tank2 => {
            let tank_id = match id {
                BreakerId::O2Tank1 => 0,
                BreakerId::O2Tank2 => 1,
                _ => unreachable!(),
            };
            if let Some(tank) = csm.environmental_control.oxygen_supply.primary_tanks.get_mut(tank_id) {
                tank.pressure_psi = if *closed { tank.pressure_psi.max(900.0) } else { 0.0 };
            }
        }
        BreakerId::H2Tank1 | BreakerId::H2Tank2 => {
            if !closed {
                csm.environmental_control.oxygen_supply.cryo_pumps_active = false;
            }
        }
        BreakerId::CryoFan1 | BreakerId::CryoFan2 => {
            csm.environmental_control.oxygen_supply.cryo_pumps_active = *closed;
        }
        BreakerId::CabinFan1 | BreakerId::CabinFan2 => {
            csm.environmental_control.cabin_fan.enabled = *closed;
        }

        // === SCE Breakers (handled in apply_breaker_to_sce) ===

        // === GNC Breakers ===
        BreakerId::GncPlatform => {
            if !closed {
                csm.gnc.imu_aligned = false;
            }
        }
        BreakerId::ImuOper => {
            if !closed {
                csm.gnc.imu_aligned = false;
                csm.gnc.standby_mode = true;
            }
        }
        BreakerId::ImuCage => {
            if *closed {
                csm.gnc.imu_aligned = false;
                csm.gnc.gimbal_angles = [0.0, 0.0, 0.0];
            }
        }
        BreakerId::CmcPower => {
            if !closed {
                csm.gnc.standby_mode = true;
            }
        }
        BreakerId::IoaPower | BreakerId::IobPower => {
            if !closed {
                match id {
                    BreakerId::IoaPower => csm.gnc.io_a_powered = false,
                    BreakerId::IobPower => csm.gnc.io_b_powered = false,
                    _ => {}
                }
            } else {
                match id {
                    BreakerId::IoaPower => csm.gnc.io_a_powered = true,
                    BreakerId::IobPower => csm.gnc.io_b_powered = true,
                    _ => {}
                }
            }
        }

        // === Communications Breakers (handled in apply_breaker_to_comms) ===
        BreakerId::SBandTransmitter | BreakerId::SBandReceiver | BreakerId::SBandPowerAmp
        | BreakerId::VhfTransmitterA | BreakerId::VhfTransmitterB | BreakerId::VhfReceiver
        | BreakerId::HighGainAntenna => {}

        // === SCE Breakers (handled in apply_breaker_to_sce) ===
        BreakerId::SceA | BreakerId::SceB => {}

        // === Data Storage Breakers ===
        BreakerId::Dse => {
            csm.dse_powered = *closed;
        }
        BreakerId::TapeRecorder => {
            csm.tape_recorder_active = *closed;
        }
        BreakerId::EventTimer => {
            csm.event_timer_powered = *closed;
        }

        // === Lighting Breakers ===
        BreakerId::InteriorLightsFlood | BreakerId::InteriorLightsPanel => {
            match id {
                BreakerId::InteriorLightsFlood => csm.flood_lights_on = *closed,
                BreakerId::InteriorLightsPanel => csm.panel_lights_on = *closed,
                _ => {}
            }
            csm.interior_lights_on = csm.flood_lights_on || csm.panel_lights_on;
        }
        BreakerId::UvLight => {
            csm.uv_lights_on = *closed;
        }

        _ => {}
    }
}

fn apply_breaker_to_comms(comms: &mut CommunicationsBus, id: &BreakerId, closed: &bool) {
    match id {
        BreakerId::SBandTransmitter => {
            comms.s_band_transmitter_on = *closed;
            if !closed {
                comms.signal_strength *= 0.5;
            }
        }
        BreakerId::SBandReceiver => {
            comms.s_band_receiver_on = *closed;
            if !closed {
                comms.mode = crate::communications::CommMode::Off;
            }
        }
        BreakerId::SBandPowerAmp => {
            comms.s_band_power_amp_on = *closed;
            if !closed {
                comms.transmitter_power_watts = 2.8;
            } else {
                comms.transmitter_power_watts = 20.0;
            }
        }
        BreakerId::VhfTransmitterA => {
            comms.vhf_transmitter_a_on = *closed;
        }
        BreakerId::VhfTransmitterB => {
            comms.vhf_transmitter_b_on = *closed;
        }
        BreakerId::VhfReceiver => {
            comms.vhf_receiver_on = *closed;
        }
        BreakerId::HighGainAntenna => {
            comms.high_gain_antenna_on = *closed;
            if !closed {
                comms.transmitter_power_watts = 2.8;
                comms.signal_strength = comms.signal_strength.min(0.5);
            } else {
                comms.transmitter_power_watts = 20.0;
                comms.signal_strength = comms.signal_strength.max(0.8);
            }
        }
        _ => {}
    }
}

fn apply_breaker_to_sce(sce: &mut SignalConditioningEquipment, id: &BreakerId, closed: &bool) {
    match id {
        BreakerId::SceA => {
            sce.primary_power = *closed;
            if *closed && sce.signal_conditioner_status == SceStatus::Off {
                sce.signal_conditioner_status = SceStatus::Primary;
            } else if !closed && sce.signal_conditioner_status == SceStatus::Primary {
                if sce.auxiliary_power {
                    sce.signal_conditioner_status = SceStatus::Auxiliary;
                } else {
                    sce.signal_conditioner_status = SceStatus::Off;
                }
            }
        }
        BreakerId::SceB => {
            sce.auxiliary_power = *closed;
            if *closed && sce.signal_conditioner_status == SceStatus::Off {
                sce.signal_conditioner_status = SceStatus::Auxiliary;
            } else if !closed && sce.signal_conditioner_status == SceStatus::Auxiliary {
                if sce.primary_power {
                    sce.signal_conditioner_status = SceStatus::Primary;
                } else {
                    sce.signal_conditioner_status = SceStatus::Off;
                }
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
