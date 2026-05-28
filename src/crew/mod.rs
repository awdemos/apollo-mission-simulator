use bevy::prelude::*;

pub struct CrewPlugin;

impl Plugin for CrewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorkloadTracker>()
            .add_systems(Update, update_crew_health.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_master_alarm_from_health.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, spawn_crew_visuals);
    }
}

#[derive(Component)]
pub struct CrewVisual;

fn spawn_crew_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crew_query: Query<(Entity, &CrewMember), Without<CrewVisual>>,
    interior_query: Query<Entity, With<crate::spacecraft::CmInterior>>,
) {
    let Ok(interior) = interior_query.get_single() else { return };
    
    let suit_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.9),
        metallic: 0.3,
        perceptual_roughness: 0.6,
        ..default()
    });
    let helmet_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.88),
        metallic: 0.6,
        perceptual_roughness: 0.3,
        ..default()
    });
    
    let scale = crate::config::SATURN_V_SCALE;
    for (entity, crew) in crew_query.iter() {
        let (x, z) = match crew.role {
            CrewRole::Commander => (-crate::config::COUCH_SPACING * scale, crate::config::COUCH_Z * scale),
            CrewRole::CommandModulePilot => (0.0, crate::config::COUCH_Z * scale + 0.15 * scale),
            CrewRole::LunarModulePilot => (crate::config::COUCH_SPACING * scale, crate::config::COUCH_Z * scale),
        };
        let y = crate::config::COUCH_Y * scale + 0.35 * scale;
        
        let body = meshes.add(Cylinder::new(0.18 * scale, 0.5 * scale));
        let head = meshes.add(Sphere::new(0.13 * scale));
        let arm = meshes.add(Cylinder::new(0.035 * scale, 0.32 * scale));
        let leg = meshes.add(Cylinder::new(0.055 * scale, 0.38 * scale));
        
        let visual = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            CrewVisual,
        )).id();
        
        commands.entity(visual).with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: body,
                material: suit_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: head,
                material: helmet_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.35, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: arm.clone(),
                material: suit_mat.clone(),
                transform: Transform::from_xyz(-0.22, 0.05, 0.0)
                    .with_rotation(Quat::from_rotation_z(0.15)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: arm,
                material: suit_mat.clone(),
                transform: Transform::from_xyz(0.22, 0.05, 0.0)
                    .with_rotation(Quat::from_rotation_z(-0.15)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: leg.clone(),
                material: suit_mat.clone(),
                transform: Transform::from_xyz(-0.1, -0.4, 0.1)
                    .with_rotation(Quat::from_rotation_x(-0.3)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: leg,
                material: suit_mat.clone(),
                transform: Transform::from_xyz(0.1, -0.4, 0.1)
                    .with_rotation(Quat::from_rotation_x(-0.3)),
                ..default()
            });
        });
        
        commands.entity(interior).add_child(visual);
        commands.entity(entity).insert(CrewVisual);
    }
}

#[derive(Component, Debug, Clone)]
pub struct CrewMember {
    pub name: String,
    pub role: CrewRole,
    pub health: CrewHealth,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerCharacter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrewRole {
    Commander,
    CommandModulePilot,
    LunarModulePilot,
}

#[derive(Debug, Clone)]
pub struct CrewHealth {
    pub heart_rate_bpm: f32,
    pub respiration_rate: f32,
    pub body_temp_c: f32,
    pub blood_pressure_sys: f32,
    pub blood_pressure_dia: f32,
    pub co2_exposure_mmhg: f32,
    pub radiation_exposure_mrem: f32,
    pub dehydration_pct: f32,
    pub fatigue_pct: f32,
    pub status: HealthStatus,
}

impl Default for CrewHealth {
    fn default() -> Self {
        Self {
            heart_rate_bpm: 72.0,
            respiration_rate: 16.0,
            body_temp_c: 37.0,
            blood_pressure_sys: 120.0,
            blood_pressure_dia: 80.0,
            co2_exposure_mmhg: 0.0,
            radiation_exposure_mrem: 0.0,
            dehydration_pct: 0.0,
            fatigue_pct: 0.0,
            status: HealthStatus::Healthy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Stressed,
    Fatigued,
    Impaired,
    Critical,
    Deceased,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct WorkloadTracker {
    pub agc_operations_last_minute: u32,
    pub switch_operations_last_minute: u32,
    pub timer: f32,
}

fn update_crew_health(
    time: Res<Time>,
    mut crew_query: Query<&mut CrewMember>,
    csm_query: Query<&crate::systems::csm::CommandServiceModule>,
    mut workload: ResMut<WorkloadTracker>,
) {
    let dt = time.delta_seconds();
    let dt_hours = dt / 3600.0;
    let dt_minutes = dt / 60.0;

    workload.timer += dt;
    if workload.timer >= 60.0 {
        workload.timer -= 60.0;
        workload.agc_operations_last_minute = workload.agc_operations_last_minute.saturating_sub(10);
        workload.switch_operations_last_minute = workload.switch_operations_last_minute.saturating_sub(5);
    }

    let high_workload = workload.agc_operations_last_minute > 20
        || workload.switch_operations_last_minute > 10;

    for mut crew in crew_query.iter_mut() {
        if crew.health.status == HealthStatus::Deceased {
            continue;
        }

        let cabin_co2 = if let Ok(csm) = csm_query.get_single() {
            csm.environmental_control.cabin_atmosphere.co2_partial_pressure_mmhg
        } else {
            0.0
        };

        let cabin_temp = if let Ok(csm) = csm_query.get_single() {
            csm.environmental_control.cabin_atmosphere.temp_c
        } else {
            21.0
        };

        let cabin_pressure = if let Ok(csm) = csm_query.get_single() {
            csm.environmental_control.cabin_atmosphere.pressure_psi
        } else {
            5.0
        };

        let suit_compressor_on = if let Ok(csm) = csm_query.get_single() {
            csm.environmental_control.suit_circuit.compressor_active
        } else {
            true
        };

        let target_hr = 72.0
            + if cabin_co2 > 7.6 { 10.0 } else { 0.0 }
            + if cabin_temp > 27.0 { 5.0 } else { 0.0 }
            + if cabin_pressure < 4.0 { 15.0 } else { 0.0 }
            + if crew.health.fatigue_pct > 80.0 { 8.0 } else { 0.0 };

        crew.health.heart_rate_bpm = crew.health.heart_rate_bpm.lerp(target_hr, 0.1 * dt_minutes);
        crew.health.heart_rate_bpm = crew.health.heart_rate_bpm.clamp(0.0, 200.0);

        if cabin_temp > 27.0 {
            crew.health.body_temp_c += 0.1 * dt_minutes;
        } else if cabin_temp < 18.0 {
            crew.health.body_temp_c -= 0.05 * dt_minutes;
        } else {
            crew.health.body_temp_c = crew.health.body_temp_c.lerp(37.0, 0.5 * dt_minutes);
        }

        if !suit_compressor_on {
            crew.health.body_temp_c += 0.2 * dt_minutes;
        }

        crew.health.body_temp_c = crew.health.body_temp_c.clamp(30.0, 42.0);

        if cabin_co2 > 7.6 {
            let rate = if cabin_co2 > 15.0 { 2.0 } else { 0.5 };
            crew.health.co2_exposure_mmhg += rate * dt_minutes;
        } else {
            crew.health.co2_exposure_mmhg = crew.health.co2_exposure_mmhg.lerp(0.0, 0.2 * dt_minutes);
        }
        crew.health.co2_exposure_mmhg = crew.health.co2_exposure_mmhg.clamp(0.0, 50.0);

        let dehydration_rate = 0.5
            + if cabin_temp > 27.0 { 1.0 } else { 0.0 };
        crew.health.dehydration_pct += dehydration_rate * dt_hours;
        crew.health.dehydration_pct = crew.health.dehydration_pct.clamp(0.0, 100.0);

        let fatigue_rate = 2.0
            + if high_workload { 5.0 } else { 0.0 }
            + if crew.health.status == HealthStatus::Critical { 0.0 } else { 0.0 };
        crew.health.fatigue_pct += fatigue_rate * dt_hours;

        if crew.health.status != HealthStatus::Critical {
            crew.health.fatigue_pct = crew.health.fatigue_pct.clamp(0.0, 100.0);
        }

        crew.health.status = compute_health_status(&crew.health);

        if crew.health.body_temp_c >= 42.0 || crew.health.heart_rate_bpm >= 200.0 {
            crew.health.status = HealthStatus::Deceased;
            crew.health.heart_rate_bpm = 0.0;
        }
    }
}

fn compute_health_status(health: &CrewHealth) -> HealthStatus {
    if health.heart_rate_bpm == 0.0 {
        return HealthStatus::Deceased;
    }

    let critical = health.body_temp_c >= 40.0
        || health.heart_rate_bpm >= 180.0
        || health.co2_exposure_mmhg >= 20.0
        || health.dehydration_pct >= 90.0;

    if critical {
        return HealthStatus::Critical;
    }

    let impaired = health.co2_exposure_mmhg >= 10.0
        || health.dehydration_pct >= 70.0
        || health.body_temp_c >= 38.5;

    if impaired {
        return HealthStatus::Impaired;
    }

    if health.fatigue_pct >= 85.0 {
        return HealthStatus::Fatigued;
    }

    let stressed = health.heart_rate_bpm >= 100.0
        || health.body_temp_c >= 37.8;

    if stressed {
        return HealthStatus::Stressed;
    }

    HealthStatus::Healthy
}

fn update_master_alarm_from_health(
    crew_query: Query<&CrewMember>,
    mut master_alarm: ResMut<crate::game_state::MasterAlarm>,
) {
    let critical = crew_query.iter().find(|c| c.health.status == HealthStatus::Critical);

    if let Some(crew) = critical {
        master_alarm.active = true;
        master_alarm.triggered_by = Some(format!("{} ({})", crew.name, crew.role_as_str()));
    } else {
        master_alarm.active = false;
        master_alarm.triggered_by = None;
    }
}

impl CrewMember {
    pub fn role_as_str(&self) -> &'static str {
        match self.role {
            CrewRole::Commander => "CDR",
            CrewRole::CommandModulePilot => "CMP",
            CrewRole::LunarModulePilot => "LMP",
        }
    }
}
