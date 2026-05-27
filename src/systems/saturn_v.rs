use bevy::prelude::*;
use super::*;

pub struct SaturnVSystemsPlugin;

impl Plugin for SaturnVSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_saturn_v_systems)
            .add_systems(Update, update_saturn_v_systems);
    }
}

#[derive(Component)]
pub struct SaturnVSystems {
    pub serial_number: String,
    pub s_ic: SIcStage,
    pub s_ii: SIIStage,
    pub s_ivb: SIVBStage,
    pub instrument_unit: InstrumentUnit,
    pub abort_system: LaunchEscapeSystem,
}

#[derive(Debug, Clone)]
pub struct SIcStage {
    pub engines: Vec<F1Engine>,
    pub propellant: PropellantTank,
    pub thrust_newtons: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct F1Engine {
    pub id: String,
    pub thrust_sl_newtons: f32,
    pub thrust_vac_newtons: f32,
    pub chamber_pressure_psi: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct SIIStage {
    pub engines: Vec<J2Engine>,
    pub propellant: PropellantTank,
    pub thrust_newtons: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct SIVBStage {
    pub engine: J2Engine,
    pub propellant: PropellantTank,
    pub thrust_newtons: f32,
    pub status: SystemStatus,
    pub restart_capability: bool,
}

#[derive(Debug, Clone)]
pub struct J2Engine {
    pub id: String,
    pub thrust_sl_newtons: f32,
    pub thrust_vac_newtons: f32,
    pub chamber_pressure_psi: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct InstrumentUnit {
    pub guidance_platform_stable: bool,
    pub status: SystemStatus,
}

#[derive(Debug, Clone)]
pub struct LaunchEscapeSystem {
    pub armed: bool,
    pub tower_jettisoned: bool,
    pub status: SystemStatus,
}

fn setup_saturn_v_systems(mut commands: Commands) {
    commands.spawn((
        SaturnVSystems {
            serial_number: "SA-506".to_string(),
            s_ic: SIcStage {
                engines: vec![
                    create_f1_engine("F-1 1", 6770000.0, 7770000.0),
                    create_f1_engine("F-1 2", 6770000.0, 7770000.0),
                    create_f1_engine("F-1 3", 6770000.0, 7770000.0),
                    create_f1_engine("F-1 4", 6770000.0, 7770000.0),
                    create_f1_engine("F-1 5", 6770000.0, 7770000.0),
                ],
                propellant: PropellantTank::new("S-IC Prop", 543000.0, 816000.0),
                thrust_newtons: 0.0,
                status: SystemStatus::Nominal,
            },
            s_ii: SIIStage {
                engines: vec![
                    create_j2_engine("J-2 1", 486000.0, 1033000.0),
                    create_j2_engine("J-2 2", 486000.0, 1033000.0),
                    create_j2_engine("J-2 3", 486000.0, 1033000.0),
                    create_j2_engine("J-2 4", 486000.0, 1033000.0),
                    create_j2_engine("J-2 5", 486000.0, 1033000.0),
                ],
                propellant: PropellantTank::new("S-II Prop", 108000.0, 362000.0),
                thrust_newtons: 0.0,
                status: SystemStatus::Nominal,
            },
            s_ivb: SIVBStage {
                engine: create_j2_engine("J-2 6", 0.0, 1033000.0),
                propellant: PropellantTank::new("S-IVB Prop", 23300.0, 73300.0),
                thrust_newtons: 0.0,
                status: SystemStatus::Nominal,
                restart_capability: true,
            },
            instrument_unit: InstrumentUnit {
                guidance_platform_stable: false,
                status: SystemStatus::Nominal,
            },
            abort_system: LaunchEscapeSystem {
                armed: true,
                tower_jettisoned: false,
                status: SystemStatus::Nominal,
            },
        },
        Name::new("SA-506 Systems"),
    ));
}

fn create_f1_engine(id: &str, thrust_sl: f32, thrust_vac: f32) -> F1Engine {
    F1Engine {
        id: id.to_string(),
        thrust_sl_newtons: thrust_sl,
        thrust_vac_newtons: thrust_vac,
        chamber_pressure_psi: 1010.0,
        status: SystemStatus::Nominal,
    }
}

fn create_j2_engine(id: &str, thrust_sl: f32, thrust_vac: f32) -> J2Engine {
    J2Engine {
        id: id.to_string(),
        thrust_sl_newtons: thrust_sl,
        thrust_vac_newtons: thrust_vac,
        chamber_pressure_psi: 763.0,
        status: SystemStatus::Nominal,
    }
}

fn update_saturn_v_systems(
    time: Res<Time>,
    mut saturn_query: Query<&mut SaturnVSystems>,
) {
    let dt = time.delta_seconds();
    
    for mut saturn in saturn_query.iter_mut() {
        if saturn.s_ic.status == SystemStatus::Nominal {
            saturn.s_ic.thrust_newtons = saturn.s_ic.engines
                .iter()
                .filter(|e| e.status != SystemStatus::Failed)
                .map(|e| e.thrust_sl_newtons)
                .sum();
            
            saturn.s_ic.propellant.fuel_kg -= 2330.0 * dt;
            saturn.s_ic.propellant.oxidizer_kg -= 3500.0 * dt;
            
            if saturn.s_ic.propellant.total_propellant_kg() <= 0.0 {
                saturn.s_ic.status = SystemStatus::Off;
                saturn.s_ic.thrust_newtons = 0.0;
            }
        }
        
        if saturn.s_ii.status == SystemStatus::Nominal {
            saturn.s_ii.thrust_newtons = saturn.s_ii.engines
                .iter()
                .filter(|e| e.status != SystemStatus::Failed)
                .map(|e| e.thrust_vac_newtons)
                .sum();
            
            saturn.s_ii.propellant.fuel_kg -= 460.0 * dt;
            saturn.s_ii.propellant.oxidizer_kg -= 1540.0 * dt;
            
            if saturn.s_ii.propellant.total_propellant_kg() <= 0.0 {
                saturn.s_ii.status = SystemStatus::Off;
                saturn.s_ii.thrust_newtons = 0.0;
            }
        }
        
        if saturn.s_ivb.status == SystemStatus::Nominal {
            saturn.s_ivb.thrust_newtons = if saturn.s_ivb.engine.status != SystemStatus::Failed {
                saturn.s_ivb.engine.thrust_vac_newtons
            } else {
                0.0
            };
            
            saturn.s_ivb.propellant.fuel_kg -= 100.0 * dt;
            saturn.s_ivb.propellant.oxidizer_kg -= 315.0 * dt;
            
            if saturn.s_ivb.propellant.total_propellant_kg() <= 0.0 {
                saturn.s_ivb.status = SystemStatus::Off;
                saturn.s_ivb.thrust_newtons = 0.0;
            }
        }
        
        if saturn.s_ic.status == SystemStatus::Off && !saturn.abort_system.tower_jettisoned {
            saturn.abort_system.tower_jettisoned = true;
        }
    }
}

pub fn ignite_s_ic(saturn: &mut SaturnVSystems) {
    saturn.s_ic.status = SystemStatus::Nominal;
    saturn.instrument_unit.guidance_platform_stable = true;
}

pub fn stage_separation(saturn: &mut SaturnVSystems, stage: u8) {
    match stage {
        1 => {
            saturn.s_ic.status = SystemStatus::Off;
            saturn.s_ic.thrust_newtons = 0.0;
        }
        2 => {
            saturn.s_ii.status = SystemStatus::Off;
            saturn.s_ii.thrust_newtons = 0.0;
        }
        3 => {
            saturn.s_ivb.status = SystemStatus::Off;
            saturn.s_ivb.thrust_newtons = 0.0;
        }
        _ => {}
    }
}
