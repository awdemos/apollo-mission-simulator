use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct EngineCoolingSystem {
    pub fuel_flow_rate_kg_s: f32,
    pub chamber_temp_k: f32,
    pub nozzle_wall_temp_k: f32,
    pub coolant_inlet_temp_k: f32,
    pub coolant_outlet_temp_k: f32,
    pub regen_channels: u32,
    pub coolant_type: CoolantType,
    pub pump_pressure_psi: f32,
    pub status: crate::systems::SystemStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolantType {
    RP1,
    LH2,
}

impl Default for EngineCoolingSystem {
    fn default() -> Self {
        Self {
            fuel_flow_rate_kg_s: 0.0,
            chamber_temp_k: 0.0,
            nozzle_wall_temp_k: 0.0,
            coolant_inlet_temp_k: 0.0,
            coolant_outlet_temp_k: 0.0,
            regen_channels: 0,
            coolant_type: CoolantType::RP1,
            pump_pressure_psi: 0.0,
            status: crate::systems::SystemStatus::Off,
        }
    }
}

pub fn create_f1_cooling() -> EngineCoolingSystem {
    EngineCoolingSystem {
        fuel_flow_rate_kg_s: 788.0,
        chamber_temp_k: 3320.0,
        nozzle_wall_temp_k: 800.0,
        coolant_inlet_temp_k: 273.0,
        coolant_outlet_temp_k: 420.0,
        regen_channels: 1780,
        coolant_type: CoolantType::RP1,
        pump_pressure_psi: 1850.0,
        status: crate::systems::SystemStatus::Nominal,
    }
}

pub fn create_j2_cooling() -> EngineCoolingSystem {
    EngineCoolingSystem {
        fuel_flow_rate_kg_s: 178.0,
        chamber_temp_k: 3220.0,
        nozzle_wall_temp_k: 650.0,
        coolant_inlet_temp_k: 20.0,
        coolant_outlet_temp_k: 180.0,
        regen_channels: 890,
        coolant_type: CoolantType::LH2,
        pump_pressure_psi: 1200.0,
        status: crate::systems::SystemStatus::Nominal,
    }
}

pub fn update_cooling_systems(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut cooling_query: Query<&mut EngineCoolingSystem>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;
    for mut cooling in cooling_query.iter_mut() {
        if cooling.status == crate::systems::SystemStatus::Nominal {
            let heat_absorbed = cooling.fuel_flow_rate_kg_s * 2000.0 * dt;
            cooling.nozzle_wall_temp_k -= heat_absorbed * 0.001;
            cooling.nozzle_wall_temp_k = cooling.nozzle_wall_temp_k.clamp(400.0, 1200.0);
            cooling.coolant_outlet_temp_k += heat_absorbed * 0.0005;
        }
    }
}
