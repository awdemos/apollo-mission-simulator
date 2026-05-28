use bevy::prelude::*;

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_solar_system)
            .add_systems(Update, update_planet_orbits.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Planet {
    pub name: &'static str,
    pub orbital_period_days: f32,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub inclination_deg: f32,
    pub radius: f32,
    pub color: Color,
    pub emissive: LinearRgba,
    pub epoch_anomaly_deg: f32,
}

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_mesh = meshes.add(Sphere::new(5.0));
    let sun_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.95, 0.8),
        emissive: LinearRgba::new(1.0, 0.9, 0.6, 5.0),
        unlit: true,
        ..default()
    });
    
    commands.spawn((
        PbrBundle {
            mesh: sun_mesh,
            material: sun_mat,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Sun,
        Name::new("Sun"),
    ));
    
    let planets = [
        Planet {
            name: "Mercury",
            orbital_period_days: 88.0,
            semi_major_axis: 25.0,
            eccentricity: 0.206,
            inclination_deg: 7.0,
            radius: 0.15,
            color: Color::srgb(0.7, 0.7, 0.7),
            emissive: LinearRgba::new(0.35, 0.35, 0.35, 0.5),
            epoch_anomaly_deg: 180.0,
        },
        Planet {
            name: "Venus",
            orbital_period_days: 225.0,
            semi_major_axis: 45.0,
            eccentricity: 0.007,
            inclination_deg: 3.4,
            radius: 0.35,
            color: Color::srgb(0.9, 0.8, 0.5),
            emissive: LinearRgba::new(0.45, 0.4, 0.25, 0.5),
            epoch_anomaly_deg: 120.0,
        },
        Planet {
            name: "Earth",
            orbital_period_days: 365.25,
            semi_major_axis: 62.5,
            eccentricity: 0.017,
            inclination_deg: 0.0,
            radius: 0.38,
            color: Color::srgb(0.2, 0.4, 0.8),
            emissive: LinearRgba::new(0.1, 0.2, 0.4, 0.3),
            epoch_anomaly_deg: 0.0,
        },
        Planet {
            name: "Mars",
            orbital_period_days: 687.0,
            semi_major_axis: 95.0,
            eccentricity: 0.094,
            inclination_deg: 1.9,
            radius: 0.2,
            color: Color::srgb(0.8, 0.3, 0.1),
            emissive: LinearRgba::new(0.4, 0.15, 0.05, 0.5),
            epoch_anomaly_deg: 60.0,
        },
        Planet {
            name: "Jupiter",
            orbital_period_days: 4333.0,
            semi_major_axis: 320.0,
            eccentricity: 0.049,
            inclination_deg: 1.3,
            radius: 2.5,
            color: Color::srgb(0.8, 0.7, 0.5),
            emissive: LinearRgba::new(0.4, 0.35, 0.25, 0.5),
            epoch_anomaly_deg: 30.0,
        },
        Planet {
            name: "Saturn",
            orbital_period_days: 10759.0,
            semi_major_axis: 588.0,
            eccentricity: 0.057,
            inclination_deg: 2.5,
            radius: 2.1,
            color: Color::srgb(0.9, 0.85, 0.6),
            emissive: LinearRgba::new(0.45, 0.42, 0.3, 0.5),
            epoch_anomaly_deg: 240.0,
        },
        Planet {
            name: "Uranus",
            orbital_period_days: 30687.0,
            semi_major_axis: 1180.0,
            eccentricity: 0.046,
            inclination_deg: 0.8,
            radius: 1.2,
            color: Color::srgb(0.4, 0.8, 0.9),
            emissive: LinearRgba::new(0.2, 0.4, 0.45, 0.5),
            epoch_anomaly_deg: 300.0,
        },
        Planet {
            name: "Neptune",
            orbital_period_days: 60190.0,
            semi_major_axis: 1850.0,
            eccentricity: 0.011,
            inclination_deg: 1.8,
            radius: 1.15,
            color: Color::srgb(0.2, 0.4, 0.9),
            emissive: LinearRgba::new(0.1, 0.2, 0.45, 0.5),
            epoch_anomaly_deg: 90.0,
        },
    ];
    
    for planet in planets {
        let mesh = meshes.add(Sphere::new(planet.radius));
        let material = materials.add(StandardMaterial {
            base_color: planet.color,
            emissive: planet.emissive,
            unlit: true,
            ..default()
        });
        
        let planet_name = planet.name;
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(planet.semi_major_axis, 0.0, 0.0),
                ..default()
            },
            planet,
            Name::new(planet_name),
        ));
    }
}

fn update_planet_orbits(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut query: Query<(&Planet, &mut Transform)>,
) {
    let dt_days = time.delta_seconds() * time_scale.multiplier / 86400.0;
    
    for (planet, mut transform) in query.iter_mut() {
        let mean_motion = 360.0 / planet.orbital_period_days;
        let mut anomaly = planet.epoch_anomaly_deg + mean_motion * dt_days;
        anomaly = anomaly % 360.0;
        let anomaly_rad = anomaly.to_radians();
        
        let r = planet.semi_major_axis * (1.0 - planet.eccentricity * planet.eccentricity)
            / (1.0 + planet.eccentricity * anomaly_rad.cos());
        
        let inclination_rad = planet.inclination_deg.to_radians();
        let x = r * anomaly_rad.cos();
        let z = r * anomaly_rad.sin();
        let y = z * inclination_rad.sin();
        let z_adjusted = z * inclination_rad.cos();
        
        transform.translation = Vec3::new(x, y, z_adjusted);
    }
}
