use bevy::prelude::*;

pub struct FaultsPlugin;

impl Plugin for FaultsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FaultManager>()
            .init_resource::<MissionDifficulty>()
            .init_resource::<PlayerActions>()
            .add_event::<FaultTriggeredEvent>()
            .add_event::<FaultResolvedEvent>()
            .add_event::<FaultEscalatedEvent>()
            .add_systems(Update, (
                update_system_degradation,
                trigger_random_faults,
                process_scripted_faults,
                process_cascade_faults,
                check_fault_time_limits,
                update_communications_impact,
                generate_ground_control_advice,
                process_player_repair_actions,
                update_difficulty_progression,
            ).chain().run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Fault {
    pub id: FaultId,
    pub severity: FaultSeverity,
    pub category: FaultCategory,
    pub state: FaultState,
    pub triggered_at: f64,
    pub auto_resolve: bool,
    pub time_limit_seconds: Option<f64>,
    pub cascade_targets: Vec<FaultId>,
    pub repair_procedure: Option<RepairProcedure>,
    pub affected_systems: Vec<String>,
}

impl Fault {
    pub fn new(id: FaultId, severity: FaultSeverity, category: FaultCategory, triggered_at: f64) -> Self {
        let cascade_targets = id.default_cascade_targets();
        let repair_procedure = id.default_repair_procedure();
        let affected_systems = id.affected_systems();
        
        Self {
            id,
            severity,
            category,
            state: FaultState::Active,
            triggered_at,
            auto_resolve: false,
            time_limit_seconds: id.time_limit(),
            cascade_targets,
            repair_procedure,
            affected_systems,
        }
    }
    
    pub fn is_time_critical(&self) -> bool {
        self.time_limit_seconds.is_some()
    }
    
    pub fn time_remaining(&self, current_time: f64) -> Option<f64> {
        self.time_limit_seconds.map(|limit| {
            let elapsed = current_time - self.triggered_at;
            (limit - elapsed).max(0.0)
        })
    }
    
    pub fn is_expired(&self, current_time: f64) -> bool {
        match self.time_limit_seconds {
            Some(limit) => (current_time - self.triggered_at) >= limit,
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultId {
    // Electrical
    MainBusA_Failure,
    MainBusB_Failure,
    FuelCell1_Failure,
    FuelCell2_Failure,
    FuelCell3_Failure,
    BatteryDegradation,
    
    // Life Support
    O2Tank1_Rupture,
    O2Tank2_Rupture,
    Co2Scrubber_Saturation,
    CabinPressure_Loss,
    WaterTank_Leak,
    
    // Communications
    SBand_Amplifier_Failure,
    HighGainAntenna_Stuck,
    PhaseLock_Loss,
    CommSystem_Degradation,
    
    // Propulsion
    Sps_Engine_Failure,
    Rcs_QuadA_Failure,
    Rcs_QuadB_Failure,
    Rcs_QuadC_Failure,
    Rcs_QuadD_Failure,
    
    // GNC
    Imu_GimbalLock,
    StarTracker_Failure,
    Agc_ProgramAlarm,
    Accelerometer_Drift,
    
    // Structural/Thermal
    CryoStir_WiringFault,
    MicrometeoriteImpact,
    HeatShield_Crack,
    
    // Central Timing
    CentralTiming_ClockDrift,
    CentralTiming_SyncLoss,
    CentralTiming_DisplayFailure,
}

impl FaultId {
    pub fn name(&self) -> &'static str {
        match self {
            FaultId::MainBusA_Failure => "Main Bus A Failure",
            FaultId::MainBusB_Failure => "Main Bus B Failure",
            FaultId::FuelCell1_Failure => "Fuel Cell 1 Failure",
            FaultId::FuelCell2_Failure => "Fuel Cell 2 Failure",
            FaultId::FuelCell3_Failure => "Fuel Cell 3 Failure",
            FaultId::BatteryDegradation => "Battery Degradation",
            FaultId::O2Tank1_Rupture => "O2 Tank 1 Rupture",
            FaultId::O2Tank2_Rupture => "O2 Tank 2 Rupture",
            FaultId::Co2Scrubber_Saturation => "CO2 Scrubber Saturation",
            FaultId::CabinPressure_Loss => "Cabin Pressure Loss",
            FaultId::WaterTank_Leak => "Water Tank Leak",
            FaultId::SBand_Amplifier_Failure => "S-Band Amplifier Failure",
            FaultId::HighGainAntenna_Stuck => "High Gain Antenna Stuck",
            FaultId::PhaseLock_Loss => "Phase Lock Loss",
            FaultId::CommSystem_Degradation => "Communication System Degradation",
            FaultId::Sps_Engine_Failure => "SPS Engine Failure",
            FaultId::Rcs_QuadA_Failure => "RCS Quad A Failure",
            FaultId::Rcs_QuadB_Failure => "RCS Quad B Failure",
            FaultId::Rcs_QuadC_Failure => "RCS Quad C Failure",
            FaultId::Rcs_QuadD_Failure => "RCS Quad D Failure",
            FaultId::Imu_GimbalLock => "IMU Gimbal Lock",
            FaultId::StarTracker_Failure => "Star Tracker Failure",
            FaultId::Agc_ProgramAlarm => "AGC Program Alarm",
            FaultId::Accelerometer_Drift => "Accelerometer Drift",
            FaultId::CryoStir_WiringFault => "Cryo Stir Wiring Fault",
            FaultId::MicrometeoriteImpact => "Micrometeorite Impact",
            FaultId::HeatShield_Crack => "Heat Shield Crack",
            FaultId::CentralTiming_ClockDrift => "Central Timing Clock Drift",
            FaultId::CentralTiming_SyncLoss => "Central Timing Sync Loss",
            FaultId::CentralTiming_DisplayFailure => "Central Timing Display Failure",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            FaultId::MainBusA_Failure => "Main Bus A has lost power. Critical systems may be affected.",
            FaultId::MainBusB_Failure => "Main Bus B has lost power. Redundancy compromised.",
            FaultId::FuelCell1_Failure => "Fuel Cell 1 has failed. Power output reduced.",
            FaultId::FuelCell2_Failure => "Fuel Cell 2 has failed. Power output reduced.",
            FaultId::FuelCell3_Failure => "Fuel Cell 3 has failed. Critical power loss.",
            FaultId::BatteryDegradation => "Battery capacity is degrading. Monitor charge levels.",
            FaultId::O2Tank1_Rupture => "O2 Tank 1 has ruptured! Life support critical.",
            FaultId::O2Tank2_Rupture => "O2 Tank 2 has ruptured! Life support critical.",
            FaultId::Co2Scrubber_Saturation => "CO2 scrubbers are saturated. CO2 levels rising.",
            FaultId::CabinPressure_Loss => "Cabin pressure dropping. Seal integrity compromised.",
            FaultId::WaterTank_Leak => "Water tank is leaking. Water supply diminishing.",
            FaultId::SBand_Amplifier_Failure => "S-Band power amplifier has failed. Signal strength reduced.",
            FaultId::HighGainAntenna_Stuck => "High gain antenna is stuck. Communications degraded.",
            FaultId::PhaseLock_Loss => "Phase lock lost. Signal acquisition difficult.",
            FaultId::CommSystem_Degradation => "Communication system performance degraded.",
            FaultId::Sps_Engine_Failure => "SPS engine failure. Major propulsion lost.",
            FaultId::Rcs_QuadA_Failure => "RCS Quad A has failed. Attitude control compromised.",
            FaultId::Rcs_QuadB_Failure => "RCS Quad B has failed. Attitude control compromised.",
            FaultId::Rcs_QuadC_Failure => "RCS Quad C has failed. Attitude control compromised.",
            FaultId::Rcs_QuadD_Failure => "RCS Quad D has failed. Attitude control compromised.",
            FaultId::Imu_GimbalLock => "IMU approaching gimbal lock. Reorient platform.",
            FaultId::StarTracker_Failure => "Star tracker failure. Navigation accuracy reduced.",
            FaultId::Agc_ProgramAlarm => "AGC program alarm. Computer overload or error.",
            FaultId::Accelerometer_Drift => "Accelerometer drift detected. Guidance accuracy affected.",
            FaultId::CryoStir_WiringFault => "Cryogenic stir fan wiring fault. Tank pressure anomaly.",
            FaultId::MicrometeoriteImpact => "Micrometeorite impact detected. Hull damage possible.",
            FaultId::HeatShield_Crack => "Heat shield integrity compromised. Re-entry at risk.",
            FaultId::CentralTiming_ClockDrift => "Central timing clock drifting. Sync issues possible.",
            FaultId::CentralTiming_SyncLoss => "Central timing sync lost. System timing disrupted.",
            FaultId::CentralTiming_DisplayFailure => "Central timing display failed. MET reading unavailable.",
        }
    }
    
    pub fn category(&self) -> FaultCategory {
        match self {
            FaultId::MainBusA_Failure | FaultId::MainBusB_Failure |
            FaultId::FuelCell1_Failure | FaultId::FuelCell2_Failure |
            FaultId::FuelCell3_Failure | FaultId::BatteryDegradation => FaultCategory::Electrical,
            
            FaultId::O2Tank1_Rupture | FaultId::O2Tank2_Rupture |
            FaultId::Co2Scrubber_Saturation | FaultId::CabinPressure_Loss |
            FaultId::WaterTank_Leak => FaultCategory::LifeSupport,
            
            FaultId::SBand_Amplifier_Failure | FaultId::HighGainAntenna_Stuck |
            FaultId::PhaseLock_Loss | FaultId::CommSystem_Degradation => FaultCategory::Communications,
            
            FaultId::Sps_Engine_Failure | FaultId::Rcs_QuadA_Failure |
            FaultId::Rcs_QuadB_Failure | FaultId::Rcs_QuadC_Failure |
            FaultId::Rcs_QuadD_Failure => FaultCategory::Propulsion,
            
            FaultId::Imu_GimbalLock | FaultId::StarTracker_Failure |
            FaultId::Agc_ProgramAlarm | FaultId::Accelerometer_Drift => FaultCategory::Guidance,
            
            FaultId::CryoStir_WiringFault | FaultId::MicrometeoriteImpact |
            FaultId::HeatShield_Crack => FaultCategory::Structural,
            
            FaultId::CentralTiming_ClockDrift | FaultId::CentralTiming_SyncLoss |
            FaultId::CentralTiming_DisplayFailure => FaultCategory::Timing,
        }
    }
    
    pub fn default_severity(&self) -> FaultSeverity {
        match self {
            FaultId::MainBusA_Failure | FaultId::MainBusB_Failure => FaultSeverity::Critical,
            FaultId::FuelCell1_Failure | FaultId::FuelCell2_Failure => FaultSeverity::Major,
            FaultId::FuelCell3_Failure => FaultSeverity::Critical,
            FaultId::BatteryDegradation => FaultSeverity::Minor,
            FaultId::O2Tank1_Rupture | FaultId::O2Tank2_Rupture => FaultSeverity::Catastrophic,
            FaultId::Co2Scrubber_Saturation => FaultSeverity::Major,
            FaultId::CabinPressure_Loss => FaultSeverity::Critical,
            FaultId::WaterTank_Leak => FaultSeverity::Minor,
            FaultId::SBand_Amplifier_Failure => FaultSeverity::Major,
            FaultId::HighGainAntenna_Stuck => FaultSeverity::Major,
            FaultId::PhaseLock_Loss => FaultSeverity::Minor,
            FaultId::CommSystem_Degradation => FaultSeverity::Minor,
            FaultId::Sps_Engine_Failure => FaultSeverity::Critical,
            FaultId::Rcs_QuadA_Failure | FaultId::Rcs_QuadB_Failure |
            FaultId::Rcs_QuadC_Failure | FaultId::Rcs_QuadD_Failure => FaultSeverity::Major,
            FaultId::Imu_GimbalLock => FaultSeverity::Major,
            FaultId::StarTracker_Failure => FaultSeverity::Major,
            FaultId::Agc_ProgramAlarm => FaultSeverity::Major,
            FaultId::Accelerometer_Drift => FaultSeverity::Minor,
            FaultId::CryoStir_WiringFault => FaultSeverity::Critical,
            FaultId::MicrometeoriteImpact => FaultSeverity::Major,
            FaultId::HeatShield_Crack => FaultSeverity::Catastrophic,
            FaultId::CentralTiming_ClockDrift => FaultSeverity::Minor,
            FaultId::CentralTiming_SyncLoss => FaultSeverity::Major,
            FaultId::CentralTiming_DisplayFailure => FaultSeverity::Minor,
        }
    }
    
    pub fn time_limit(&self) -> Option<f64> {
        match self {
            FaultId::O2Tank1_Rupture | FaultId::O2Tank2_Rupture => Some(3600.0),
            FaultId::CabinPressure_Loss => Some(1800.0),
            FaultId::Co2Scrubber_Saturation => Some(7200.0),
            FaultId::HeatShield_Crack => Some(86400.0),
            _ => None,
        }
    }
    
    pub fn default_cascade_targets(&self) -> Vec<FaultId> {
        match self {
            FaultId::MainBusA_Failure => vec![FaultId::CommSystem_Degradation],
            FaultId::MainBusB_Failure => vec![FaultId::FuelCell1_Failure, FaultId::CommSystem_Degradation],
            FaultId::FuelCell1_Failure => vec![FaultId::BatteryDegradation],
            FaultId::FuelCell2_Failure => vec![FaultId::BatteryDegradation],
            FaultId::FuelCell3_Failure => vec![FaultId::BatteryDegradation, FaultId::Sps_Engine_Failure],
            FaultId::BatteryDegradation => vec![FaultId::CommSystem_Degradation],
            FaultId::O2Tank1_Rupture => vec![FaultId::CabinPressure_Loss],
            FaultId::O2Tank2_Rupture => vec![FaultId::MainBusB_Failure, FaultId::CabinPressure_Loss],
            FaultId::CabinPressure_Loss => vec![FaultId::Co2Scrubber_Saturation],
            FaultId::SBand_Amplifier_Failure => vec![FaultId::CommSystem_Degradation],
            FaultId::PhaseLock_Loss => vec![FaultId::CommSystem_Degradation],
            FaultId::Sps_Engine_Failure => vec![],
            FaultId::Rcs_QuadA_Failure | FaultId::Rcs_QuadB_Failure |
            FaultId::Rcs_QuadC_Failure | FaultId::Rcs_QuadD_Failure => vec![],
            FaultId::Imu_GimbalLock => vec![FaultId::Agc_ProgramAlarm],
            FaultId::StarTracker_Failure => vec![FaultId::Accelerometer_Drift],
            FaultId::Agc_ProgramAlarm => vec![],
            FaultId::CryoStir_WiringFault => vec![FaultId::O2Tank2_Rupture],
            FaultId::MicrometeoriteImpact => vec![FaultId::CabinPressure_Loss, FaultId::CommSystem_Degradation],
            FaultId::CentralTiming_SyncLoss => vec![FaultId::Agc_ProgramAlarm],
            _ => vec![],
        }
    }
    
    pub fn affected_systems(&self) -> Vec<String> {
        match self {
            FaultId::MainBusA_Failure => vec!["Electrical".to_string(), "Communications".to_string()],
            FaultId::MainBusB_Failure => vec!["Electrical".to_string(), "Fuel Cell 1".to_string()],
            FaultId::FuelCell1_Failure | FaultId::FuelCell2_Failure | FaultId::FuelCell3_Failure => {
                vec!["Electrical".to_string(), "Power".to_string()]
            }
            FaultId::BatteryDegradation => vec!["Electrical".to_string()],
            FaultId::O2Tank1_Rupture | FaultId::O2Tank2_Rupture => {
                vec!["Life Support".to_string(), "Oxygen".to_string()]
            }
            FaultId::Co2Scrubber_Saturation => vec!["Life Support".to_string(), "CO2 Removal".to_string()],
            FaultId::CabinPressure_Loss => vec!["Life Support".to_string(), "Pressure".to_string()],
            FaultId::WaterTank_Leak => vec!["Life Support".to_string(), "Water".to_string()],
            FaultId::SBand_Amplifier_Failure | FaultId::HighGainAntenna_Stuck |
            FaultId::PhaseLock_Loss | FaultId::CommSystem_Degradation => {
                vec!["Communications".to_string()]
            }
            FaultId::Sps_Engine_Failure => vec!["Propulsion".to_string(), "SPS".to_string()],
            FaultId::Rcs_QuadA_Failure | FaultId::Rcs_QuadB_Failure |
            FaultId::Rcs_QuadC_Failure | FaultId::Rcs_QuadD_Failure => {
                vec!["Propulsion".to_string(), "RCS".to_string()]
            }
            FaultId::Imu_GimbalLock | FaultId::Accelerometer_Drift => {
                vec!["GNC".to_string(), "IMU".to_string()]
            }
            FaultId::StarTracker_Failure => vec!["GNC".to_string(), "Navigation".to_string()],
            FaultId::Agc_ProgramAlarm => vec!["GNC".to_string(), "AGC".to_string()],
            FaultId::CryoStir_WiringFault => vec!["Cryogenics".to_string(), "O2 Tanks".to_string()],
            FaultId::MicrometeoriteImpact => vec!["Hull".to_string(), "Structure".to_string()],
            FaultId::HeatShield_Crack => vec!["Thermal Protection".to_string()],
            FaultId::CentralTiming_ClockDrift | FaultId::CentralTiming_SyncLoss |
            FaultId::CentralTiming_DisplayFailure => {
                vec!["Timing".to_string(), "MET".to_string()]
            }
        }
    }
    
    pub fn default_repair_procedure(&self) -> Option<RepairProcedure> {
        match self {
            FaultId::MainBusA_Failure => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Check circuit breakers for Main Bus A".to_string(),
                        action: RepairAction::FlipSwitch("CB MAIN BUS A".to_string()),
                        diagram_reference: Some("ELECTRICAL_SCHEMATIC_1".to_string()),
                    },
                    RepairStep {
                        description: "Verify fuel cell connections to Bus A".to_string(),
                        action: RepairAction::ReroutePower("FC2".to_string(), "MAIN_BUS_A".to_string()),
                        diagram_reference: Some("ELECTRICAL_SCHEMATIC_2".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(600.0),
                can_perform_without_comm: true,
            }),
            FaultId::MainBusB_Failure => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Check circuit breakers for Main Bus B".to_string(),
                        action: RepairAction::FlipSwitch("CB MAIN BUS B".to_string()),
                        diagram_reference: Some("ELECTRICAL_SCHEMATIC_1".to_string()),
                    },
                    RepairStep {
                        description: "Reroute FC3 to Bus B".to_string(),
                        action: RepairAction::ReroutePower("FC3".to_string(), "MAIN_BUS_B".to_string()),
                        diagram_reference: Some("ELECTRICAL_SCHEMATIC_2".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(600.0),
                can_perform_without_comm: true,
            }),
            FaultId::FuelCell1_Failure => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Attempt FC1 restart procedure".to_string(),
                        action: RepairAction::FlipSwitch("FC1 PURGE".to_string()),
                        diagram_reference: None,
                    },
                    RepairStep {
                        description: "If restart fails, isolate FC1".to_string(),
                        action: RepairAction::FlipSwitch("CB FUEL CELL 1".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![],
                time_limit: Some(300.0),
                can_perform_without_comm: true,
            }),
            FaultId::O2Tank1_Rupture | FaultId::O2Tank2_Rupture => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Close O2 tank isolation valve".to_string(),
                        action: RepairAction::FlipSwitch("O2 TANK ISOLATION".to_string()),
                        diagram_reference: Some("LIFE_SUPPORT_SCHEMATIC".to_string()),
                    },
                    RepairStep {
                        description: "Switch to remaining O2 tank".to_string(),
                        action: RepairAction::ReroutePower("O2_TANK_2".to_string(), "CABIN_SUPPLY".to_string()),
                        diagram_reference: Some("LIFE_SUPPORT_SCHEMATIC".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(180.0),
                can_perform_without_comm: true,
            }),
            FaultId::Co2Scrubber_Saturation => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Replace CO2 canister".to_string(),
                        action: RepairAction::ReplaceComponent("CO2_CANISTER".to_string()),
                        diagram_reference: Some("LIFE_SUPPORT_SCHEMATIC".to_string()),
                    },
                    RepairStep {
                        description: "Verify CO2 levels dropping".to_string(),
                        action: RepairAction::ManualOverride("CO2_MONITOR".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![Tool::ReplacementCanister],
                time_limit: Some(1200.0),
                can_perform_without_comm: true,
            }),
            FaultId::CabinPressure_Loss => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Identify pressure leak source".to_string(),
                        action: RepairAction::ManualOverride("PRESSURE_MONITOR".to_string()),
                        diagram_reference: None,
                    },
                    RepairStep {
                        description: "Seal leak or isolate affected section".to_string(),
                        action: RepairAction::JuryRig("PRESSURE_SEAL".to_string()),
                        diagram_reference: Some("HULL_DIAGRAM".to_string()),
                    },
                ],
                required_tools: vec![Tool::SealantKit],
                time_limit: Some(300.0),
                can_perform_without_comm: true,
            }),
            FaultId::SBand_Amplifier_Failure => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Switch to backup S-Band amplifier".to_string(),
                        action: RepairAction::FlipSwitch("SBAND AMP BACKUP".to_string()),
                        diagram_reference: Some("COMM_SCHEMATIC".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(600.0),
                can_perform_without_comm: true,
            }),
            FaultId::HighGainAntenna_Stuck => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Attempt antenna unstow sequence".to_string(),
                        action: RepairAction::EnterVerbNoun(41, 1),
                        diagram_reference: Some("ANTENNA_PROCEDURE".to_string()),
                    },
                    RepairStep {
                        description: "If stuck, switch to omnidirectional".to_string(),
                        action: RepairAction::FlipSwitch("OMNI ANTENNA SELECT".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![],
                time_limit: Some(900.0),
                can_perform_without_comm: true,
            }),
            FaultId::Sps_Engine_Failure => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Check SPS engine circuit breakers".to_string(),
                        action: RepairAction::FlipSwitch("CB SPS ENGINE".to_string()),
                        diagram_reference: Some("PROPULSION_SCHEMATIC".to_string()),
                    },
                    RepairStep {
                        description: "Verify propellant valve positions".to_string(),
                        action: RepairAction::ManualOverride("SPS_VALVES".to_string()),
                        diagram_reference: Some("PROPULSION_SCHEMATIC".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(1200.0),
                can_perform_without_comm: true,
            }),
            FaultId::Imu_GimbalLock => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Enter VERB 41 NOUN 1 to coarse align".to_string(),
                        action: RepairAction::EnterVerbNoun(41, 1),
                        diagram_reference: Some("AGC_PROCEDURE".to_string()),
                    },
                    RepairStep {
                        description: "Manually slew away from gimbal lock orientation".to_string(),
                        action: RepairAction::ManualOverride("IMU_SLEW".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![],
                time_limit: Some(300.0),
                can_perform_without_comm: true,
            }),
            FaultId::Agc_ProgramAlarm => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Note alarm code on DSKY".to_string(),
                        action: RepairAction::ManualOverride("DSKY_READOUT".to_string()),
                        diagram_reference: None,
                    },
                    RepairStep {
                        description: "Enter VERB 5 NOUN 9 to monitor alarm".to_string(),
                        action: RepairAction::EnterVerbNoun(5, 9),
                        diagram_reference: Some("AGC_PROCEDURE".to_string()),
                    },
                    RepairStep {
                        description: "If 1201/1202: priority scheduling overflow".to_string(),
                        action: RepairAction::EnterVerbNoun(37, 0),
                        diagram_reference: Some("AGC_PROCEDURE".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(120.0),
                can_perform_without_comm: true,
            }),
            FaultId::CryoStir_WiringFault => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "STOP all cryo stir operations immediately".to_string(),
                        action: RepairAction::FlipSwitch("CRYO STIR OFF".to_string()),
                        diagram_reference: Some("CRYO_SCHEMATIC".to_string()),
                    },
                    RepairStep {
                        description: "Monitor tank pressures manually".to_string(),
                        action: RepairAction::ManualOverride("CRYO_PRESSURE_MONITOR".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![],
                time_limit: Some(60.0),
                can_perform_without_comm: true,
            }),
            FaultId::CentralTiming_ClockDrift => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Resync timing unit with AGC".to_string(),
                        action: RepairAction::EnterVerbNoun(16, 36),
                        diagram_reference: Some("TIMING_PROCEDURE".to_string()),
                    },
                ],
                required_tools: vec![],
                time_limit: Some(300.0),
                can_perform_without_comm: true,
            }),
            FaultId::CentralTiming_SyncLoss => Some(RepairProcedure {
                fault_id: *self,
                steps: vec![
                    RepairStep {
                        description: "Restart timing sync sequence".to_string(),
                        action: RepairAction::FlipSwitch("CTE SYNC RESET".to_string()),
                        diagram_reference: Some("TIMING_PROCEDURE".to_string()),
                    },
                    RepairStep {
                        description: "Verify MET display updating".to_string(),
                        action: RepairAction::ManualOverride("MET_DISPLAY".to_string()),
                        diagram_reference: None,
                    },
                ],
                required_tools: vec![],
                time_limit: Some(600.0),
                can_perform_without_comm: true,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultSeverity {
    Minor,
    Major,
    Critical,
    Catastrophic,
}

impl FaultSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultSeverity::Minor => "MINOR",
            FaultSeverity::Major => "MAJOR",
            FaultSeverity::Critical => "CRITICAL",
            FaultSeverity::Catastrophic => "CATASTROPHIC",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            FaultSeverity::Minor => Color::srgb(1.0, 0.8, 0.0),
            FaultSeverity::Major => Color::srgb(1.0, 0.5, 0.0),
            FaultSeverity::Critical => Color::srgb(1.0, 0.0, 0.0),
            FaultSeverity::Catastrophic => Color::srgb(0.8, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultCategory {
    Electrical,
    LifeSupport,
    Communications,
    Propulsion,
    Guidance,
    Structural,
    Timing,
}

impl FaultCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultCategory::Electrical => "ELECTRICAL",
            FaultCategory::LifeSupport => "LIFE SUPPORT",
            FaultCategory::Communications => "COMMUNICATIONS",
            FaultCategory::Propulsion => "PROPULSION",
            FaultCategory::Guidance => "GNC",
            FaultCategory::Structural => "STRUCTURAL",
            FaultCategory::Timing => "TIMING",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultState {
    Dormant,
    Active,
    Contained,
    Resolved,
    Escalated,
}

impl FaultState {
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultState::Dormant => "DORMANT",
            FaultState::Active => "ACTIVE",
            FaultState::Contained => "CONTAINED",
            FaultState::Resolved => "RESOLVED",
            FaultState::Escalated => "ESCALATED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FaultRecord {
    pub fault_id: FaultId,
    pub triggered_at: f64,
    pub resolved_at: Option<f64>,
    pub final_state: FaultState,
    pub escalation_count: u32,
}

#[derive(Debug, Clone)]
pub struct RepairProcedure {
    pub fault_id: FaultId,
    pub steps: Vec<RepairStep>,
    pub required_tools: Vec<Tool>,
    pub time_limit: Option<f64>,
    pub can_perform_without_comm: bool,
}

#[derive(Debug, Clone)]
pub struct RepairStep {
    pub description: String,
    pub action: RepairAction,
    pub diagram_reference: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RepairAction {
    FlipSwitch(String),
    EnterVerbNoun(u8, u8),
    ReplaceComponent(String),
    ReroutePower(String, String),
    ManualOverride(String),
    ConnectHose(String),
    JuryRig(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    ReplacementCanister,
    SealantKit,
    WiringKit,
    Multimeter,
}

#[derive(Resource)]
pub struct FaultManager {
    pub active_faults: Vec<Fault>,
    pub fault_history: Vec<FaultRecord>,
    pub difficulty_level: u32,
    pub random_seed: u64,
    pub next_random_fault_time: f64,
    pub scripted_faults: Vec<(f64, FaultId)>,
    pub cascade_probability: f32,
    pub fault_probability_per_minute: f32,
}

impl Default for FaultManager {
    fn default() -> Self {
        Self {
            active_faults: Vec::new(),
            fault_history: Vec::new(),
            difficulty_level: 1,
            random_seed: 12345,
            next_random_fault_time: 300.0,
            scripted_faults: Vec::new(),
            cascade_probability: 0.3,
            fault_probability_per_minute: 0.01,
        }
    }
}

impl FaultManager {
    pub fn trigger_fault(&mut self, fault_id: FaultId, mission_time: f64) {
        if self.has_active_fault(fault_id) {
            return;
        }
        
        let fault = Fault::new(
            fault_id,
            fault_id.default_severity(),
            fault_id.category(),
            mission_time,
        );
        
        self.active_faults.push(fault);
        self.fault_history.push(FaultRecord {
            fault_id,
            triggered_at: mission_time,
            resolved_at: None,
            final_state: FaultState::Active,
            escalation_count: 0,
        });
    }
    
    pub fn resolve_fault(&mut self, fault_id: FaultId, mission_time: f64) {
        if let Some(fault) = self.active_faults.iter_mut().find(|f| f.id == fault_id) {
            fault.state = FaultState::Resolved;
        }
        
        if let Some(record) = self.fault_history.iter_mut().find(|r| {
            r.fault_id == fault_id && r.resolved_at.is_none()
        }) {
            record.resolved_at = Some(mission_time);
            record.final_state = FaultState::Resolved;
        }
        
        self.active_faults.retain(|f| f.id != fault_id);
    }
    
    pub fn escalate_fault(&mut self, fault_id: FaultId) {
        if let Some(fault) = self.active_faults.iter_mut().find(|f| f.id == fault_id) {
            fault.state = FaultState::Escalated;
        }
        
        if let Some(record) = self.fault_history.iter_mut().find(|r| {
            r.fault_id == fault_id && r.resolved_at.is_none()
        }) {
            record.escalation_count += 1;
            record.final_state = FaultState::Escalated;
        }
    }
    
    pub fn has_active_fault(&self, fault_id: FaultId) -> bool {
        self.active_faults.iter().any(|f| f.id == fault_id)
    }
    
    pub fn get_active_faults_by_category(&self, category: FaultCategory) -> Vec<&Fault> {
        self.active_faults.iter()
            .filter(|f| f.category == category)
            .collect()
    }
    
    pub fn get_critical_faults(&self) -> Vec<&Fault> {
        self.active_faults.iter()
            .filter(|f| matches!(f.severity, FaultSeverity::Critical | FaultSeverity::Catastrophic))
            .collect()
    }
    
    pub fn load_scripted_faults(&mut self, mission_id: &str) {
        self.scripted_faults = match mission_id {
            "apollo13" => vec![
                (198_000.0, FaultId::CryoStir_WiringFault),
                (198_060.0, FaultId::O2Tank2_Rupture),
                (198_120.0, FaultId::MainBusB_Failure),
                (198_180.0, FaultId::FuelCell1_Failure),
            ],
            _ => vec![],
        };
    }
    
    pub fn set_difficulty(&mut self, level: u32) {
        self.difficulty_level = level;
        self.fault_probability_per_minute = 0.005 + (level as f32 * 0.005);
        self.cascade_probability = 0.1 + (level as f32 * 0.1).min(0.8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultTrigger {
    Random,
    Scripted(f64),
    Cascade(FaultId),
    PlayerAction,
    ExternalEvent,
}

#[derive(Resource)]
pub struct MissionDifficulty {
    pub phase: DifficultyPhase,
    pub fault_probability: f32,
    pub max_concurrent_faults: usize,
    pub cascade_probability: f32,
    pub comm_reliability: f32,
    pub time_pressure_multiplier: f32,
    pub phase_start_time: f64,
}

impl Default for MissionDifficulty {
    fn default() -> Self {
        Self {
            phase: DifficultyPhase::NormalFlight,
            fault_probability: 0.0,
            max_concurrent_faults: 1,
            cascade_probability: 0.1,
            comm_reliability: 1.0,
            time_pressure_multiplier: 1.0,
            phase_start_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyPhase {
    NormalFlight,
    MinorAnomalies,
    SystemDegradation,
    CrisisMode,
    SurvivalMode,
    Unsurvivable,
}

impl DifficultyPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            DifficultyPhase::NormalFlight => "NORMAL FLIGHT",
            DifficultyPhase::MinorAnomalies => "MINOR ANOMALIES",
            DifficultyPhase::SystemDegradation => "SYSTEM DEGRADATION",
            DifficultyPhase::CrisisMode => "CRISIS MODE",
            DifficultyPhase::SurvivalMode => "SURVIVAL MODE",
            DifficultyPhase::Unsurvivable => "UNSURVIVABLE",
        }
    }
    
    pub fn next(&self) -> Option<Self> {
        match self {
            DifficultyPhase::NormalFlight => Some(DifficultyPhase::MinorAnomalies),
            DifficultyPhase::MinorAnomalies => Some(DifficultyPhase::SystemDegradation),
            DifficultyPhase::SystemDegradation => Some(DifficultyPhase::CrisisMode),
            DifficultyPhase::CrisisMode => Some(DifficultyPhase::SurvivalMode),
            DifficultyPhase::SurvivalMode => Some(DifficultyPhase::Unsurvivable),
            DifficultyPhase::Unsurvivable => None,
        }
    }
}

#[derive(Resource)]
pub struct PlayerActions {
    pub circuit_breakers_flipped: Vec<String>,
    pub agc_commands_entered: Vec<String>,
    pub systems_powered_down: Vec<String>,
    pub manual_overrides: Vec<String>,
    pub repairs_attempted: Vec<FaultId>,
    pub repairs_successful: Vec<FaultId>,
    pub current_repair_target: Option<FaultId>,
    pub current_repair_step: usize,
}

impl Default for PlayerActions {
    fn default() -> Self {
        Self {
            circuit_breakers_flipped: Vec::new(),
            agc_commands_entered: Vec::new(),
            systems_powered_down: Vec::new(),
            manual_overrides: Vec::new(),
            repairs_attempted: Vec::new(),
            repairs_successful: Vec::new(),
            current_repair_target: None,
            current_repair_step: 0,
        }
    }
}

#[derive(Event)]
pub struct FaultTriggeredEvent {
    pub fault_id: FaultId,
    pub severity: FaultSeverity,
    pub category: FaultCategory,
    pub trigger: FaultTrigger,
}

#[derive(Event)]
pub struct FaultResolvedEvent {
    pub fault_id: FaultId,
    pub resolution_time: f64,
}

#[derive(Event)]
pub struct FaultEscalatedEvent {
    pub fault_id: FaultId,
    pub new_severity: FaultSeverity,
    pub cascade_triggered: Vec<FaultId>,
}

pub enum FaultAssistanceLevel {
    FullGuidance,
    PartialGuidance,
    DiagnosticsOnly,
    NoAssistance,
}

// System functions
fn update_system_degradation(
    mut fault_manager: ResMut<FaultManager>,
    time: Res<Time>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    fault_manager.next_random_fault_time -= time.delta_seconds_f64();
}

fn trigger_random_faults(
    mut fault_manager: ResMut<FaultManager>,
    mut events: EventWriter<FaultTriggeredEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    if fault_manager.next_random_fault_time <= 0.0 {
        fault_manager.next_random_fault_time = 60.0;
        
        let probability = fault_manager.fault_probability_per_minute;
        let roll = rand::random::<f32>();
        
        if roll < probability && fault_manager.active_faults.len() < 5 {
            let possible_faults = vec![
                FaultId::BatteryDegradation,
                FaultId::PhaseLock_Loss,
                FaultId::CommSystem_Degradation,
                FaultId::Accelerometer_Drift,
                FaultId::CentralTiming_ClockDrift,
            ];
            
            if let Some(&fault_id) = possible_faults.get(rand::random::<usize>() % possible_faults.len()) {
                if !fault_manager.has_active_fault(fault_id) {
                    fault_manager.trigger_fault(fault_id, mission_state.mission_time);
                    events.send(FaultTriggeredEvent {
                        fault_id,
                        severity: fault_id.default_severity(),
                        category: fault_id.category(),
                        trigger: FaultTrigger::Random,
                    });
                }
            }
        }
    }
}

fn process_scripted_faults(
    mut fault_manager: ResMut<FaultManager>,
    mut events: EventWriter<FaultTriggeredEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    let current_time = mission_state.mission_time;
    let mut triggered = Vec::new();
    
    for (time, fault_id) in &fault_manager.scripted_faults {
        if current_time >= *time && !fault_manager.has_active_fault(*fault_id) {
            triggered.push(*fault_id);
        }
    }
    
    for fault_id in triggered {
        fault_manager.trigger_fault(fault_id, current_time);
        events.send(FaultTriggeredEvent {
            fault_id,
            severity: fault_id.default_severity(),
            category: fault_id.category(),
            trigger: FaultTrigger::Scripted(current_time),
        });
    }
}

fn process_cascade_faults(
    mut fault_manager: ResMut<FaultManager>,
    mut events: EventWriter<FaultEscalatedEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    let current_time = mission_state.mission_time;
    let active_faults: Vec<Fault> = fault_manager.active_faults.clone();
    
    for fault in active_faults {
        if fault.state == FaultState::Active || fault.state == FaultState::Escalated {
            for target_id in &fault.cascade_targets {
                if !fault_manager.has_active_fault(*target_id) {
                    let cascade_roll = rand::random::<f32>();
                    if cascade_roll < fault_manager.cascade_probability {
                        fault_manager.trigger_fault(*target_id, current_time);
                        fault_manager.escalate_fault(fault.id);
                        
                        events.send(FaultEscalatedEvent {
                            fault_id: fault.id,
                            new_severity: fault.severity,
                            cascade_triggered: vec![*target_id],
                        });
                    }
                }
            }
        }
    }
}

fn check_fault_time_limits(
    mut fault_manager: ResMut<FaultManager>,
    mut events: EventWriter<FaultEscalatedEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    let current_time = mission_state.mission_time;
    let mut expired_faults = Vec::new();
    
    for fault in &fault_manager.active_faults {
        if fault.is_expired(current_time) && fault.state == FaultState::Active {
            expired_faults.push(fault.id);
        }
    }
    
    for fault_id in expired_faults {
        fault_manager.escalate_fault(fault_id);
        
        events.send(FaultEscalatedEvent {
            fault_id,
            new_severity: FaultSeverity::Critical,
            cascade_triggered: vec![],
        });
    }
}

fn update_communications_impact(
    fault_manager: Res<FaultManager>,
    mut comms: ResMut<crate::communications::CommunicationsBus>,
) {
    let comm_faults = fault_manager.get_active_faults_by_category(FaultCategory::Communications);
    let elec_faults = fault_manager.get_active_faults_by_category(FaultCategory::Electrical);
    
    let comm_impact = comm_faults.len() as f32 * 0.2;
    let power_impact = elec_faults.len() as f32 * 0.1;
    
    comms.signal_strength = (comms.signal_strength - comm_impact - power_impact).clamp(0.0, 1.0);
    
    if comms.signal_strength < 0.3 {
        comms.status = crate::communications::CommStatus::Critical;
    } else if comms.signal_strength < 0.6 {
        comms.status = crate::communications::CommStatus::Degraded;
    }
}

fn generate_ground_control_advice(
    fault_manager: Res<FaultManager>,
    comms: Res<crate::communications::CommunicationsBus>,
    _ground: ResMut<crate::communications::GroundControlState>,
) {
    let _assistance_level = if comms.signal_strength < 0.1 {
        FaultAssistanceLevel::NoAssistance
    } else if comms.signal_strength < 0.3 {
        FaultAssistanceLevel::DiagnosticsOnly
    } else if comms.signal_strength < 0.6 {
        FaultAssistanceLevel::PartialGuidance
    } else {
        FaultAssistanceLevel::FullGuidance
    };
    
    let critical_faults = fault_manager.get_critical_faults();
    if !critical_faults.is_empty() && comms.signal_strength > 0.1 {
        // Ground control would notice critical faults
    }
}

fn process_player_repair_actions(
    mut player_actions: ResMut<PlayerActions>,
    mut fault_manager: ResMut<FaultManager>,
    mut events: EventWriter<FaultResolvedEvent>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if let Some(target_id) = player_actions.current_repair_target {
        if let Some(fault) = fault_manager.active_faults.iter().find(|f| f.id == target_id) {
            if let Some(procedure) = &fault.repair_procedure {
                if player_actions.current_repair_step >= procedure.steps.len() {
                    fault_manager.resolve_fault(target_id, mission_state.mission_time);
                    player_actions.repairs_successful.push(target_id);
                    player_actions.current_repair_target = None;
                    player_actions.current_repair_step = 0;
                    
                    events.send(FaultResolvedEvent {
                        fault_id: target_id,
                        resolution_time: mission_state.mission_time,
                    });
                }
            }
        }
    }
}

fn update_difficulty_progression(
    mut difficulty: ResMut<MissionDifficulty>,
    fault_manager: Res<FaultManager>,
    mission_state: Res<crate::mission::MissionState>,
) {
    if !mission_state.is_running {
        return;
    }
    
    let active_count = fault_manager.active_faults.len();
    let critical_count = fault_manager.get_critical_faults().len();
    
    let new_phase = match (active_count, critical_count) {
        (0, 0) => DifficultyPhase::NormalFlight,
        (1..=2, 0) => DifficultyPhase::MinorAnomalies,
        (3..=5, 0) | (1..=2, 1) => DifficultyPhase::SystemDegradation,
        (6.., _) | (3.., 1) | (0, 1) | (_, 2) => DifficultyPhase::CrisisMode,
        (_, 3..) => DifficultyPhase::SurvivalMode,
    };
    
    if new_phase != difficulty.phase {
        difficulty.phase = new_phase;
        difficulty.phase_start_time = mission_state.mission_time;
        
        match difficulty.phase {
            DifficultyPhase::NormalFlight => {
                difficulty.fault_probability = 0.0;
                difficulty.max_concurrent_faults = 1;
                difficulty.comm_reliability = 1.0;
            }
            DifficultyPhase::MinorAnomalies => {
                difficulty.fault_probability = 0.01;
                difficulty.max_concurrent_faults = 2;
                difficulty.comm_reliability = 0.95;
            }
            DifficultyPhase::SystemDegradation => {
                difficulty.fault_probability = 0.03;
                difficulty.max_concurrent_faults = 4;
                difficulty.comm_reliability = 0.8;
                difficulty.cascade_probability = 0.3;
            }
            DifficultyPhase::CrisisMode => {
                difficulty.fault_probability = 0.05;
                difficulty.max_concurrent_faults = 6;
                difficulty.comm_reliability = 0.5;
                difficulty.cascade_probability = 0.5;
                difficulty.time_pressure_multiplier = 1.5;
            }
            DifficultyPhase::SurvivalMode => {
                difficulty.fault_probability = 0.08;
                difficulty.max_concurrent_faults = 8;
                difficulty.comm_reliability = 0.2;
                difficulty.cascade_probability = 0.7;
                difficulty.time_pressure_multiplier = 2.0;
            }
            DifficultyPhase::Unsurvivable => {
                difficulty.fault_probability = 0.1;
                difficulty.max_concurrent_faults = 10;
                difficulty.comm_reliability = 0.0;
                difficulty.cascade_probability = 0.9;
                difficulty.time_pressure_multiplier = 3.0;
            }
        }
    }
}

pub fn get_fault_assistance_level(comms: &crate::communications::CommunicationsBus) -> FaultAssistanceLevel {
    if comms.signal_strength < 0.1 {
        FaultAssistanceLevel::NoAssistance
    } else if comms.signal_strength < 0.3 {
        FaultAssistanceLevel::DiagnosticsOnly
    } else if comms.signal_strength < 0.6 {
        FaultAssistanceLevel::PartialGuidance
    } else {
        FaultAssistanceLevel::FullGuidance
    }
}

pub fn get_assistance_description(level: &FaultAssistanceLevel) -> &'static str {
    match level {
        FaultAssistanceLevel::FullGuidance => "Ground Control: Full step-by-step guidance available",
        FaultAssistanceLevel::PartialGuidance => "Ground Control: Partial guidance - general advice only",
        FaultAssistanceLevel::DiagnosticsOnly => "Ground Control: Diagnostics only - no repair procedures",
        FaultAssistanceLevel::NoAssistance => "NO COMMUNICATION - You are on your own",
    }
}

pub fn start_repair(player_actions: &mut PlayerActions, fault_id: FaultId) {
    player_actions.current_repair_target = Some(fault_id);
    player_actions.current_repair_step = 0;
    player_actions.repairs_attempted.push(fault_id);
}

pub fn advance_repair_step(player_actions: &mut PlayerActions) {
    player_actions.current_repair_step += 1;
}

pub fn cancel_repair(player_actions: &mut PlayerActions) {
    player_actions.current_repair_target = None;
    player_actions.current_repair_step = 0;
}

pub fn is_repairable_without_comm(fault_id: FaultId) -> bool {
    fault_id.default_repair_procedure()
        .map(|p| p.can_perform_without_comm)
        .unwrap_or(true)
}


#[cfg(test)]
mod fault_cascade_tests {
    use super::*;

    #[test]
    fn fault_new_sets_active_state() {
        let fault = Fault::new(
            FaultId::MainBusA_Failure,
            FaultSeverity::Critical,
            FaultCategory::Electrical,
            100.0,
        );
        assert_eq!(fault.state, FaultState::Active);
        assert_eq!(fault.id, FaultId::MainBusA_Failure);
        assert_eq!(fault.severity, FaultSeverity::Critical);
        assert_eq!(fault.triggered_at, 100.0);
    }

    #[test]
    fn fault_time_critical_when_limit_set() {
        let mut fault = Fault::new(
            FaultId::O2Tank2_Rupture,
            FaultSeverity::Catastrophic,
            FaultCategory::LifeSupport,
            0.0,
        );
        assert!(fault.is_time_critical());
        assert_eq!(fault.time_limit_seconds, Some(3600.0));
    }

    #[test]
    fn fault_not_time_critical_when_no_limit() {
        let fault = Fault::new(
            FaultId::MainBusA_Failure,
            FaultSeverity::Critical,
            FaultCategory::Electrical,
            0.0,
        );
        assert!(!fault.is_time_critical());
        assert_eq!(fault.time_limit_seconds, None);
    }

    #[test]
    fn fault_time_remaining() {
        let fault = Fault::new(
            FaultId::O2Tank2_Rupture,
            FaultSeverity::Catastrophic,
            FaultCategory::LifeSupport,
            100.0,
        );
        assert_eq!(fault.time_remaining(3100.0), Some(600.0));
        assert_eq!(fault.time_remaining(3700.0), Some(0.0));
    }

    #[test]
    fn fault_expired() {
        let fault = Fault::new(
            FaultId::CabinPressure_Loss,
            FaultSeverity::Critical,
            FaultCategory::LifeSupport,
            0.0,
        );
        assert_eq!(fault.time_limit_seconds, Some(1800.0));
        assert!(!fault.is_expired(1799.0));
        assert!(fault.is_expired(1800.0));
    }

    // ============================================================
    // FaultManager
    // ============================================================

    #[test]
    fn manager_trigger_fault() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::MainBusA_Failure, 100.0);
        assert!(mgr.has_active_fault(FaultId::MainBusA_Failure));
        assert_eq!(mgr.active_faults.len(), 1);
        assert_eq!(mgr.fault_history.len(), 1);
    }

    #[test]
    fn manager_no_duplicate_trigger() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::MainBusA_Failure, 100.0);
        mgr.trigger_fault(FaultId::MainBusA_Failure, 200.0);
        assert_eq!(mgr.active_faults.len(), 1);
    }

    #[test]
    fn manager_resolve_fault() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::MainBusA_Failure, 100.0);
        mgr.resolve_fault(FaultId::MainBusA_Failure, 200.0);
        assert!(!mgr.has_active_fault(FaultId::MainBusA_Failure));
        assert!(mgr.active_faults.is_empty());

        let record = &mgr.fault_history[0];
        assert_eq!(record.resolved_at, Some(200.0));
        assert_eq!(record.final_state, FaultState::Resolved);
    }

    #[test]
    fn manager_escalate_fault() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::MainBusA_Failure, 100.0);
        mgr.escalate_fault(FaultId::MainBusA_Failure);
        assert_eq!(mgr.active_faults[0].state, FaultState::Escalated);
        assert_eq!(mgr.fault_history[0].escalation_count, 1);
        assert_eq!(mgr.fault_history[0].final_state, FaultState::Escalated);
    }

    #[test]
    fn manager_get_by_category() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::MainBusA_Failure, 0.0);
        mgr.trigger_fault(FaultId::O2Tank2_Rupture, 0.0);
        mgr.trigger_fault(FaultId::SBand_Amplifier_Failure, 0.0);

        assert_eq!(mgr.get_active_faults_by_category(FaultCategory::Electrical).len(), 1);
        assert_eq!(mgr.get_active_faults_by_category(FaultCategory::LifeSupport).len(), 1);
        assert_eq!(mgr.get_active_faults_by_category(FaultCategory::Communications).len(), 1);
        assert_eq!(mgr.get_active_faults_by_category(FaultCategory::Propulsion).len(), 0);
    }

    #[test]
    fn manager_get_critical_faults() {
        let mut mgr = FaultManager::default();
        mgr.trigger_fault(FaultId::BatteryDegradation, 0.0);
        mgr.trigger_fault(FaultId::MainBusA_Failure, 0.0);
        mgr.trigger_fault(FaultId::O2Tank2_Rupture, 0.0);

        let critical = mgr.get_critical_faults();
        assert_eq!(critical.len(), 2);
        assert!(critical.iter().all(|f| matches!(f.severity,
            FaultSeverity::Critical | FaultSeverity::Catastrophic)));
    }

    #[test]
    fn manager_load_scripted_apollo13() {
        let mut mgr = FaultManager::default();
        mgr.load_scripted_faults("apollo13");
        assert_eq!(mgr.scripted_faults.len(), 4);
        assert_eq!(mgr.scripted_faults[0].0, 198_000.0);
        assert_eq!(mgr.scripted_faults[0].1, FaultId::CryoStir_WiringFault);
        assert_eq!(mgr.scripted_faults[1].0, 198_060.0);
        assert_eq!(mgr.scripted_faults[1].1, FaultId::O2Tank2_Rupture);
        assert_eq!(mgr.scripted_faults[2].0, 198_120.0);
        assert_eq!(mgr.scripted_faults[2].1, FaultId::MainBusB_Failure);
        assert_eq!(mgr.scripted_faults[3].0, 198_180.0);
        assert_eq!(mgr.scripted_faults[3].1, FaultId::FuelCell1_Failure);
    }

    // ============================================================
    // Cascade Chains
    // ============================================================

    #[test]
    fn cascade_o2tank2_triggers_mainbusb_and_cabin() {
        let targets = FaultId::O2Tank2_Rupture.default_cascade_targets();
        assert!(targets.contains(&FaultId::MainBusB_Failure));
        assert!(targets.contains(&FaultId::CabinPressure_Loss));
    }

    #[test]
    fn cascade_mainbusb_triggers_fc1_and_comm() {
        let targets = FaultId::MainBusB_Failure.default_cascade_targets();
        assert!(targets.contains(&FaultId::FuelCell1_Failure));
        assert!(targets.contains(&FaultId::CommSystem_Degradation));
    }

    #[test]
    fn cascade_cabin_triggers_co2() {
        let targets = FaultId::CabinPressure_Loss.default_cascade_targets();
        assert!(targets.contains(&FaultId::Co2Scrubber_Saturation));
    }

    #[test]
    fn cascade_fc3_triggers_battery_and_sps() {
        let targets = FaultId::FuelCell3_Failure.default_cascade_targets();
        assert!(targets.contains(&FaultId::BatteryDegradation));
        assert!(targets.contains(&FaultId::Sps_Engine_Failure));
    }

    #[test]
    fn cascade_cryo_stir_triggers_o2tank2() {
        let targets = FaultId::CryoStir_WiringFault.default_cascade_targets();
        assert!(targets.contains(&FaultId::O2Tank2_Rupture));
    }

    #[test]
    fn cascade_meteorite_triggers_cabin_and_comm() {
        let targets = FaultId::MicrometeoriteImpact.default_cascade_targets();
        assert!(targets.contains(&FaultId::CabinPressure_Loss));
        assert!(targets.contains(&FaultId::CommSystem_Degradation));
    }

    #[test]
    fn cascade_sps_no_children() {
        let targets = FaultId::Sps_Engine_Failure.default_cascade_targets();
        assert!(targets.is_empty());
    }

    #[test]
    fn full_apollo13_cascade_chain() {
        let mut mgr = FaultManager::default();
        let t0: f64 = 198_000.0;

        mgr.trigger_fault(FaultId::CryoStir_WiringFault, t0);
        assert!(mgr.has_active_fault(FaultId::CryoStir_WiringFault));

        mgr.trigger_fault(FaultId::O2Tank2_Rupture, t0 + 1.0);
        assert!(mgr.has_active_fault(FaultId::O2Tank2_Rupture));

        mgr.trigger_fault(FaultId::MainBusB_Failure, t0 + 2.0);
        assert!(mgr.has_active_fault(FaultId::MainBusB_Failure));

        mgr.trigger_fault(FaultId::FuelCell1_Failure, t0 + 5.0);
        assert!(mgr.has_active_fault(FaultId::FuelCell1_Failure));

        mgr.trigger_fault(FaultId::CabinPressure_Loss, t0 + 10.0);
        assert!(mgr.has_active_fault(FaultId::CabinPressure_Loss));

        mgr.trigger_fault(FaultId::CommSystem_Degradation, t0 + 10.0);
        assert!(mgr.has_active_fault(FaultId::CommSystem_Degradation));

        mgr.trigger_fault(FaultId::BatteryDegradation, t0 + 120.0);
        assert!(mgr.has_active_fault(FaultId::BatteryDegradation));

        mgr.trigger_fault(FaultId::Co2Scrubber_Saturation, t0 + 43200.0);
        assert!(mgr.has_active_fault(FaultId::Co2Scrubber_Saturation));

        assert_eq!(mgr.active_faults.len(), 8);
        assert_eq!(mgr.fault_history.len(), 8);
    }

    // ============================================================
    // FaultId Properties
    // ============================================================

    #[test]
    fn all_faults_have_names() {
        let all_ids = [
            FaultId::MainBusA_Failure, FaultId::MainBusB_Failure,
            FaultId::FuelCell1_Failure, FaultId::FuelCell2_Failure, FaultId::FuelCell3_Failure,
            FaultId::BatteryDegradation,
            FaultId::O2Tank1_Rupture, FaultId::O2Tank2_Rupture,
            FaultId::Co2Scrubber_Saturation, FaultId::CabinPressure_Loss, FaultId::WaterTank_Leak,
            FaultId::SBand_Amplifier_Failure, FaultId::HighGainAntenna_Stuck,
            FaultId::PhaseLock_Loss, FaultId::CommSystem_Degradation,
            FaultId::Sps_Engine_Failure,
            FaultId::Rcs_QuadA_Failure, FaultId::Rcs_QuadB_Failure,
            FaultId::Rcs_QuadC_Failure, FaultId::Rcs_QuadD_Failure,
            FaultId::Imu_GimbalLock, FaultId::StarTracker_Failure,
            FaultId::Agc_ProgramAlarm, FaultId::Accelerometer_Drift,
            FaultId::CryoStir_WiringFault, FaultId::MicrometeoriteImpact, FaultId::HeatShield_Crack,
            FaultId::CentralTiming_ClockDrift, FaultId::CentralTiming_SyncLoss,
            FaultId::CentralTiming_DisplayFailure,
        ];
        for id in &all_ids {
            assert!(!id.name().is_empty(), "{:?} has empty name", id);
            assert!(!id.description().is_empty(), "{:?} has empty description", id);
        }
    }

    #[test]
    fn severity_ordering() {
        assert!(matches!(FaultId::O2Tank2_Rupture.default_severity(), FaultSeverity::Catastrophic));
        assert!(matches!(FaultId::MainBusA_Failure.default_severity(), FaultSeverity::Critical));
        assert!(matches!(FaultId::FuelCell1_Failure.default_severity(), FaultSeverity::Major));
        assert!(matches!(FaultId::BatteryDegradation.default_severity(), FaultSeverity::Minor));
    }

    #[test]
    fn category_mapping() {
        assert_eq!(FaultId::MainBusA_Failure.category(), FaultCategory::Electrical);
        assert_eq!(FaultId::O2Tank2_Rupture.category(), FaultCategory::LifeSupport);
        assert_eq!(FaultId::SBand_Amplifier_Failure.category(), FaultCategory::Communications);
        assert_eq!(FaultId::Sps_Engine_Failure.category(), FaultCategory::Propulsion);
        assert_eq!(FaultId::Imu_GimbalLock.category(), FaultCategory::Guidance);
        assert_eq!(FaultId::HeatShield_Crack.category(), FaultCategory::Structural);
        assert_eq!(FaultId::CentralTiming_ClockDrift.category(), FaultCategory::Timing);
    }

    // ============================================================
    // Repair Procedures
    // ============================================================

    #[test]
    fn repair_procedure_for_main_bus_a() {
        let proc = FaultId::MainBusA_Failure.default_repair_procedure().unwrap();
        assert_eq!(proc.steps.len(), 2);
        assert_eq!(proc.time_limit, Some(600.0));
        assert!(proc.can_perform_without_comm);
    }

    #[test]
    fn repair_procedure_for_co2_scrubber() {
        let proc = FaultId::Co2Scrubber_Saturation.default_repair_procedure().unwrap();
        assert_eq!(proc.steps.len(), 2);
        assert!(proc.required_tools.contains(&Tool::ReplacementCanister));
    }

    #[test]
    fn repair_procedure_for_agc_alarm() {
        let proc = FaultId::Agc_ProgramAlarm.default_repair_procedure().unwrap();
        assert_eq!(proc.steps.len(), 3);
        assert_eq!(proc.time_limit, Some(120.0));
        assert!(matches!(proc.steps[2].action, RepairAction::EnterVerbNoun(37, 0)));
    }

    #[test]
    fn repair_procedure_for_gimbal_lock() {
        let proc = FaultId::Imu_GimbalLock.default_repair_procedure().unwrap();
        assert!(matches!(proc.steps[0].action, RepairAction::EnterVerbNoun(41, 1)));
    }

    #[test]
    fn some_faults_have_no_repair() {
        assert!(FaultId::WaterTank_Leak.default_repair_procedure().is_none());
        assert!(FaultId::Accelerometer_Drift.default_repair_procedure().is_none());
    }

    // ============================================================
    // Apollo 12 Lightning Strike Scenario
    // ============================================================

    #[test]
    fn apollo12_lightning_strike() {
        let mut mgr = FaultManager::default();
        let t_launch: f64 = 0.0;
        let t_strike: f64 = 36.5;

        mgr.trigger_fault(FaultId::MainBusA_Failure, t_strike);
        mgr.trigger_fault(FaultId::MainBusB_Failure, t_strike + 0.1);

        assert!(mgr.has_active_fault(FaultId::MainBusA_Failure));
        assert!(mgr.has_active_fault(FaultId::MainBusB_Failure));

        let critical = mgr.get_critical_faults();
        assert_eq!(critical.len(), 2);

        mgr.resolve_fault(FaultId::MainBusA_Failure, t_strike + 30.0);
        assert!(!mgr.has_active_fault(FaultId::MainBusA_Failure));
        assert!(mgr.has_active_fault(FaultId::MainBusB_Failure));
    }

    // ============================================================
    // Severity/State String Formatting
    // ============================================================

    #[test]
    fn severity_strings() {
        assert_eq!(FaultSeverity::Minor.as_str(), "MINOR");
        assert_eq!(FaultSeverity::Major.as_str(), "MAJOR");
        assert_eq!(FaultSeverity::Critical.as_str(), "CRITICAL");
        assert_eq!(FaultSeverity::Catastrophic.as_str(), "CATASTROPHIC");
    }

    #[test]
    fn state_strings() {
        assert_eq!(FaultState::Dormant.as_str(), "DORMANT");
        assert_eq!(FaultState::Active.as_str(), "ACTIVE");
        assert_eq!(FaultState::Contained.as_str(), "CONTAINED");
        assert_eq!(FaultState::Resolved.as_str(), "RESOLVED");
        assert_eq!(FaultState::Escalated.as_str(), "ESCALATED");
    }

    #[test]
    fn category_strings() {
        assert_eq!(FaultCategory::Electrical.as_str(), "ELECTRICAL");
        assert_eq!(FaultCategory::LifeSupport.as_str(), "LIFE SUPPORT");
        assert_eq!(FaultCategory::Communications.as_str(), "COMMUNICATIONS");
        assert_eq!(FaultCategory::Propulsion.as_str(), "PROPULSION");
        assert_eq!(FaultCategory::Guidance.as_str(), "GNC");
        assert_eq!(FaultCategory::Structural.as_str(), "STRUCTURAL");
        assert_eq!(FaultCategory::Timing.as_str(), "TIMING");
    }
}
