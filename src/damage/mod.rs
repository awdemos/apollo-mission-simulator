use bevy::prelude::*;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<ExplosionEvent>()
            .add_event::<EmergencyAbortEvent>()
            .init_resource::<DamageControlState>()
            .add_systems(Update, apply_damage.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, check_explosions.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, spawn_explosion_visuals.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_debris.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_explosion_particles.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, apply_fault_damage.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, process_damage_control.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, handle_emergency_abort.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_fire_suppression.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, monitor_structural_integrity.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Component, Debug, Clone)]
pub struct StructuralIntegrity {
    pub max_integrity: f32,
    pub current_integrity: f32,
    pub component_name: String,
    pub is_destroyed: bool,
    pub explosion_threshold: f32,
}

impl StructuralIntegrity {
    pub fn new(name: &str, max: f32) -> Self {
        Self {
            max_integrity: max,
            current_integrity: max,
            component_name: name.to_string(),
            is_destroyed: false,
            explosion_threshold: 0.15,
        }
    }
    
    pub fn integrity_pct(&self) -> f32 {
        self.current_integrity / self.max_integrity
    }
    
    pub fn apply_damage(&mut self, amount: f32) {
        if self.is_destroyed {
            return;
        }
        self.current_integrity = (self.current_integrity - amount).max(0.0);
        if self.current_integrity <= 0.0 {
            self.is_destroyed = true;
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub target_entity: Entity,
    pub damage_amount: f32,
    pub damage_type: DamageType,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Explosive,
    Thermal,
    Structural,
    Impact,
    Overpressure,
}

#[derive(Event, Debug, Clone)]
pub struct ExplosionEvent {
    pub position: Vec3,
    pub radius: f32,
    pub intensity: f32,
    pub source: String,
}

#[derive(Component)]
pub struct ExplosionEffect {
    pub timer: Timer,
    pub max_radius: f32,
    pub intensity: f32,
}

#[derive(Component)]
pub struct ExplosionParticle {
    pub velocity: Vec3,
    pub lifetime: Timer,
    pub initial_size: f32,
}

#[derive(Component)]
pub struct Debris {
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub lifetime: Timer,
    pub mass: f32,
}

fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut integrity_query: Query<&mut StructuralIntegrity>,
) {
    for event in damage_events.read() {
        if let Ok(mut integrity) = integrity_query.get_mut(event.target_entity) {
            let multiplier = match event.damage_type {
                DamageType::Explosive => 2.0,
                DamageType::Thermal => 1.5,
                DamageType::Structural => 1.0,
                DamageType::Impact => 1.2,
                DamageType::Overpressure => 1.8,
            };
            integrity.apply_damage(event.damage_amount * multiplier);
        }
    }
}

fn check_explosions(
    mut integrity_query: Query<(Entity, &mut StructuralIntegrity, &GlobalTransform)>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut commands: Commands,
) {
    for (entity, mut integrity, transform) in integrity_query.iter_mut() {
        if integrity.is_destroyed && integrity.explosion_threshold > 0.0 {
            let pos = transform.translation();
            let radius = integrity.max_integrity * 0.1;
            let intensity = integrity.max_integrity * 0.5;
            
            explosion_events.send(ExplosionEvent {
                position: pos,
                radius,
                intensity,
                source: integrity.component_name.clone(),
            });
            
            integrity.explosion_threshold = 0.0;
            commands.entity(entity).insert(NeedsExplosion);
        }
    }
}

#[derive(Component)]
pub struct NeedsExplosion;

fn spawn_explosion_visuals(
    mut explosion_events: EventReader<ExplosionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in explosion_events.read() {
        let core_mesh = meshes.add(Sphere::new(event.radius * 0.3));
        let core_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.6, 0.1),
            emissive: LinearRgba::new(2.0, 1.2, 0.2, 5.0),
            unlit: true,
            ..default()
        });
        
        commands.spawn((
            PbrBundle {
                mesh: core_mesh,
                material: core_mat,
                transform: Transform::from_translation(event.position)
                    .with_scale(Vec3::splat(1.0)),
                ..default()
            },
            ExplosionEffect {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
                max_radius: event.radius,
                intensity: event.intensity,
            },
        ));
        
        let particle_count = (event.intensity * 2.0) as i32;
        for _ in 0..particle_count.clamp(10, 100) {
            let size = event.radius * 0.05 * (rand::random::<f32>() * 0.5 + 0.5);
            let particle_mesh = meshes.add(Sphere::new(size));
            let particle_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.4 + rand::random::<f32>() * 0.4, 0.1),
                emissive: LinearRgba::new(1.5, 0.6, 0.1, 3.0),
                unlit: true,
                ..default()
            });
            
            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * event.radius * 3.0,
                (rand::random::<f32>() - 0.5) * event.radius * 3.0,
                (rand::random::<f32>() - 0.5) * event.radius * 3.0,
            );
            
            commands.spawn((
                PbrBundle {
                    mesh: particle_mesh,
                    material: particle_mat,
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                ExplosionParticle {
                    velocity,
                    lifetime: Timer::from_seconds(2.0 + rand::random::<f32>() * 2.0, TimerMode::Once),
                    initial_size: size,
                },
            ));
        }
        
        let debris_count = (event.intensity * 0.5) as i32;
        for _ in 0..debris_count.clamp(5, 30) {
            let size = event.radius * 0.1 * (rand::random::<f32>() + 0.5);
            let debris_mesh = meshes.add(Cuboid::new(size, size * 0.7, size * 0.5));
            let debris_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.3 + rand::random::<f32>() * 0.3, 0.3, 0.3),
                metallic: 0.7,
                perceptual_roughness: 0.4,
                ..default()
            });
            
            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * event.radius * 2.0,
                (rand::random::<f32>() - 0.5) * event.radius * 2.0,
                (rand::random::<f32>() - 0.5) * event.radius * 2.0,
            );
            
            let angular = Vec3::new(
                (rand::random::<f32>() - 0.5) * 5.0,
                (rand::random::<f32>() - 0.5) * 5.0,
                (rand::random::<f32>() - 0.5) * 5.0,
            );
            
            commands.spawn((
                PbrBundle {
                    mesh: debris_mesh,
                    material: debris_mat,
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                Debris {
                    velocity,
                    angular_velocity: angular,
                    lifetime: Timer::from_seconds(10.0 + rand::random::<f32>() * 20.0, TimerMode::Once),
                    mass: size * 10.0,
                },
            ));
        }
    }
}

fn update_explosion_particles(
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut ExplosionParticle, &mut Transform)>,
    mut commands: Commands,
) {
    let dt = time.delta_seconds();
    for (entity, mut particle, mut transform) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        let t = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
        let scale = 1.0 + t * 3.0;
        transform.scale = Vec3::splat(scale);
        transform.translation += particle.velocity * dt;
        particle.velocity *= 0.98;
    }
}

fn update_debris(
    time: Res<Time>,
    mut debris_query: Query<(Entity, &mut Debris, &mut Transform)>,
    mut commands: Commands,
) {
    let dt = time.delta_seconds();
    for (entity, mut debris, mut transform) in debris_query.iter_mut() {
        debris.lifetime.tick(time.delta());
        
        if debris.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        transform.translation += debris.velocity * dt;
        transform.rotation *= Quat::from_euler(
            EulerRot::XYZ,
            debris.angular_velocity.x * dt,
            debris.angular_velocity.y * dt,
            debris.angular_velocity.z * dt,
        );
        
        debris.velocity.y -= 0.5 * dt;
        debris.velocity *= 0.995;
    }
}

fn apply_fault_damage(
    mut fault_events: EventReader<crate::faults::FaultTriggeredEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    integrity_query: Query<(Entity, &StructuralIntegrity)>,
) {
    for event in fault_events.read() {
        let (target, damage) = match event.fault_id {
            crate::faults::FaultId::O2Tank1_Rupture | crate::faults::FaultId::O2Tank2_Rupture => {
                find_component(&integrity_query, "Service Module")
                    .map(|e| (e, 800.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::Sps_Engine_Failure => {
                find_component(&integrity_query, "Service Module")
                    .map(|e| (e, 600.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::Rcs_QuadA_Failure | crate::faults::FaultId::Rcs_QuadB_Failure |
            crate::faults::FaultId::Rcs_QuadC_Failure | crate::faults::FaultId::Rcs_QuadD_Failure => {
                find_component(&integrity_query, "Service Module")
                    .map(|e| (e, 400.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::HeatShield_Crack => {
                find_component(&integrity_query, "Command Module")
                    .map(|e| (e, 500.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::MicrometeoriteImpact => {
                find_random_component(&integrity_query)
                    .map(|e| (e, 300.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::MainBusA_Failure | crate::faults::FaultId::MainBusB_Failure |
            crate::faults::FaultId::FuelCell1_Failure | crate::faults::FaultId::FuelCell2_Failure |
            crate::faults::FaultId::FuelCell3_Failure => {
                find_component(&integrity_query, "Service Module")
                    .map(|e| (e, 350.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            crate::faults::FaultId::CabinPressure_Loss => {
                find_component(&integrity_query, "Command Module")
                    .map(|e| (e, 450.0))
                    .unwrap_or((Entity::PLACEHOLDER, 0.0))
            }
            _ => (Entity::PLACEHOLDER, 0.0),
        };
        
        if target != Entity::PLACEHOLDER && damage > 0.0 {
            damage_events.send(DamageEvent {
                target_entity: target,
                damage_amount: damage,
                damage_type: DamageType::Structural,
                source: format!("Fault: {:?}", event.fault_id),
            });
        }
    }
}

fn find_component(query: &Query<(Entity, &StructuralIntegrity)>, name: &str) -> Option<Entity> {
    query.iter().find(|(_, i)| i.component_name == name).map(|(e, _)| e)
}

fn find_random_component(query: &Query<(Entity, &StructuralIntegrity)>) -> Option<Entity> {
    let count = query.iter().count();
    if count == 0 {
        return None;
    }
    let idx = rand::random::<usize>() % count;
    query.iter().nth(idx).map(|(e, _)| e)
}

#[derive(Resource, Default, Debug)]
pub struct DamageControlState {
    pub active_repairs: Vec<ActiveRepair>,
    pub fire_suppression_active: bool,
    pub abort_mode: AbortMode,
    pub last_alert_time: f32,
}

#[derive(Debug, Clone)]
pub struct ActiveRepair {
    pub target_entity: Entity,
    pub repair_rate: f32,
    pub elapsed: f32,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AbortMode {
    #[default]
    None,
    PadAbort,
    Mode1,
    Mode2,
    Mode3,
}

#[derive(Event, Debug, Clone)]
pub struct EmergencyAbortEvent {
    pub abort_mode: AbortMode,
    pub reason: String,
}

#[derive(Component, Debug, Clone)]
pub struct DamageControlSystem {
    pub repair_teams: u32,
    pub max_concurrent_repairs: u32,
    pub repair_efficiency: f32,
    pub fire_suppression_charges: u32,
    pub emergency_separation_ready: bool,
}

impl Default for DamageControlSystem {
    fn default() -> Self {
        Self {
            repair_teams: 3,
            max_concurrent_repairs: 2,
            repair_efficiency: 1.0,
            fire_suppression_charges: 5,
            emergency_separation_ready: true,
        }
    }
}

#[derive(Component)]
pub struct FireSuppressionSystem {
    pub suppressant_remaining_kg: f32,
    pub suppression_radius: f32,
    pub active: bool,
    pub cooldown_timer: Timer,
}

impl Default for FireSuppressionSystem {
    fn default() -> Self {
        Self {
            suppressant_remaining_kg: 100.0,
            suppression_radius: 5.0,
            active: false,
            cooldown_timer: Timer::from_seconds(30.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct EmergencyEscapeSystem {
    pub launch_escape_motor_ready: bool,
    pub tower_jettison_ready: bool,
    pub cm_sm_separation_ready: bool,
    pub drogue_chutes_ready: bool,
    pub main_chutes_ready: bool,
}

impl Default for EmergencyEscapeSystem {
    fn default() -> Self {
        Self {
            launch_escape_motor_ready: true,
            tower_jettison_ready: true,
            cm_sm_separation_ready: true,
            drogue_chutes_ready: true,
            main_chutes_ready: true,
        }
    }
}

fn process_damage_control(
    time: Res<Time>,
    mut damage_control: ResMut<DamageControlState>,
    mut integrity_query: Query<&mut StructuralIntegrity>,
    dc_system_query: Query<&DamageControlSystem>,
) {
    let dt = time.delta_seconds();
    let repair_multiplier = if let Ok(dc) = dc_system_query.get_single() {
        dc.repair_efficiency * dc.repair_teams as f32 * 0.33
    } else {
        1.0
    };
    
    damage_control.active_repairs.retain_mut(|repair| {
        repair.elapsed += dt;
        if let Ok(mut integrity) = integrity_query.get_mut(repair.target_entity) {
            if !integrity.is_destroyed {
                let repair_amount = repair.repair_rate * dt * repair_multiplier;
                integrity.current_integrity = (integrity.current_integrity + repair_amount)
                    .min(integrity.max_integrity);
            }
        }
        repair.elapsed < repair.duration
    });
}

fn handle_emergency_abort(
    mut abort_events: EventReader<EmergencyAbortEvent>,
    mut commands: Commands,
    mut integrity_query: Query<(Entity, &StructuralIntegrity, &Transform)>,
    escape_query: Query<(Entity, &EmergencyEscapeSystem)>,
) {
    for event in abort_events.read() {
        match event.abort_mode {
            AbortMode::PadAbort => {
                for (entity, _, transform) in integrity_query.iter_mut() {
                    if let Ok((_, escape)) = escape_query.get(entity) {
                        if escape.launch_escape_motor_ready {
                            commands.spawn((
                                SpatialBundle {
                                    transform: transform.clone(),
                                    ..default()
                                },
                                ExplosionEffect {
                                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                                    max_radius: 10.0,
                                    intensity: 1000.0,
                                },
                            ));
                        }
                    }
                }
            }
            AbortMode::Mode1 => {
                for (entity, _, transform) in integrity_query.iter_mut() {
                    if let Ok((_, escape)) = escape_query.get(entity) {
                        if escape.cm_sm_separation_ready {
                            commands.spawn((
                                SpatialBundle {
                                    transform: transform.clone(),
                                    ..default()
                                },
                                ExplosionEffect {
                                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                                    max_radius: 5.0,
                                    intensity: 500.0,
                                },
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_fire_suppression(
    time: Res<Time>,
    mut suppression_query: Query<&mut FireSuppressionSystem>,
    mut integrity_query: Query<&mut StructuralIntegrity>,
) {
    for mut suppression in suppression_query.iter_mut() {
        suppression.cooldown_timer.tick(time.delta());
        
        if suppression.active && suppression.suppressant_remaining_kg > 0.0 {
            suppression.suppressant_remaining_kg -= 5.0 * time.delta_seconds();
            suppression.suppressant_remaining_kg = suppression.suppressant_remaining_kg.max(0.0);
            
            for mut integrity in integrity_query.iter_mut() {
                if integrity.current_integrity < integrity.max_integrity && !integrity.is_destroyed {
                    integrity.current_integrity += 10.0 * time.delta_seconds();
                    integrity.current_integrity = integrity.current_integrity.min(integrity.max_integrity);
                }
            }
            
            if suppression.suppressant_remaining_kg <= 0.0 {
                suppression.active = false;
            }
        }
    }
}

fn monitor_structural_integrity(
    time: Res<Time>,
    mut damage_control: ResMut<DamageControlState>,
    integrity_query: Query<(Entity, &StructuralIntegrity)>,
    mut abort_events: EventWriter<EmergencyAbortEvent>,
) {
    let total_integrity: f32 = integrity_query.iter().map(|(_, i)| i.integrity_pct()).sum();
    let count = integrity_query.iter().count() as f32;
    let avg_integrity = if count > 0.0 { total_integrity / count } else { 1.0 };
    
    if avg_integrity < 0.3 && damage_control.abort_mode == AbortMode::None {
        if time.elapsed_seconds() - damage_control.last_alert_time > 5.0 {
            damage_control.last_alert_time = time.elapsed_seconds();
            abort_events.send(EmergencyAbortEvent {
                abort_mode: AbortMode::Mode2,
                reason: "Critical structural damage detected".to_string(),
            });
        }
    }
}
