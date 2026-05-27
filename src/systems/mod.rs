use bevy::prelude::*;

pub mod csm;
pub mod lm;
pub mod saturn_v;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(csm::CsmSystemsPlugin)
            .add_plugins(lm::LmSystemsPlugin)
            .add_plugins(saturn_v::SaturnVSystemsPlugin);
    }
}

#[derive(Component, Debug, Clone)]
pub struct ElectricalSystem {
    pub fuel_cells: Vec<FuelCell>,
    pub batteries: Vec<Battery>,
    pub bus_voltage: f32,
    pub total_power_kw: f32,
    pub main_bus_a: bool,
    pub main_bus_b: bool,
}

impl Default for ElectricalSystem {
    fn default() -> Self {
        Self {
            fuel_cells: vec![
                FuelCell::new("FC1", 1.4),
                FuelCell::new("FC2", 1.4),
                FuelCell::new("FC3", 1.4),
            ],
            batteries: vec![
                Battery::new("BAT A", 40.0),
                Battery::new("BAT B", 40.0),
                Battery::new("BAT C", 40.0),
            ],
            bus_voltage: 28.0,
            total_power_kw: 2.0,
            main_bus_a: true,
            main_bus_b: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuelCell {
    pub id: String,
    pub output_kw: f32,
    pub temp_c: f32,
    pub status: SystemStatus,
}

impl FuelCell {
    pub fn new(id: &str, output_kw: f32) -> Self {
        Self {
            id: id.to_string(),
            output_kw,
            temp_c: 200.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Battery {
    pub id: String,
    pub capacity_ah: f32,
    pub charge_pct: f32,
    pub status: SystemStatus,
}

impl Battery {
    pub fn new(id: &str, capacity_ah: f32) -> Self {
        Self {
            id: id.to_string(),
            capacity_ah,
            charge_pct: 100.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct PropulsionSystem {
    pub engine: Engine,
    pub fuel_tanks: Vec<PropellantTank>,
    pub thrust_newtons: f32,
    pub isp_seconds: f32,
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub name: String,
    pub thrust_sl_newtons: f32,
    pub thrust_vac_newtons: f32,
    pub chamber_pressure_psi: f32,
    pub mixture_ratio: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct PropellantTank {
    pub name: String,
    pub fuel_kg: f32,
    pub oxidizer_kg: f32,
    pub capacity_kg: f32,
    pub ullage_pct: f32,
    pub pressure_psi: f32,
}

impl PropellantTank {
    pub fn new(name: &str, fuel_kg: f32, oxidizer_kg: f32) -> Self {
        let capacity_kg = fuel_kg + oxidizer_kg;
        Self {
            name: name.to_string(),
            fuel_kg,
            oxidizer_kg,
            capacity_kg,
            ullage_pct: 5.0,
            pressure_psi: 20.0,
        }
    }
    
    pub fn total_propellant_kg(&self) -> f32 {
        self.fuel_kg + self.oxidizer_kg
    }
    
    pub fn propellant_pct(&self) -> f32 {
        (self.total_propellant_kg() / self.capacity_kg) * 100.0
    }
}

#[derive(Component, Debug, Clone)]
pub struct RcsSystem {
    pub quads: Vec<RcsQuad>,
    pub propellant_kg: f32,
    pub pressure_psi: f32,
}

#[derive(Debug, Clone)]
pub struct RcsQuad {
    pub id: String,
    pub thrusters: [RcsThruster; 4],
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct RcsThruster {
    pub thrust_n: f32,
    pub status: SystemStatus,
    pub fired_count: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    Nominal,
    Caution,
    Warning,
    Critical,
    Failed,
    Off,
}

impl SystemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemStatus::Nominal => "NOMINAL",
            SystemStatus::Caution => "CAUTION",
            SystemStatus::Warning => "WARNING",
            SystemStatus::Critical => "CRITICAL",
            SystemStatus::Failed => "FAILED",
            SystemStatus::Off => "OFF",
        }
    }
}
