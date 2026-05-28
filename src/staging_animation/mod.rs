use bevy::prelude::*;

pub struct StagingAnimationPlugin;

impl Plugin for StagingAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StageSeparateEvent>()
            .add_event::<EngineIgnitionEvent>()
            .add_systems(PostStartup, tag_saturn_v_stages)
            .add_systems(Update, animate_stage_separation.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, spawn_ignition_effect.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_engine_plume.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, spawn_separation_particles.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_separation_debris.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

fn tag_saturn_v_stages(
    mut commands: Commands,
    saturn_query: Query<(Entity, &Children), With<crate::spacecraft::PlayerVehicle>>,
    transform_query: Query<&GlobalTransform>,
) {
    for (saturn_entity, children) in saturn_query.iter() {
        if let Ok(saturn_transform) = transform_query.get(saturn_entity) {
            let saturn_y = saturn_transform.translation().y;
            let half_height = crate::config::SATURN_V_TOTAL_HEIGHT * 0.5;
            
            for &child in children.iter() {
                if let Ok(child_transform) = transform_query.get(child) {
                    let local_y = child_transform.translation().y - saturn_y + half_height;
                    
                    let stage = if local_y < crate::config::S_IC_HEIGHT + 0.5 {
                        StageIdentifier::SIC
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + 0.5 {
                        StageIdentifier::SII
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + crate::config::S_IVb_HEIGHT + 0.5 {
                        StageIdentifier::SIVB
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + crate::config::S_IVb_HEIGHT + crate::config::IU_HEIGHT + 0.5 {
                        StageIdentifier::IU
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + crate::config::S_IVb_HEIGHT + crate::config::IU_HEIGHT + crate::config::SLA_HEIGHT + 0.5 {
                        StageIdentifier::SLA
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + crate::config::S_IVb_HEIGHT + crate::config::IU_HEIGHT + crate::config::SLA_HEIGHT + crate::config::SM_HEIGHT + 0.5 {
                        StageIdentifier::SM
                    } else if local_y < crate::config::S_IC_HEIGHT + crate::config::S_II_HEIGHT + crate::config::S_IVb_HEIGHT + crate::config::IU_HEIGHT + crate::config::SLA_HEIGHT + crate::config::SM_HEIGHT + crate::config::CSM_HEIGHT + 0.5 {
                        StageIdentifier::CM
                    } else {
                        StageIdentifier::LES
                    };
                    
                    commands.entity(child).insert(stage);
                }
            }
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageIdentifier {
    SIC,
    SII,
    SIVB,
    IU,
    SLA,
    SM,
    CM,
    LES,
}

#[derive(Component)]
pub struct SeparatingStage {
    pub stage: StageIdentifier,
    pub separation_velocity: Vec3,
    pub separation_progress: f32,
    pub rotation_drift: Vec3,
}

#[derive(Event, Debug, Clone)]
pub struct StageSeparateEvent {
    pub stage: StageIdentifier,
    pub separation_point: Vec3,
    pub rocket_velocity: Vec3,
}

#[derive(Event, Debug, Clone)]
pub struct EngineIgnitionEvent {
    pub position: Vec3,
    pub engine_radius: f32,
    pub engine_count: u32,
}

#[derive(Component)]
pub struct EnginePlume {
    pub timer: Timer,
    pub max_scale: f32,
    pub intensity: f32,
}

#[derive(Component)]
pub struct SeparationParticle {
    pub velocity: Vec3,
    pub lifetime: Timer,
    pub initial_scale: f32,
}

#[derive(Component)]
pub struct SeparationDebris {
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub lifetime: Timer,
}

fn animate_stage_separation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SeparatingStage, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut separation, mut transform) in query.iter_mut() {
        separation.separation_progress += dt * 0.5;
        
        transform.translation += separation.separation_velocity * dt;
        transform.rotation *= Quat::from_euler(
            EulerRot::XYZ,
            separation.rotation_drift.x * dt,
            separation.rotation_drift.y * dt,
            separation.rotation_drift.z * dt,
        );
        
        let scale = 1.0 - separation.separation_progress * 0.3;
        transform.scale = Vec3::splat(scale.max(0.1));
        
        if separation.separation_progress >= 3.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_ignition_effect(
    mut events: EventReader<EngineIgnitionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        for i in 0..event.engine_count {
            let angle = if event.engine_count > 1 {
                (i as f32 / event.engine_count as f32) * std::f32::consts::TAU
            } else {
                0.0
            };
            let offset = if event.engine_count > 1 {
                event.engine_radius * 0.5
            } else {
                0.0
            };
            let x = event.position.x + angle.cos() * offset;
            let z = event.position.z + angle.sin() * offset;
            let y = event.position.y - event.engine_radius * 0.5;
            
            let plume_mesh = meshes.add(Cone {
                radius: event.engine_radius * 1.5,
                height: event.engine_radius * 4.0,
            });
            let plume_mat = materials.add(StandardMaterial {
                base_color: Color::srgba(0.9, 0.7, 0.2, 0.7),
                emissive: LinearRgba::new(2.0, 1.5, 0.3, 8.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });
            
            commands.spawn((
                PbrBundle {
                    mesh: plume_mesh,
                    material: plume_mat,
                    transform: Transform::from_xyz(x, y, z)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
                    ..default()
                },
                EnginePlume {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                    max_scale: 3.0,
                    intensity: 1.0,
                },
            ));
        }
    }
}

fn update_engine_plume(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EnginePlume, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut plume, mut transform) in query.iter_mut() {
        plume.timer.tick(time.delta());
        
        if plume.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        let t = plume.timer.elapsed_secs() / plume.timer.duration().as_secs_f32();
        let scale = if t < 0.2 {
            t / 0.2 * plume.max_scale
        } else {
            plume.max_scale * (1.0 - (t - 0.2) / 0.8)
        };
        transform.scale = Vec3::new(1.0, scale, 1.0);
    }
}

fn spawn_separation_particles(
    mut events: EventReader<StageSeparateEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        let particle_count = 50;
        for _ in 0..particle_count {
            let size = 0.1 + rand::random::<f32>() * 0.3;
            let mesh = meshes.add(Sphere::new(size));
            let mat = materials.add(StandardMaterial {
                base_color: Color::srgba(0.8, 0.8, 0.8, 0.6),
                emissive: LinearRgba::new(0.4, 0.4, 0.4, 1.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });
            
            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
            );
            
            commands.spawn((
                PbrBundle {
                    mesh,
                    material: mat,
                    transform: Transform::from_translation(event.separation_point),
                    ..default()
                },
                SeparationParticle {
                    velocity,
                    lifetime: Timer::from_seconds(1.0 + rand::random::<f32>() * 2.0, TimerMode::Once),
                    initial_scale: size,
                },
            ));
        }
        
        let debris_count = 15;
        for _ in 0..debris_count {
            let size = 0.2 + rand::random::<f32>() * 0.5;
            let mesh = meshes.add(Cuboid::new(size, size * 0.5, size * 0.3));
            let mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.4),
                metallic: 0.6,
                perceptual_roughness: 0.4,
                ..default()
            });
            
            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * 8.0,
                (rand::random::<f32>() - 0.5) * 8.0,
                (rand::random::<f32>() - 0.5) * 8.0,
            );
            let angular = Vec3::new(
                (rand::random::<f32>() - 0.5) * 3.0,
                (rand::random::<f32>() - 0.5) * 3.0,
                (rand::random::<f32>() - 0.5) * 3.0,
            );
            
            commands.spawn((
                PbrBundle {
                    mesh,
                    material: mat,
                    transform: Transform::from_translation(event.separation_point),
                    ..default()
                },
                SeparationDebris {
                    velocity,
                    angular_velocity: angular,
                    lifetime: Timer::from_seconds(3.0 + rand::random::<f32>() * 5.0, TimerMode::Once),
                },
            ));
        }
    }
}

fn update_separation_debris(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SeparationParticle, &mut Transform), Without<SeparationDebris>>,
    mut debris_query: Query<(Entity, &mut SeparationDebris, &mut Transform), Without<SeparationParticle>>,
    mut commands: Commands,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        let t = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
        transform.scale = Vec3::splat(particle.initial_scale * (1.0 - t));
        transform.translation += particle.velocity * dt;
        particle.velocity *= 0.98;
    }
    
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
        debris.velocity.y -= 2.0 * dt;
        debris.velocity *= 0.99;
    }
}
