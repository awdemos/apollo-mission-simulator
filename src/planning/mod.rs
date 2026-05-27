use bevy::prelude::*;

pub struct PlanningPlugin;

impl Plugin for PlanningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MissionPlan>()
            .add_systems(Startup, setup_planning);
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct MissionPlan {
    pub launch: LaunchPlan,
    pub flight_path: FlightPathPlan,
    pub return_trajectory: ReturnPlan,
    pub status: PlanStatus,
}

#[derive(Default, Debug, Clone)]
pub struct LaunchPlan {
    pub launch_window_start: String,
    pub launch_window_end: String,
    pub launch_azimuth: f32,
    pub launch_time_met: f64,
    pub target_inclination: f32,
    pub target_orbit_altitude_km: f32,
}

#[derive(Default, Debug, Clone)]
pub struct FlightPathPlan {
    pub tli_burn_time_met: f64,
    pub tli_duration_seconds: f32,
    pub target_free_return: bool,
    pub loi_burn_altitude_km: f32,
    pub landing_site_latitude: f32,
    pub landing_site_longitude: f32,
    pub landing_site_name: String,
}

#[derive(Default, Debug, Clone)]
pub struct ReturnPlan {
    pub tei_burn_time_met: f64,
    pub tei_duration_seconds: f32,
    pub entry_interface_altitude_km: f32,
    pub splashdown_latitude: f32,
    pub splashdown_longitude: f32,
    pub recovery_ship: String,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanStatus {
    #[default]
    Draft,
    Validated,
    Approved,
    Executing,
    Completed,
    Aborted,
}

impl PlanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlanStatus::Draft => "DRAFT",
            PlanStatus::Validated => "VALIDATED",
            PlanStatus::Approved => "APPROVED",
            PlanStatus::Executing => "EXECUTING",
            PlanStatus::Completed => "COMPLETED",
            PlanStatus::Aborted => "ABORTED",
        }
    }
}

fn setup_planning(mut plan: ResMut<MissionPlan>) {
    *plan = MissionPlan {
        launch: LaunchPlan {
            launch_window_start: "1969-07-16T13:32:00Z".to_string(),
            launch_window_end: "1969-07-16T16:32:00Z".to_string(),
            launch_azimuth: 72.0,
            launch_time_met: 0.0,
            target_inclination: 32.5,
            target_orbit_altitude_km: 185.0,
        },
        flight_path: FlightPathPlan {
            tli_burn_time_met: 10_200.0,
            tli_duration_seconds: 349.0,
            target_free_return: true,
            loi_burn_altitude_km: 110.0,
            landing_site_latitude: 0.6741,
            landing_site_longitude: 23.4731,
            landing_site_name: "Mare Tranquillitatis".to_string(),
        },
        return_trajectory: ReturnPlan {
            tei_burn_time_met: 531_600.0,
            tei_duration_seconds: 151.0,
            entry_interface_altitude_km: 121.9,
            splashdown_latitude: 13.19,
            splashdown_longitude: -169.14,
            recovery_ship: "USS Hornet (CV-12)".to_string(),
        },
        status: PlanStatus::Draft,
    };
}

pub fn validate_launch_plan(plan: &LaunchPlan) -> Vec<String> {
    let mut issues = Vec::new();
    
    if plan.launch_azimuth < 45.0 || plan.launch_azimuth > 115.0 {
        issues.push("Launch azimuth out of acceptable range (45°-115°)".to_string());
    }
    if plan.target_inclination < 28.0 || plan.target_inclination > 40.0 {
        issues.push("Target inclination out of acceptable range (28°-40°)".to_string());
    }
    if plan.target_orbit_altitude_km < 160.0 || plan.target_orbit_altitude_km > 220.0 {
        issues.push("Target orbit altitude out of acceptable range (160-220 km)".to_string());
    }
    
    issues
}

pub fn validate_flight_path(plan: &FlightPathPlan) -> Vec<String> {
    let mut issues = Vec::new();
    
    if plan.tli_burn_time_met < 3_600.0 || plan.tli_burn_time_met > 28_800.0 {
        issues.push("TLI burn time outside acceptable window".to_string());
    }
    if plan.landing_site_latitude.abs() > 30.0 {
        issues.push("Landing site latitude too extreme for Apollo landings".to_string());
    }
    if plan.loi_burn_altitude_km < 80.0 || plan.loi_burn_altitude_km > 150.0 {
        issues.push("LOI burn altitude out of acceptable range".to_string());
    }
    
    issues
}

pub fn validate_return_plan(plan: &ReturnPlan) -> Vec<String> {
    let mut issues = Vec::new();
    
    if plan.tei_burn_time_met < 432_000.0 || plan.tei_burn_time_met > 720_000.0 {
        issues.push("TEI burn time outside acceptable window".to_string());
    }
    if plan.entry_interface_altitude_km < 100.0 || plan.entry_interface_altitude_km > 150.0 {
        issues.push("Entry interface altitude out of acceptable range".to_string());
    }
    if plan.splashdown_latitude.abs() > 30.0 {
        issues.push("Splashdown latitude too extreme for Pacific recovery".to_string());
    }
    
    issues
}
