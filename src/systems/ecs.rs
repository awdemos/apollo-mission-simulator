use bevy::prelude::*;
use rand::Rng;
use super::SystemStatus;

/// Primary ECS component for the Command Module.
/// Apollo Block II environmental control with historically accurate parameters.
#[derive(Component, Debug, Clone)]
pub struct EnvironmentalControlSystem {
    pub cabin_atmosphere: CabinAtmosphere,
    pub suit_circuit: SuitCircuit,
    pub cabin_fan: CabinFan,
    pub lioh_canisters: Vec<LiOHCanister>,
    pub water_management: WaterManagement,
    pub oxygen_supply: OxygenSupply,
    pub waste_disposal: WasteDisposal,
}

impl Default for EnvironmentalControlSystem {
    fn default() -> Self {
        Self {
            cabin_atmosphere: CabinAtmosphere::default(),
            suit_circuit: SuitCircuit::default(),
            cabin_fan: CabinFan::default(),
            lioh_canisters: vec![
                LiOHCanister::new("PRIMARY"),
                LiOHCanister::new("SECONDARY"),
            ],
            water_management: WaterManagement::default(),
            oxygen_supply: OxygenSupply::default(),
            waste_disposal: WasteDisposal::default(),
        }
    }
}

/// Cabin atmosphere: 5.0 psia pure O2, 320 cu ft volume.
#[derive(Debug, Clone)]
pub struct CabinAtmosphere {
    pub pressure_psi: f32,
    pub temp_c: f32,
    pub humidity_pct: f32,
    pub co2_partial_pressure_mmhg: f32,
    pub gas_volume_cu_ft: f32,
}

impl Default for CabinAtmosphere {
    fn default() -> Self {
        Self {
            pressure_psi: 5.0,
            temp_c: 21.0,
            humidity_pct: 50.0,
            co2_partial_pressure_mmhg: 0.0,
            gas_volume_cu_ft: 320.0,
        }
    }
}

/// Suit circuit compressor: 55 lb/hr, 85W, 3-phase 110V 400Hz.
#[derive(Debug, Clone)]
pub struct SuitCircuit {
    pub compressor_active: bool,
    pub flow_rate_lb_hr: f32,
    pub pressure_rise_in_h2o: f32,
    pub power_watts: f32,
    pub gas_exit_temp_f: f32,
}

impl Default for SuitCircuit {
    fn default() -> Self {
        Self {
            compressor_active: true,
            flow_rate_lb_hr: 55.0,
            pressure_rise_in_h2o: 10.0,
            power_watts: 85.0,
            gas_exit_temp_f: 50.0,
        }
    }
}

/// Cabin circulation fans (2 fans, 86 cfm each, 27.5W each).
#[derive(Debug, Clone)]
pub struct CabinFan {
    pub enabled: bool,
    pub flow_rate_cfm: f32,
    pub power_watts: f32,
    pub noisy: bool,
}

impl Default for CabinFan {
    fn default() -> Self {
        Self {
            enabled: true,
            flow_rate_cfm: 86.0,
            power_watts: 27.5,
            noisy: true,
        }
    }
}

/// LiOH canister: 1.54 kg capacity, 12 hr replacement, 0.064 kg/hr CO2 removal.
#[derive(Debug, Clone)]
pub struct LiOHCanister {
    pub id: String,
    pub capacity_kg: f32,
    pub saturation_pct: f32,
    pub active: bool,
    pub replacement_hours: f32,
    pub hours_used: f32,
    pub co2_removal_rate_kg_hr: f32,
}

impl LiOHCanister {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            capacity_kg: 1.54,
            saturation_pct: 0.0,
            active: true,
            replacement_hours: 12.0,
            hours_used: 0.0,
            co2_removal_rate_kg_hr: 0.064,
        }
    }
}

/// Water management: potable 16.3 kg, waste 25.4 kg, fuel cells produce 0.77 kg/hr.
#[derive(Debug, Clone)]
pub struct WaterManagement {
    pub potable_tank_kg: f32,
    pub waste_tank_kg: f32,
    pub fuel_cell_production_rate_kg_hr: f32,
    pub system_pressure_psi: f32,
    pub chiller_temp_f: f32,
}

impl Default for WaterManagement {
    fn default() -> Self {
        Self {
            potable_tank_kg: 16.3,
            waste_tank_kg: 25.4,
            fuel_cell_production_rate_kg_hr: 0.77,
            system_pressure_psi: 25.0,
            chiller_temp_f: 45.0,
        }
    }
}

/// Oxygen supply with 2 cryo tanks (640 lb total) and 3-stage regulation.
#[derive(Debug, Clone)]
pub struct OxygenSupply {
    pub primary_tanks: Vec<CryoO2Tank>,
    pub cryo_pumps_active: bool,
    pub regulated_pressure_levels_psi: Vec<f32>,
    pub demand_regulator_psi: f32,
}

impl Default for OxygenSupply {
    fn default() -> Self {
        Self {
            primary_tanks: vec![
                CryoO2Tank::new("O2 TANK 1", 320.0),
                CryoO2Tank::new("O2 TANK 2", 320.0),
            ],
            cryo_pumps_active: true,
            regulated_pressure_levels_psi: vec![100.0, 20.0, 5.0],
            demand_regulator_psi: 5.0,
        }
    }
}

/// Cryogenic O2 tank (approx 320 lb each for CM, supercritical at ~900 psi, -185 C).
#[derive(Debug, Clone)]
pub struct CryoO2Tank {
    pub id: String,
    pub pressure_psi: f32,
    pub capacity_kg: f32,
    pub remaining_kg: f32,
    pub temp_c: f32,
    pub status: SystemStatus,
}

impl CryoO2Tank {
    pub fn new(id: &str, remaining_lb: f32) -> Self {
        let remaining_kg = remaining_lb * 0.453592;
        Self {
            id: id.to_string(),
            pressure_psi: 900.0,
            capacity_kg: remaining_kg,
            remaining_kg,
            temp_c: -185.0,
            status: SystemStatus::Nominal,
        }
    }
}

/// Apollo waste disposal: urine overboard dump + fecal bag storage.
/// Period-accurate for Block II CM (post-Apollo 11 URA variant).
#[derive(Debug, Clone)]
pub struct WasteDisposal {
    pub urine_system: UrineCollectionSystem,
    pub fecal_system: FecalCollectionSystem,
    pub stowage_compartment: WasteStowageCompartment,
    pub crew_count: u8,
}

impl Default for WasteDisposal {
    fn default() -> Self {
        Self {
            urine_system: UrineCollectionSystem::default(),
            fecal_system: FecalCollectionSystem::default(),
            stowage_compartment: WasteStowageCompartment::default(),
            crew_count: 3,
        }
    }
}

/// Urine Collection and Transfer Assembly (UCTA).
/// Post-Apollo 11: Urine Receiver Assembly (URA) - no crew contact required.
#[derive(Debug, Clone)]
pub struct UrineCollectionSystem {
    pub bag_capacity_liters: f32,
    pub bag_volume_liters: f32,
    pub receiver_connected: bool,
    pub valve_open: bool,
    pub dump_nozzle_heater_on: bool,
    pub quick_disconnect_sealed: bool,
    pub status: SystemStatus,
}

impl Default for UrineCollectionSystem {
    fn default() -> Self {
        Self {
            bag_capacity_liters: 0.8,
            bag_volume_liters: 0.0,
            receiver_connected: false,
            valve_open: false,
            dump_nozzle_heater_on: true,
            quick_disconnect_sealed: true,
            status: SystemStatus::Nominal,
        }
    }
}

/// Fecal collection: adhesive bag with germicide.
#[derive(Debug, Clone)]
pub struct FecalCollectionSystem {
    pub bags_remaining: u8,
    pub bags_used: u8,
    pub germicide_available: bool,
    pub tissues_available: bool,
    pub status: SystemStatus,
}

const BAGS_PER_CREWMEMBER: u8 = 20;

impl Default for FecalCollectionSystem {
    fn default() -> Self {
        Self {
            bags_remaining: BAGS_PER_CREWMEMBER * 3,
            bags_used: 0,
            germicide_available: true,
            tissues_available: true,
            status: SystemStatus::Nominal,
        }
    }
}

/// Waste stowage compartment in CM lower equipment bay.
/// Sealed container for used fecal bags, with odor purge vent.
#[derive(Debug, Clone)]
pub struct WasteStowageCompartment {
    pub capacity_bags: u8,
    pub stored_bags: u8,
    pub odor_purge_active: bool,
    pub sealed: bool,
    pub status: SystemStatus,
}

impl Default for WasteStowageCompartment {
    fn default() -> Self {
        Self {
            capacity_bags: BAGS_PER_CREWMEMBER * 3,
            stored_bags: 0,
            odor_purge_active: false,
            sealed: true,
            status: SystemStatus::Nominal,
        }
    }
}

pub fn update_environmental_control_system(ecs: &mut EnvironmentalControlSystem, dt: f32) {
    let o2_consumption_kg_hr = 0.0625;
    let o2_consumption_kg = o2_consumption_kg_hr * dt / 3600.0;

    let mut total_o2_remaining = 0.0;
    for tank in &mut ecs.oxygen_supply.primary_tanks {
        if tank.status != SystemStatus::Failed && tank.remaining_kg > 0.0 {
            let draw = o2_consumption_kg.min(tank.remaining_kg);
            tank.remaining_kg -= draw;
            if tank.capacity_kg > 0.0 {
                tank.pressure_psi = (tank.remaining_kg / tank.capacity_kg) * 900.0;
            } else {
                tank.pressure_psi = 0.0;
            }
            if tank.pressure_psi < 100.0 {
                tank.status = SystemStatus::Warning;
            }
            if tank.remaining_kg <= 0.0 {
                tank.status = SystemStatus::Failed;
                tank.pressure_psi = 0.0;
            }
        }
        total_o2_remaining += tank.remaining_kg;
    }

    let co2_generation_kg_hr = 0.125;
    let co2_generated = co2_generation_kg_hr * dt / 3600.0;

    let mut co2_removed = 0.0;
    for canister in &mut ecs.lioh_canisters {
        if canister.active && canister.saturation_pct < 100.0 {
            let removal = (canister.co2_removal_rate_kg_hr * dt / 3600.0)
                .min(co2_generated - co2_removed)
                .max(0.0);
            co2_removed += removal;
            canister.hours_used += dt / 3600.0;
            if canister.replacement_hours > 0.0 {
                canister.saturation_pct = (canister.hours_used / canister.replacement_hours * 100.0)
                    .min(100.0);
            } else {
                canister.saturation_pct = 100.0;
            }
            if canister.saturation_pct >= 100.0 {
                canister.active = false;
            }
        }
    }

    let net_co2_kg = (co2_generated - co2_removed).max(0.0);
    ecs.cabin_atmosphere.co2_partial_pressure_mmhg += net_co2_kg * 0.5;
    ecs.cabin_atmosphere.co2_partial_pressure_mmhg =
        ecs.cabin_atmosphere.co2_partial_pressure_mmhg.clamp(0.0, 15.0);

    if total_o2_remaining > 0.0 {
        ecs.cabin_atmosphere.pressure_psi = ecs.oxygen_supply.demand_regulator_psi;
    } else {
        ecs.cabin_atmosphere.pressure_psi -= 0.001 * dt;
        ecs.cabin_atmosphere.pressure_psi = ecs.cabin_atmosphere.pressure_psi.max(0.0);
    }

    if ecs.cabin_fan.enabled {
        ecs.cabin_atmosphere.temp_c = ecs.cabin_atmosphere.temp_c.lerp(21.0, 0.5 * dt);
    } else {
        ecs.cabin_atmosphere.temp_c += 0.001 * dt;
        ecs.cabin_atmosphere.temp_c = ecs.cabin_atmosphere.temp_c.min(30.0);
    }

    let water_produced = ecs.water_management.fuel_cell_production_rate_kg_hr * dt / 3600.0;
    ecs.water_management.potable_tank_kg += water_produced;
    if ecs.water_management.potable_tank_kg > 20.0 {
        ecs.water_management.waste_tank_kg +=
            ecs.water_management.potable_tank_kg - 20.0;
        ecs.water_management.potable_tank_kg = 20.0;
    }

    ecs.suit_circuit.power_watts = if ecs.suit_circuit.compressor_active {
        85.0
    } else {
        0.0
    };

    update_waste_disposal(&mut ecs.waste_disposal, dt);
}

fn update_waste_disposal(waste: &mut WasteDisposal, dt: f32) {
    let urine_generation_rate_l_hr = 0.0625 * waste.crew_count as f32;
    let urine_generated = urine_generation_rate_l_hr * dt / 3600.0;

    if waste.urine_system.receiver_connected {
        waste.urine_system.bag_volume_liters += urine_generated;

        if waste.urine_system.valve_open && waste.urine_system.bag_volume_liters > 0.0 {
            let dump_rate_l_hr = 2.0;
            let dumped = (dump_rate_l_hr * dt / 3600.0).min(waste.urine_system.bag_volume_liters);
            waste.urine_system.bag_volume_liters -= dumped;
        }

        if waste.urine_system.bag_volume_liters >= waste.urine_system.bag_capacity_liters {
            waste.urine_system.bag_volume_liters = waste.urine_system.bag_capacity_liters;
            waste.urine_system.status = SystemStatus::Warning;
        } else if waste.urine_system.bag_volume_liters > waste.urine_system.bag_capacity_liters * 0.8 {
            waste.urine_system.status = SystemStatus::Caution;
        } else {
            waste.urine_system.status = SystemStatus::Nominal;
        }
    }

    if !waste.urine_system.dump_nozzle_heater_on && waste.urine_system.valve_open {
        waste.urine_system.status = SystemStatus::Warning;
    }

    let fecal_events_per_day = 0.4 * waste.crew_count as f32;
    let fecal_probability = fecal_events_per_day * dt / 86400.0;
    if rand::thread_rng().gen::<f32>() < fecal_probability && waste.fecal_system.bags_remaining > 0 {
        waste.fecal_system.bags_remaining -= 1;
        waste.fecal_system.bags_used += 1;
        waste.stowage_compartment.stored_bags += 1;

        if waste.stowage_compartment.stored_bags >= waste.stowage_compartment.capacity_bags {
            waste.stowage_compartment.status = SystemStatus::Warning;
            waste.fecal_system.status = SystemStatus::Warning;
        }
    }

    if waste.stowage_compartment.odor_purge_active {
        waste.stowage_compartment.odor_purge_active = false;
    }
}

#[derive(Component, Debug, Clone)]
pub struct LifeSupportSystem {
    pub o2_tanks: Vec<OxygenTank>,
    pub co2_scrubbers: Vec<Co2Scrubber>,
    pub cabin_pressure_psi: f32,
    pub cabin_temp_c: f32,
    pub water_tank_kg: f32,
}

#[derive(Debug, Clone)]
pub struct OxygenTank {
    pub id: String,
    pub pressure_psi: f32,
    pub capacity_kg: f32,
    pub remaining_kg: f32,
    pub temp_c: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct Co2Scrubber {
    pub id: String,
    pub canister_hours: f32,
    pub co2_ppm: f32,
    pub status: SystemStatus,
}
