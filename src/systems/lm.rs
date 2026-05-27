use bevy::prelude::*;
use super::*;

pub struct LmSystemsPlugin;

impl Plugin for LmSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lm_systems)
            .add_systems(Update, update_lm_systems);
    }
}

#[derive(Component)]
pub struct LunarModuleSystems {
    pub serial_number: String,
    pub mass_kg: f32,
    pub descent_stage: DescentStage,
    pub ascent_stage: AscentStage,
    pub rcs: RcsSystem,
    pub life_support: LifeSupportSystem,
    pub batteries: Vec<Battery>,
}

#[derive(Debug, Clone)]
pub struct DescentStage {
    pub dps: Engine,
    pub propellant: PropellantTank,
    pub landing_radar: LandingRadar,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct AscentStage {
    pub aps: Engine,
    pub propellant: PropellantTank,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct LandingRadar {
    pub altitude_m: f32,
    pub velocity_mps: f32,
    pub status: SystemStatus,
}

fn setup_lm_systems(mut commands: Commands) {
    commands.spawn((
        LunarModuleSystems {
            serial_number: "LM-5".to_string(),
            mass_kg: 15264.0,
            descent_stage: DescentStage {
                dps: Engine {
                    name: "TR-201".to_string(),
                    thrust_sl_newtons: 0.0,
                    thrust_vac_newtons: 45040.0,
                    chamber_pressure_psi: 120.0,
                    mixture_ratio: 1.6,
                    status: SystemStatus::Nominal,
                },
                propellant: PropellantTank::new("DPS Prop", 3911.0, 6235.0),
                landing_radar: LandingRadar {
                    altitude_m: 0.0,
                    velocity_mps: 0.0,
                    status: SystemStatus::Nominal,
                },
                status: SystemStatus::Nominal,
            },
            ascent_stage: AscentStage {
                aps: Engine {
                    name: "AJ10-137".to_string(),
                    thrust_sl_newtons: 0.0,
                    thrust_vac_newtons: 15600.0,
                    chamber_pressure_psi: 130.0,
                    mixture_ratio: 1.6,
                    status: SystemStatus::Nominal,
                },
                propellant: PropellantTank::new("APS Prop", 890.0, 1420.0),
                status: SystemStatus::Nominal,
            },
            rcs: RcsSystem {
                quads: vec![
                    create_rcs_quad("1", true),
                    create_rcs_quad("2", true),
                    create_rcs_quad("3", true),
                    create_rcs_quad("4", true),
                ],
                propellant_kg: 287.0,
                pressure_psi: 270.0,
            },
            life_support: LifeSupportSystem {
                o2_tanks: vec![
                    OxygenTank {
                        id: "LM O2".to_string(),
                        pressure_psi: 900.0,
                        capacity_kg: 0.5,
                        remaining_kg: 0.5,
                        temp_c: -185.0,
                        status: SystemStatus::Nominal,
                    },
                ],
                co2_scrubbers: vec![
                    Co2Scrubber {
                        id: "LM Primary".to_string(),
                        canister_hours: 12.0,
                        co2_ppm: 0.0,
                        status: SystemStatus::Nominal,
                    },
                    Co2Scrubber {
                        id: "LM Secondary".to_string(),
                        canister_hours: 12.0,
                        co2_ppm: 0.0,
                        status: SystemStatus::Nominal,
                    },
                ],
                cabin_pressure_psi: 5.0,
                cabin_temp_c: 21.0,
                water_tank_kg: 151.0,
            },
            batteries: vec![
                Battery::new("LM BAT 1", 415.0),
                Battery::new("LM BAT 2", 415.0),
                Battery::new("LM BAT 3", 415.0),
                Battery::new("LM BAT 4", 415.0),
            ],
        },
        Name::new("LM-5 Systems"),
    ));
}

fn create_rcs_quad(id: &str, enabled: bool) -> RcsQuad {
    RcsQuad {
        id: id.to_string(),
        thrusters: [
            RcsThruster { thrust_n: 445.0, status: SystemStatus::Nominal, fired_count: 0 },
            RcsThruster { thrust_n: 445.0, status: SystemStatus::Nominal, fired_count: 0 },
            RcsThruster { thrust_n: 445.0, status: SystemStatus::Nominal, fired_count: 0 },
            RcsThruster { thrust_n: 445.0, status: SystemStatus::Nominal, fired_count: 0 },
        ],
        enabled,
    }
}

fn update_lm_systems(
    time: Res<Time>,
    mut lm_query: Query<&mut LunarModuleSystems>,
) {
    let dt = time.delta_seconds();
    
    for mut lm in lm_query.iter_mut() {
        for battery in &mut lm.batteries {
            if battery.status != SystemStatus::Failed && battery.charge_pct > 0.0 {
                battery.charge_pct -= 0.001 * dt;
                if battery.charge_pct < 10.0 {
                    battery.status = SystemStatus::Warning;
                }
                if battery.charge_pct <= 0.0 {
                    battery.status = SystemStatus::Failed;
                }
            }
        }
        
        for tank in &mut lm.life_support.o2_tanks {
            if tank.status != SystemStatus::Failed && tank.remaining_kg > 0.0 {
                tank.remaining_kg -= 0.0001 * dt;
                tank.pressure_psi = (tank.remaining_kg / tank.capacity_kg) * 900.0;
            }
        }
        
        for scrubber in &mut lm.life_support.co2_scrubbers {
            if scrubber.status != SystemStatus::Failed {
                scrubber.co2_ppm += 10.0 * dt;
                scrubber.canister_hours -= dt / 3600.0;
            }
        }
        
        lm.descent_stage.propellant.fuel_kg -= 0.01 * dt;
        lm.descent_stage.propellant.oxidizer_kg -= 0.016 * dt;
    }
}
