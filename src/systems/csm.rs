use bevy::prelude::*;
use super::*;

pub struct CsmSystemsPlugin;

impl Plugin for CsmSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_csm_systems)
            .add_systems(Update, update_csm_systems);
    }
}

#[derive(Component)]
pub struct CommandServiceModule {
    pub serial_number: String,
    pub mass_kg: f32,
    pub electrical: ElectricalSystem,
    pub sps: PropulsionSystem,
    pub rcs: RcsSystem,
    pub life_support: LifeSupportSystem,
    pub gnc: GuidanceNavigationControl,
}

#[derive(Debug, Clone)]
pub struct GuidanceNavigationControl {
    pub imu_aligned: bool,
    pub gimbal_angles: [f32; 3],
    pub accelerometer_bias: f32,
    pub star_tracker_aligned: bool,
    pub agc_memory_words: u16,
    pub standby_mode: bool,
}

impl Default for GuidanceNavigationControl {
    fn default() -> Self {
        Self {
            imu_aligned: false,
            gimbal_angles: [0.0, 0.0, 0.0],
            accelerometer_bias: 0.0,
            star_tracker_aligned: false,
            agc_memory_words: 2048,
            standby_mode: false,
        }
    }
}

fn setup_csm_systems(mut commands: Commands) {
    commands.spawn((
        CommandServiceModule {
            serial_number: "CSM-107".to_string(),
            mass_kg: 28801.0,
            electrical: ElectricalSystem {
                fuel_cells: vec![
                    FuelCell::new("FC1", 1.42),
                    FuelCell::new("FC2", 1.42),
                    FuelCell::new("FC3", 1.42),
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
            },
            sps: PropulsionSystem {
                engine: Engine {
                    name: "AJ10-137".to_string(),
                    thrust_sl_newtons: 0.0,
                    thrust_vac_newtons: 91000.0,
                    chamber_pressure_psi: 100.0,
                    mixture_ratio: 1.6,
                    status: SystemStatus::Nominal,
                },
                fuel_tanks: vec![
                    PropellantTank::new("SPS Ox", 18413.0, 29300.0),
                ],
                thrust_newtons: 0.0,
                isp_seconds: 314.0,
            },
            rcs: RcsSystem {
                quads: vec![
                    create_rcs_quad("A", true),
                    create_rcs_quad("B", true),
                    create_rcs_quad("C", true),
                    create_rcs_quad("D", true),
                ],
                propellant_kg: 118.0,
                pressure_psi: 270.0,
            },
            life_support: LifeSupportSystem {
                o2_tanks: vec![
                    OxygenTank {
                        id: "O2 Tank 1".to_string(),
                        pressure_psi: 900.0,
                        capacity_kg: 1.1,
                        remaining_kg: 1.1,
                        temp_c: -185.0,
                        status: SystemStatus::Nominal,
                    },
                    OxygenTank {
                        id: "O2 Tank 2".to_string(),
                        pressure_psi: 900.0,
                        capacity_kg: 1.1,
                        remaining_kg: 1.1,
                        temp_c: -185.0,
                        status: SystemStatus::Nominal,
                    },
                ],
                co2_scrubbers: vec![
                    Co2Scrubber {
                        id: "Primary".to_string(),
                        canister_hours: 12.0,
                        co2_ppm: 0.0,
                        status: SystemStatus::Nominal,
                    },
                    Co2Scrubber {
                        id: "Secondary".to_string(),
                        canister_hours: 12.0,
                        co2_ppm: 0.0,
                        status: SystemStatus::Nominal,
                    },
                ],
                cabin_pressure_psi: 5.0,
                cabin_temp_c: 21.0,
                water_tank_kg: 20.5,
            },
            gnc: GuidanceNavigationControl::default(),
        },
        Name::new("CSM-107 Systems"),
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

fn update_csm_systems(
    time: Res<Time>,
    mut csm_query: Query<&mut CommandServiceModule>,
) {
    let dt = time.delta_seconds();
    
    for mut csm in csm_query.iter_mut() {
        for fc in &mut csm.electrical.fuel_cells {
            if fc.status != SystemStatus::Failed {
                fc.temp_c = 200.0 + (fc.output_kw - 1.4) * 50.0;
                if fc.temp_c > 260.0 {
                    fc.status = SystemStatus::Warning;
                }
            }
        }
        
        csm.electrical.total_power_kw = csm.electrical.fuel_cells
            .iter()
            .filter(|fc| fc.status != SystemStatus::Failed)
            .map(|fc| fc.output_kw)
            .sum();
        
        for tank in &mut csm.life_support.o2_tanks {
            if tank.status != SystemStatus::Failed && tank.remaining_kg > 0.0 {
                tank.remaining_kg -= 0.0001 * dt;
                tank.pressure_psi = (tank.remaining_kg / tank.capacity_kg) * 900.0;
                if tank.pressure_psi < 100.0 {
                    tank.status = SystemStatus::Warning;
                }
            }
        }
        
        for scrubber in &mut csm.life_support.co2_scrubbers {
            if scrubber.status != SystemStatus::Failed {
                scrubber.co2_ppm += 10.0 * dt;
                if scrubber.co2_ppm > 7000.0 {
                    scrubber.status = SystemStatus::Warning;
                }
                scrubber.canister_hours -= dt / 3600.0;
            }
        }
    }
}

pub fn simulate_o2_tank_explosion(
    mut csm: &mut CommandServiceModule,
) {
    if let Some(tank) = csm.life_support.o2_tanks.iter_mut().find(|t| t.id == "O2 Tank 2") {
        tank.status = SystemStatus::Failed;
        tank.pressure_psi = 0.0;
        tank.remaining_kg = 0.0;
    }
    
    if let Some(fc) = csm.electrical.fuel_cells.iter_mut().find(|f| f.id == "FC1") {
        fc.status = SystemStatus::Failed;
        fc.output_kw = 0.0;
    }
    
    csm.electrical.main_bus_b = false;
    csm.electrical.total_power_kw = csm.electrical.fuel_cells
        .iter()
        .filter(|fc| fc.status != SystemStatus::Failed)
        .map(|fc| fc.output_kw)
        .sum();
}
