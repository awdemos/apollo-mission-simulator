use bevy::prelude::*;

pub struct EngineEffectsPlugin;

impl Plugin for EngineEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_engine_exhaust.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_exhaust_particles.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Component)]
pub struct ExhaustParticle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub initial_scale: f32,
}

fn spawn_engine_exhaust(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rocket_query: Query<(&crate::spacecraft::LaunchController, &GlobalTransform)>,
) {
    let particle_mesh = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let core_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.9, 0.6, 0.9),
        emissive: LinearRgba::new(3.0, 2.0, 0.5, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let smoke_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.8, 0.8, 0.4),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for (controller, global_transform) in rocket_query.iter() {
        let is_firing = matches!(
            controller.state,
            crate::spacecraft::LaunchState::Ignition
                | crate::spacecraft::LaunchState::Liftoff
                | crate::spacecraft::LaunchState::InFlight
        );

        if !is_firing {
            continue;
        }

        let (engine_count, engine_radius, stage_y_offset) = match controller.stage {
            crate::spacecraft::LaunchStage::SIC_Burn => {
                let offset = -crate::config::SATURN_V_TOTAL_HEIGHT * 0.5
                    + 0.56f32 * 1.2f32 * crate::config::SATURN_V_SCALE * 0.5f32;
                (5u32, 0.183f32, offset)
            }
            crate::spacecraft::LaunchStage::SII_Burn => {
                let offset = -crate::config::SATURN_V_TOTAL_HEIGHT * 0.5
                    + crate::config::S_IC_HEIGHT
                    + 0.3f32 * crate::config::SATURN_V_SCALE
                    + crate::config::S_II_HEIGHT
                    - 0.34f32 * crate::config::SATURN_V_SCALE * 0.3f32;
                (5u32, 0.10f32, offset)
            }
            crate::spacecraft::LaunchStage::SIVB_Burn1 => {
                let offset = -crate::config::SATURN_V_TOTAL_HEIGHT * 0.5
                    + crate::config::S_IC_HEIGHT
                    + 0.3f32 * crate::config::SATURN_V_SCALE
                    + crate::config::S_II_HEIGHT
                    + 0.25f32 * crate::config::SATURN_V_SCALE
                    + crate::config::S_IVb_HEIGHT
                    - 0.34f32 * crate::config::SATURN_V_SCALE * 0.3f32;
                (1u32, 0.10f32, offset)
            }
            _ => continue,
        };

        let rocket_pos = global_transform.translation();
        let (_, rocket_rot, _) = global_transform.to_scale_rotation_translation();
        let up = rocket_rot * Vec3::Y;

        for _ in 0..3 {
            for i in 0..engine_count {
                let angle = if engine_count > 1 {
                    (i as f32 / engine_count as f32) * std::f32::consts::TAU
                } else {
                    0.0
                };
                let offset = if engine_count > 1 {
                    engine_radius * 2.5
                } else {
                    0.0
                };
                let local_x = angle.cos() * offset;
                let local_z = angle.sin() * offset;
                let local_pos = Vec3::new(local_x, stage_y_offset, local_z);
                let world_pos = rocket_pos + rocket_rot * local_pos;

                let is_core = i == 0 || engine_count == 1;
                let scale = if is_core {
                    engine_radius * (1.5 + rand::random::<f32>() * 1.5)
                } else {
                    engine_radius * (1.0 + rand::random::<f32>() * 1.0)
                };

                let velocity = -up * (8.0 + rand::random::<f32>() * 12.0)
                    + Vec3::new(
                        (rand::random::<f32>() - 0.5) * 4.0,
                        (rand::random::<f32>() - 0.5) * 2.0,
                        (rand::random::<f32>() - 0.5) * 4.0,
                    );

                let mat = if is_core && rand::random::<f32>() < 0.6 {
                    core_mat.clone()
                } else {
                    smoke_mat.clone()
                };

                commands.spawn((
                    PbrBundle {
                        mesh: particle_mesh.clone(),
                        material: mat,
                        transform: Transform::from_translation(world_pos)
                            .with_scale(Vec3::splat(scale)),
                        ..default()
                    },
                    ExhaustParticle {
                        velocity,
                        lifetime: 0.6 + rand::random::<f32>() * 1.2,
                        max_lifetime: 0.6 + rand::random::<f32>() * 1.2,
                        initial_scale: scale,
                    },
                ));
            }
        }
    }
}

fn update_exhaust_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExhaustParticle, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.lifetime -= dt;
        transform.translation += particle.velocity * dt;
        transform.scale = Vec3::splat(
            particle.initial_scale * (1.0 + (particle.max_lifetime - particle.lifetime) * 3.0),
        );

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
