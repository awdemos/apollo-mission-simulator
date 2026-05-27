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
    pub environmental_control: EnvironmentalControlSystem,
    pub thermal: ThermalControlSystem,
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
            electrical: ElectricalSystem::default(),
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
            environmental_control: EnvironmentalControlSystem::default(),
            thermal: ThermalControlSystem::default(),
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
        update_electrical_system(&mut csm.electrical, dt);
        let total_power = csm.electrical.total_power_kw;
        update_environmental_control_system(&mut csm.environmental_control, dt);
        update_thermal_control_system(&mut csm.thermal, dt, total_power);
        csm.rcs.propellant_kg = csm.rcs.propellant_kg.max(0.0);
    }
}

pub fn simulate_o2_tank_explosion(
    csm: &mut CommandServiceModule,
) {
    if let Some(tank) = csm.environmental_control.oxygen_supply.primary_tanks.iter_mut().find(|t| t.id == "O2 TANK 2") {
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
