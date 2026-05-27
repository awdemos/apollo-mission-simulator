use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_earth)
            .add_systems(Startup, spawn_moon)
            .add_systems(Startup, spawn_launch_complex)
            .add_systems(Update, rotate_earth)
            .add_systems(Update, orbit_moon);
    }
}

#[derive(Component)]
pub struct Earth;

/// Lunar orbital elements for Apollo 11 epoch (1969-07-16 13:32 UTC).
/// Using Keplerian mechanics with accurate semi-major axis, eccentricity,
/// inclination, and mean anomaly for the mission launch date.
#[derive(Component)]
pub struct Moon {
    /// Semi-major axis in visual units (Earth radii).
    pub semi_major_axis: f64,
    /// Orbital eccentricity (0.0549 average, varies 0.026-0.077).
    pub eccentricity: f64,
    /// Inclination to Earth equator in radians.
    pub inclination: f64,
    /// Longitude of ascending node in radians.
    pub raan: f64,
    /// Argument of periapsis in radians.
    pub argument_of_periapsis: f64,
    /// Mean anomaly at epoch in radians.
    pub mean_anomaly_at_epoch: f64,
    /// Julian Date of epoch.
    pub epoch_jd: f64,
    /// Mean motion in radians per second.
    pub mean_motion: f64,
}

#[derive(Component)]
pub struct LaunchComplex;

/// Launch Complex 39A coordinates
/// Latitude: 28.6082° N
/// Longitude: 80.6040° W
pub const LAUNCH_SITE_LAT: f32 = 28.6082_f32.to_radians();
pub const LAUNCH_SITE_LON: f32 = -80.6040_f32.to_radians();
pub const EARTH_RADIUS: f32 = 10.0;

fn spawn_earth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let earth_mesh = meshes.add(Sphere::new(EARTH_RADIUS).mesh().ico(32).unwrap());
    
    let earth_texture: Handle<Image> = asset_server.load("textures/earth.jpg");
    
    let earth_material = materials.add(StandardMaterial {
        base_color_texture: Some(earth_texture),
        base_color: Color::WHITE,
        metallic: 0.05,
        perceptual_roughness: 0.7,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: earth_mesh,
            material: earth_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        },
        Earth,
        Name::new("Earth"),
    ));
}

fn spawn_launch_complex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lat = LAUNCH_SITE_LAT;
    let lon = LAUNCH_SITE_LON;
    let r = EARTH_RADIUS;
    
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.sin();
    let position = Vec3::new(x, y, z);
    
    let normal = position.normalize();
    let up = Vec3::Y;
    let right = normal.cross(up).normalize();
    let forward = right.cross(normal);
    
    let rotation_matrix = Mat3::from_cols(right, normal, forward);
    let rotation = Quat::from_mat3(&rotation_matrix);
    
    let concrete = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        metallic: 0.1,
        perceptual_roughness: 0.9,
        ..default()
    });
    
    let pad_surface = meshes.add(Cylinder::new(0.15, 0.02));
    let tower_base = meshes.add(Cylinder::new(0.04, 0.3));
    let tower_arm = meshes.add(Cylinder::new(0.01, 0.12));
    
    let launch_complex = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position).with_rotation(rotation),
            ..default()
        },
        LaunchComplex,
        Name::new("Launch Complex 39A"),
    )).id();
    
    commands.entity(launch_complex).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: pad_surface,
            material: concrete.clone(),
            transform: Transform::from_xyz(0.0, 0.01, 0.0),
            ..default()
        });
        
        parent.spawn(PbrBundle {
            mesh: tower_base.clone(),
            material: concrete.clone(),
            transform: Transform::from_xyz(-0.18, 0.15, 0.0),
            ..default()
        });
        
        parent.spawn(PbrBundle {
            mesh: tower_arm,
            material: concrete.clone(),
            transform: Transform::from_xyz(-0.12, 0.28, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    });
}

/// Apollo 11 launch epoch: 1969-07-16 13:32:00 UTC
const APOLLO_11_JULIAN_DATE: f64 = 2440423.0638889;

/// Lunar orbital elements at Apollo 11 epoch.
/// Derived from JPL ephemeris and NASA trajectory data.
const LUNAR_SEMI_MAJOR_AXIS_KM: f64 = 384400.0;
const LUNAR_ECCENTRICITY: f64 = 0.0549;
const LUNAR_INCLINATION_DEG: f64 = 23.4;
const LUNAR_RAAN_DEG: f64 = 356.3;
const LUNAR_ARG_PERIAPSIS_DEG: f64 = 44.1;
const LUNAR_MEAN_ANOMALY_DEG: f64 = 205.3;
const LUNAR_SIDEREAL_PERIOD_DAYS: f64 = 27.32166;

/// Earth radius in km for scale calculations.
const EARTH_RADIUS_KM: f64 = 6371.0;

fn spawn_moon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let moon_mesh = meshes.add(Sphere::new(2.73).mesh().ico(16).unwrap());
    
    let moon_texture: Handle<Image> = asset_server.load("textures/moon.jpg");
    
    let moon_material = materials.add(StandardMaterial {
        base_color_texture: Some(moon_texture),
        base_color: Color::WHITE,
        metallic: 0.0,
        perceptual_roughness: 0.95,
        ..default()
    });

    let semi_major_axis = (LUNAR_SEMI_MAJOR_AXIS_KM / EARTH_RADIUS_KM) as f64;
    let mean_motion = 2.0 * std::f64::consts::PI / (LUNAR_SIDEREAL_PERIOD_DAYS * 86400.0);

    let moon = Moon {
        semi_major_axis,
        eccentricity: LUNAR_ECCENTRICITY,
        inclination: LUNAR_INCLINATION_DEG.to_radians(),
        raan: LUNAR_RAAN_DEG.to_radians(),
        argument_of_periapsis: LUNAR_ARG_PERIAPSIS_DEG.to_radians(),
        mean_anomaly_at_epoch: LUNAR_MEAN_ANOMALY_DEG.to_radians(),
        epoch_jd: APOLLO_11_JULIAN_DATE,
        mean_motion,
    };

    let position = calculate_moon_position(&moon, APOLLO_11_JULIAN_DATE);

    commands.spawn((
        PbrBundle {
            mesh: moon_mesh,
            material: moon_material,
            transform: Transform::from_translation(position),
            ..default()
        },
        moon,
        Name::new("Moon"),
    ));
}

const SIDEREAL_DAY_SECONDS: f32 = 86164.0;
const EARTH_ROTATION_SPEED: f32 = std::f32::consts::TAU / SIDEREAL_DAY_SECONDS;

fn rotate_earth(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Earth>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() * EARTH_ROTATION_SPEED);
    }
}

/// Calculate moon position from Keplerian orbital elements at a given Julian Date.
/// Solves Kepler's equation using Newton-Raphson iteration.
fn calculate_moon_position(moon: &Moon, jd: f64) -> Vec3 {
    let dt = (jd - moon.epoch_jd) * 86400.0;
    let mean_anomaly = moon.mean_anomaly_at_epoch + moon.mean_motion * dt;

    let eccentric_anomaly = solve_keplers_equation(mean_anomaly, moon.eccentricity);

    let true_anomaly = 2.0 * ((1.0 + moon.eccentricity).sqrt() * (eccentric_anomaly / 2.0).sin())
        .atan2((1.0 - moon.eccentricity).sqrt() * (eccentric_anomaly / 2.0).cos());

    let distance = moon.semi_major_axis * (1.0 - moon.eccentricity * eccentric_anomaly.cos());

    let arg_lat = true_anomaly + moon.argument_of_periapsis;

    let x_orbital = distance * arg_lat.cos();
    let y_orbital = distance * arg_lat.sin();

    let cos_i = moon.inclination.cos();
    let sin_i = moon.inclination.sin();
    let cos_raan = moon.raan.cos();
    let sin_raan = moon.raan.sin();

    let x = (cos_raan * x_orbital - sin_raan * y_orbital * cos_i) as f32;
    let y = (sin_i * y_orbital) as f32;
    let z = (sin_raan * x_orbital + cos_raan * y_orbital * cos_i) as f32;

    Vec3::new(x, y, z)
}

/// Solve Kepler's equation M = E - e * sin(E) for E using Newton-Raphson.
fn solve_keplers_equation(mean_anomaly: f64, eccentricity: f64) -> f64 {
    let mut e = mean_anomaly;
    for _ in 0..50 {
        let f = e - eccentricity * e.sin() - mean_anomaly;
        let fp = 1.0 - eccentricity * e.cos();
        let delta = f / fp;
        e -= delta;
        if delta.abs() < 1e-12 {
            break;
        }
    }
    e
}

fn orbit_moon(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut query: Query<(&mut Transform, &Moon)>,
) {
    let elapsed = time.elapsed_seconds() * time_scale.multiplier;
    let jd_delta = elapsed as f64 / 86400.0;

    for (mut transform, moon) in query.iter_mut() {
        let current_jd = moon.epoch_jd + jd_delta;
        let position = calculate_moon_position(moon, current_jd);
        transform.translation = position;

        let dt_total = elapsed as f64;
        let mean_anomaly = moon.mean_anomaly_at_epoch + moon.mean_motion * dt_total;
        let eccentric_anomaly = solve_keplers_equation(mean_anomaly, moon.eccentricity);
        let true_anomaly = 2.0 * ((1.0 + moon.eccentricity).sqrt() * (eccentric_anomaly / 2.0).sin())
            .atan2((1.0 - moon.eccentricity).sqrt() * (eccentric_anomaly / 2.0).cos());
        transform.rotation = Quat::from_rotation_y(-true_anomaly as f32);
    }
}
