use bevy::prelude::*;

pub mod csm;
pub mod lm;
pub mod saturn_v;
pub mod ecs;
pub mod eps;
pub mod thermal;

pub use ecs::{
    EnvironmentalControlSystem,
    CabinAtmosphere,
    SuitCircuit,
    CabinFan,
    LiOHCanister,
    WaterManagement,
    OxygenSupply,
    CryoO2Tank,
    LifeSupportSystem,
    OxygenTank,
    Co2Scrubber,
    update_environmental_control_system,
};
pub use eps::{
    ElectricalSystem,
    FuelCell,
    Battery,
    Inverter,
    DcBus,
    AcBus,
    PowerDistribution,
    update_electrical_system,
};
pub use thermal::{
    ThermalControlSystem,
    GlycolLoop,
    SpaceRadiator,
    GlycolEvaporator,
    CabinTemperatureControl,
    ColdPlate,
    EquipmentCooling,
    update_thermal_control_system,
};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(csm::CsmSystemsPlugin)
            .add_plugins(lm::LmSystemsPlugin)
            .add_plugins(saturn_v::SaturnVSystemsPlugin);
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
