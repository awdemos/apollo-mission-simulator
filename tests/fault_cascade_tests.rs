#[cfg(test)]
mod fault_cascade_tests {
    use apollo_mission_simulator::faults::*;

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
        assert_eq!(mgr.scripted_faults.len(), 2);
        assert_eq!(mgr.scripted_faults[0].0, 198_000.0);
        assert_eq!(mgr.scripted_faults[0].1, FaultId::CryoStir_WiringFault);
        assert_eq!(mgr.scripted_faults[1].1, FaultId::O2Tank2_Rupture);
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
            FaultId::RcsQuadA_Failure, FaultId::RcsQuadB_Failure,
            FaultId::RcsQuadC_Failure, FaultId::RcsQuadD_Failure,
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
