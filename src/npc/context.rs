use bevy::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SituationReport {
    pub mission_time_seconds: f64,
    pub mission_phase: String,
    pub difficulty_phase: String,
    pub comm_signal_strength: f32,
    pub active_faults: Vec<FaultReport>,
    pub crew_status: Vec<CrewReport>,
    pub csm_systems: CsmReport,
}

#[derive(Debug, Clone, Serialize)]
pub struct FaultReport {
    pub id: String,
    pub severity: String,
    pub category: String,
    pub state: String,
    pub affected_systems: Vec<String>,
    pub time_active_seconds: f64,
    pub repair_steps_remaining: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrewReport {
    pub name: String,
    pub role: String,
    pub status: String,
    pub heart_rate: f32,
    pub body_temp_c: f32,
    pub co2_exposure: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CsmReport {
    pub main_bus_a_voltage: f32,
    pub main_bus_b_voltage: f32,
    pub fuel_cell_1_load: f32,
    pub fuel_cell_2_load: f32,
    pub fuel_cell_3_load: f32,
    pub battery_charge_pct: f32,
    pub o2_tank_1_pressure: f32,
    pub o2_tank_2_pressure: f32,
    pub cabin_pressure_psi: f32,
    pub cabin_temp_c: f32,
    pub cabin_co2_mmhg: f32,
    pub sps_status: String,
    pub rcs_quads_nominal: u8,
    pub imu_aligned: bool,
    pub agc_alarm: bool,
}

impl SituationReport {
    pub fn build(
        mission_time: f64,
        mission_phase: &str,
        difficulty_phase: &str,
        comm_signal: f32,
        fault_manager: &crate::faults::FaultManager,
        crew_query: &Query<&crate::crew::CrewMember>,
        csm_query: &Query<&crate::systems::csm::CommandServiceModule>,
    ) -> Self {
        let active_faults = fault_manager
            .active_faults
            .iter()
            .map(|f| FaultReport {
                id: f.id.name().to_string(),
                severity: f.severity.as_str().to_string(),
                category: f.category.as_str().to_string(),
                state: f.state.as_str().to_string(),
                affected_systems: f.affected_systems.clone(),
                time_active_seconds: mission_time - f.triggered_at,
                repair_steps_remaining: f
                    .repair_procedure
                    .as_ref()
                    .map(|p| p.steps.len().saturating_sub(0))
                    .unwrap_or(0),
            })
            .collect();

        let crew_status = crew_query
            .iter()
            .map(|c| CrewReport {
                name: c.name.clone(),
                role: c.role_as_str().to_string(),
                status: format!("{:?}", c.health.status),
                heart_rate: c.health.heart_rate_bpm,
                body_temp_c: c.health.body_temp_c,
                co2_exposure: c.health.co2_exposure_mmhg,
            })
            .collect();

        let csm_systems = csm_query
            .get_single()
            .map(|csm| CsmReport {
                main_bus_a_voltage: if csm.electrical.main_bus_a { csm.electrical.bus_voltage } else { 0.0 },
                main_bus_b_voltage: if csm.electrical.main_bus_b { csm.electrical.bus_voltage } else { 0.0 },
                fuel_cell_1_load: csm.electrical.fuel_cells.get(0).map(|fc| fc.output_kw).unwrap_or(0.0),
                fuel_cell_2_load: csm.electrical.fuel_cells.get(1).map(|fc| fc.output_kw).unwrap_or(0.0),
                fuel_cell_3_load: csm.electrical.fuel_cells.get(2).map(|fc| fc.output_kw).unwrap_or(0.0),
                battery_charge_pct: csm.electrical.batteries.iter().map(|b| b.charge_pct).sum::<f32>() / csm.electrical.batteries.len().max(1) as f32,
                o2_tank_1_pressure: 900.0,
                o2_tank_2_pressure: 900.0,
                cabin_pressure_psi: csm.environmental_control.cabin_atmosphere.pressure_psi,
                cabin_temp_c: csm.environmental_control.cabin_atmosphere.temp_c,
                cabin_co2_mmhg: csm.environmental_control.cabin_atmosphere.co2_partial_pressure_mmhg,
                sps_status: format!("{:?}", csm.sps.engine.status),
                rcs_quads_nominal: csm
                    .rcs
                    .quads
                    .iter()
                    .filter(|q| q.enabled)
                    .count() as u8,
                imu_aligned: csm.gnc.imu_aligned,
                agc_alarm: false,
            })
            .unwrap_or_else(|_| CsmReport {
                main_bus_a_voltage: 28.0,
                main_bus_b_voltage: 28.0,
                fuel_cell_1_load: 0.0,
                fuel_cell_2_load: 0.0,
                fuel_cell_3_load: 0.0,
                battery_charge_pct: 100.0,
                o2_tank_1_pressure: 900.0,
                o2_tank_2_pressure: 900.0,
                cabin_pressure_psi: 5.0,
                cabin_temp_c: 21.0,
                cabin_co2_mmhg: 2.0,
                sps_status: "Nominal".into(),
                rcs_quads_nominal: 4,
                imu_aligned: false,
                agc_alarm: false,
            });

        Self {
            mission_time_seconds: mission_time,
            mission_phase: mission_phase.to_string(),
            difficulty_phase: difficulty_phase.to_string(),
            comm_signal_strength: comm_signal,
            active_faults,
            crew_status,
            csm_systems,
        }
    }

    pub fn to_prompt_text(&self) -> String {
        let mut s = String::with_capacity(2048);

        let hours = (self.mission_time_seconds / 3600.0) as u32;
        let mins = ((self.mission_time_seconds % 3600.0) / 60.0) as u32;
        let secs = (self.mission_time_seconds % 60.0) as u32;

        s.push_str(&format!(
            "MET: {:02}:{:02}:{:02}\n",
            hours, mins, secs
        ));
        s.push_str(&format!("Phase: {}\n", self.mission_phase));
        s.push_str(&format!("Difficulty: {}\n", self.difficulty_phase));
        s.push_str(&format!(
            "Comm Signal: {:.0}%\n",
            self.comm_signal_strength * 100.0
        ));

        if self.active_faults.is_empty() {
            s.push_str("\nNo active faults.\n");
        } else {
            s.push_str(&format!("\nACTIVE FAULTS ({}):\n", self.active_faults.len()));
            for f in &self.active_faults {
                s.push_str(&format!(
                    "- {} [{}] {} - active {:.0}s - affects: {}\n",
                    f.id,
                    f.severity,
                    f.state,
                    f.time_active_seconds,
                    f.affected_systems.join(", ")
                ));
            }
        }

        s.push_str("\nCREW:\n");
        for c in &self.crew_status {
            s.push_str(&format!(
                "- {} ({}): {} HR:{}bpm T:{:.1}C CO2:{:.1}mmHg\n",
                c.name, c.role, c.status, c.heart_rate, c.body_temp_c, c.co2_exposure
            ));
        }

        let sys = &self.csm_systems;
        s.push_str("\nCSM SYSTEMS:\n");
        s.push_str(&format!(
            "Bus A: {:.1}V  Bus B: {:.1}V  Battery: {:.0}%\n",
            sys.main_bus_a_voltage, sys.main_bus_b_voltage, sys.battery_charge_pct
        ));
        s.push_str(&format!(
            "O2 T1: {:.0}psi  O2 T2: {:.0}psi  Cabin: {:.1}psi {:.1}C CO2:{:.1}mmHg\n",
            sys.o2_tank_1_pressure,
            sys.o2_tank_2_pressure,
            sys.cabin_pressure_psi,
            sys.cabin_temp_c,
            sys.cabin_co2_mmhg
        ));
        s.push_str(&format!(
            "SPS: {}  RCS quads nominal: {}/4  IMU: {}  AGC alarm: {}\n",
            sys.sps_status,
            sys.rcs_quads_nominal,
            if sys.imu_aligned { "ALIGNED" } else { "UNALIGNED" },
            sys.agc_alarm
        ));

        s
    }
}
