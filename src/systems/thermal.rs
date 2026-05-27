use bevy::prelude::*;
use super::SystemStatus;

#[derive(Component, Debug, Clone)]
pub struct ThermalControlSystem {
    pub glycol_loops: Vec<GlycolLoop>,
    pub radiators: Vec<SpaceRadiator>,
    pub evaporators: Vec<GlycolEvaporator>,
    pub cabin_temp_control: CabinTemperatureControl,
    pub equipment_cooling: EquipmentCooling,
}

impl Default for ThermalControlSystem {
    fn default() -> Self {
        Self {
            glycol_loops: vec![
                GlycolLoop::new("PRIMARY"),
                GlycolLoop::new("SECONDARY"),
            ],
            radiators: vec![
                SpaceRadiator::new("RAD_1"),
                SpaceRadiator::new("RAD_2"),
            ],
            evaporators: vec![
                GlycolEvaporator::new("EVAP_1"),
            ],
            cabin_temp_control: CabinTemperatureControl::default(),
            equipment_cooling: EquipmentCooling::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlycolLoop {
    pub id: String,
    pub flow_rate_lb_hr: f32,
    pub coolant_mixture_pct: f32,
    pub inlet_temp_f: f32,
    pub outlet_temp_f: f32,
    pub pump_active: bool,
}

impl GlycolLoop {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            flow_rate_lb_hr: 200.0,
            coolant_mixture_pct: 62.5,
            inlet_temp_f: 50.0,
            outlet_temp_f: 41.5,
            pump_active: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpaceRadiator {
    pub id: String,
    pub area_sq_ft: f32,
    pub heat_rejection_btu_hr: f32,
    pub primary_tubes: u8,
    pub secondary_tubes: u8,
    pub heater_on_temp_f: f32,
    pub status: SystemStatus,
}

impl SpaceRadiator {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            area_sq_ft: 30.0,
            heat_rejection_btu_hr: 4850.0,
            primary_tubes: 5,
            secondary_tubes: 4,
            heater_on_temp_f: 45.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlycolEvaporator {
    pub id: String,
    pub dimensions_in: [f32; 3],
    pub weight_lb: f32,
    pub boiling_temp_f: f32,
    pub heat_removal_btu_hr: f32,
    pub steam_pressure_psia: f32,
    pub water_feed_rate_lb_hr: f32,
}

impl GlycolEvaporator {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            dimensions_in: [8.0, 4.7, 6.62],
            weight_lb: 18.0,
            boiling_temp_f: 37.5,
            heat_removal_btu_hr: 8000.0,
            steam_pressure_psia: 0.1,
            water_feed_rate_lb_hr: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CabinTemperatureControl {
    pub control_valve_position: f32,
    pub cabin_heat_exchanger_temp_f: f32,
    pub target_temp_f: f32,
}

impl Default for CabinTemperatureControl {
    fn default() -> Self {
        Self {
            control_valve_position: 0.5,
            cabin_heat_exchanger_temp_f: 50.0,
            target_temp_f: 75.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColdPlate {
    pub id: String,
    pub coolant_flow_lb_hr: f32,
    pub equipment_temp_f: f32,
    pub status: SystemStatus,
}

impl ColdPlate {
    pub fn new(id: &str, flow_lb_hr: f32) -> Self {
        Self {
            id: id.to_string(),
            coolant_flow_lb_hr: flow_lb_hr,
            equipment_temp_f: 100.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EquipmentCooling {
    pub coldplates: Vec<ColdPlate>,
    pub imu_flow_lb_hr: f32,
    pub suit_heat_exchanger_flow_lb_hr: f32,
}

impl Default for EquipmentCooling {
    fn default() -> Self {
        let mut coldplates = Vec::new();
        for i in 1..=22 {
            let flow = match i {
                1..=5 => 10.0,
                6..=10 => 8.0,
                11..=15 => 6.0,
                16..=20 => 5.0,
                _ => 4.0,
            };
            coldplates.push(ColdPlate::new(&format!("CP{}", i), flow));
        }
        Self {
            coldplates,
            imu_flow_lb_hr: 35.0,
            suit_heat_exchanger_flow_lb_hr: 165.0,
        }
    }
}

pub fn update_thermal_control_system(tcs: &mut ThermalControlSystem, dt: f32, electrical_power_kw: f32) {
    let electrical_heat_load = electrical_power_kw * 1000.0 * 3.4;

    for loop_system in &mut tcs.glycol_loops {
        if loop_system.pump_active {
            let temp_rise = electrical_heat_load / (loop_system.flow_rate_lb_hr * 0.9);
            loop_system.inlet_temp_f = loop_system.outlet_temp_f + temp_rise * 0.1 * dt;
            loop_system.inlet_temp_f = loop_system.inlet_temp_f.clamp(30.0, 120.0);
        }
    }

    let mut total_radiator_rejection = 0.0;
    for radiator in &mut tcs.radiators {
        if radiator.status != SystemStatus::Failed {
            let temp_diff = radiator.heater_on_temp_f.max(50.0) - 45.0;
            let rejection = radiator.heat_rejection_btu_hr * (1.0 + temp_diff * 0.01);
            total_radiator_rejection += rejection;
        }
    }

    for evaporator in &mut tcs.evaporators {
        if tcs.glycol_loops.iter().any(|l| l.pump_active) {
            let heat_to_remove = (electrical_heat_load - total_radiator_rejection).max(0.0);
            evaporator.water_feed_rate_lb_hr = (heat_to_remove / 1060.0).min(10.0);
            evaporator.heat_removal_btu_hr = heat_to_remove.min(8000.0);
        } else {
            evaporator.water_feed_rate_lb_hr = 0.0;
        }
    }

    let cabin_temp_error = tcs.cabin_temp_control.target_temp_f - tcs.cabin_temp_control.cabin_heat_exchanger_temp_f;
    tcs.cabin_temp_control.control_valve_position = ((cabin_temp_error / 50.0) + 0.5).clamp(0.0, 1.0);

    let coldplate_count = tcs.equipment_cooling.coldplates.len();
    for coldplate in &mut tcs.equipment_cooling.coldplates {
        if coldplate.status != SystemStatus::Failed {
            let heat_per_plate = electrical_heat_load / coldplate_count as f32;
            let temp_rise = heat_per_plate / (coldplate.coolant_flow_lb_hr * 0.9);
            coldplate.equipment_temp_f += temp_rise * 0.01 * dt;
            coldplate.equipment_temp_f = coldplate.equipment_temp_f.clamp(60.0, 200.0);

            if coldplate.equipment_temp_f > 150.0 {
                coldplate.status = SystemStatus::Warning;
            }
            if coldplate.equipment_temp_f > 180.0 {
                coldplate.status = SystemStatus::Critical;
            }
        }
    }

    let glycol_loop_count = tcs.glycol_loops.len();
    for loop_system in &mut tcs.glycol_loops {
        if loop_system.pump_active {
            let cooling_effect = (total_radiator_rejection
                + tcs.evaporators.iter().map(|e| e.heat_removal_btu_hr).sum::<f32>())
                / glycol_loop_count as f32;
            let temp_drop = cooling_effect / (loop_system.flow_rate_lb_hr * 0.9);
            loop_system.outlet_temp_f = loop_system.inlet_temp_f - temp_drop * 0.1 * dt;
            loop_system.outlet_temp_f = loop_system.outlet_temp_f.clamp(35.0, 100.0);
        }
    }
}
