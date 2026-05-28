use bevy::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeatherState::apollo_11())
            .add_systems(Startup, spawn_weather)
            .add_systems(Update, animate_wind_particles.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, animate_heat_haze.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, drift_clouds.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct WeatherState {
    pub cloud_cover: f32,
    pub temperature_f: f32,
    pub wind_speed_knots: f32,
    pub wind_direction_deg: f32,
    pub visibility_miles: f32,
}

impl WeatherState {
    pub fn apollo_11() -> Self {
        Self {
            cloud_cover: 0.15,
            temperature_f: 85.0,
            wind_speed_knots: 12.0,
            wind_direction_deg: 135.0,
            visibility_miles: 7.0,
        }
    }

    pub fn wind_direction_vector(&self) -> Vec3 {
        let rad = self.wind_direction_deg.to_radians();
        Vec3::new(rad.sin(), 0.0, rad.cos())
    }

    pub fn wind_speed_mps(&self) -> f32 {
        self.wind_speed_knots * 0.514444
    }
}

#[derive(Component)]
pub struct CloudCluster;

#[derive(Component)]
pub struct WindParticle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

#[derive(Component)]
pub struct HeatHaze;

#[derive(Component)]
pub struct CloudDrift {
    pub base_pos: Vec3,
    pub drift_speed: f32,
    pub phase: f32,
}

fn spawn_weather(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    weather: Res<WeatherState>,
) {
    spawn_clouds(&mut commands, &mut meshes, &mut materials, &weather);
    spawn_wind_particles(&mut commands, &mut meshes, &mut materials, &weather);
    spawn_heat_haze(&mut commands, &mut meshes, &mut materials);
}

fn spawn_clouds(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    weather: &WeatherState,
) {
    let cloud_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.92),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    let mut rng = StdRng::seed_from_u64(1969);
    let num_clouds = (weather.cloud_cover * 8.0).max(1.0) as i32;

    for _ in 0..num_clouds {
        let altitude = 35.0 + rng.gen::<f32>() * 25.0;
        let x = (rng.gen::<f32>() - 0.5) * 150.0;
        let z = (rng.gen::<f32>() - 0.5) * 150.0;
        let base_pos = Vec3::new(x, altitude, z);

        let cluster_size = 3.0 + rng.gen::<f32>() * 4.0;
        let num_puffs = 6 + rng.gen::<u32>() % 6;

        let cluster = commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(base_pos),
                ..default()
            },
            CloudCluster,
            CloudDrift {
                base_pos,
                drift_speed: 0.15 + rng.gen::<f32>() * 0.25,
                phase: rng.gen::<f32>() * std::f32::consts::TAU,
            },
        )).id();

        commands.entity(cluster).with_children(|parent| {
            for _ in 0..num_puffs {
                let puff_radius = 1.0 + rng.gen::<f32>() * 1.5;
                let px = (rng.gen::<f32>() - 0.5) * cluster_size;
                let py = (rng.gen::<f32>() - 0.5) * cluster_size * 0.5;
                let pz = (rng.gen::<f32>() - 0.5) * cluster_size;
                let puff_mesh = meshes.add(Sphere::new(puff_radius).mesh().ico(5).unwrap());

                let scale_y = 0.7 + rng.gen::<f32>() * 0.5;
                parent.spawn(PbrBundle {
                    mesh: puff_mesh,
                    material: cloud_mat.clone(),
                    transform: Transform::from_xyz(px, py, pz).with_scale(Vec3::new(1.0, scale_y, 1.0)),
                    ..default()
                });
            }
        });
    }
}

fn spawn_wind_particles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    weather: &WeatherState,
) {
    let particle_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.75, 0.70, 0.60, 0.4),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let wind_dir = weather.wind_direction_vector();
    let wind_speed = weather.wind_speed_mps();
    let mut rng = StdRng::seed_from_u64(711969);

    for _ in 0..60 {
        let x = (rng.gen::<f32>() - 0.5) * 40.0;
        let z = (rng.gen::<f32>() - 0.5) * 40.0;
        let y = rng.gen::<f32>() * 1.5;
        let pos = Vec3::new(x, y, z);

        let speed_var = 0.8 + rng.gen::<f32>() * 0.4;
        let velocity = wind_dir * wind_speed * speed_var;
        let lifetime = 3.0 + rng.gen::<f32>() * 5.0;

        let particle_mesh = meshes.add(Sphere::new(0.03 + rng.gen::<f32>() * 0.04).mesh().ico(2).unwrap());

        commands.spawn((
            PbrBundle {
                mesh: particle_mesh,
                material: particle_mat.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            WindParticle {
                velocity,
                lifetime,
                max_lifetime: lifetime,
            },
        ));
    }
}

fn spawn_heat_haze(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let haze_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.85, 0.7, 0.03),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    for i in 0..3 {
        let haze_mesh = meshes.add(Cylinder::new(8.0 + i as f32 * 3.0, 0.05));
        commands.spawn((
            PbrBundle {
                mesh: haze_mesh,
                material: haze_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.5 + i as f32 * 0.3, 0.0),
                ..default()
            },
            HeatHaze,
        ));
    }
}

fn animate_wind_particles(
    time: Res<Time>,
    weather: Res<WeatherState>,
    mut query: Query<(&mut Transform, &mut WindParticle)>,
) {
    let dt = time.delta_seconds();
    let wind_dir = weather.wind_direction_vector();
    let wind_speed = weather.wind_speed_mps();

    for (mut transform, mut particle) in query.iter_mut() {
        particle.lifetime -= dt;

        if particle.lifetime <= 0.0 {
            let mut rng = StdRng::seed_from_u64(
                (transform.translation.x.abs() * 1000.0) as u64
                    + (transform.translation.z.abs() * 100.0) as u64,
            );
            let reset_dist = 20.0 + rng.gen::<f32>() * 10.0;
            let reset_offset = wind_dir * -reset_dist;
            let spread = 15.0;
            transform.translation = Vec3::new(
                reset_offset.x + (rng.gen::<f32>() - 0.5) * spread,
                rng.gen::<f32>() * 1.5,
                reset_offset.z + (rng.gen::<f32>() - 0.5) * spread,
            );
            particle.lifetime = particle.max_lifetime;
            let speed_var = 0.8 + rng.gen::<f32>() * 0.4;
            particle.velocity = wind_dir * wind_speed * speed_var;
        } else {
            transform.translation += particle.velocity * dt;
            let age_ratio = 1.0 - (particle.lifetime / particle.max_lifetime);
            let fade = if age_ratio < 0.2 {
                age_ratio / 0.2
            } else if age_ratio > 0.8 {
                (1.0 - age_ratio) / 0.2
            } else {
                1.0
            };
            transform.scale = Vec3::splat(fade.max(0.1));
        }
    }
}

fn animate_heat_haze(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<HeatHaze>>,
) {
    let t = time.elapsed_seconds();
    for (i, mut transform) in query.iter_mut().enumerate() {
        let offset = (t * 0.3 + i as f32 * 1.5).sin() * 0.15;
        transform.translation.x = offset;
        let scale_pulse = 1.0 + (t * 0.5 + i as f32).sin() * 0.02;
        transform.scale = Vec3::new(scale_pulse, 1.0, scale_pulse);
    }
}

fn drift_clouds(
    time: Res<Time>,
    weather: Res<WeatherState>,
    mut query: Query<(&mut Transform, &CloudDrift)>,
) {
    let dt = time.delta_seconds();
    let wind_dir = weather.wind_direction_vector();

    for (mut transform, drift) in query.iter_mut() {
        let drift_offset = wind_dir * drift.drift_speed * dt;
        let bob = (time.elapsed_seconds() * 0.1 + drift.phase).sin() * 0.02 * dt;
        transform.translation += drift_offset + Vec3::Y * bob;

        if transform.translation.x.abs() > 150.0 || transform.translation.z.abs() > 150.0 {
            transform.translation = drift.base_pos;
        }
    }
}
