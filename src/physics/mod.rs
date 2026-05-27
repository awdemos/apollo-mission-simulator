use bevy::prelude::*;
use crate::spacecraft::Spacecraft;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitalParameters::default())
            .add_systems(Update, update_orbital_mechanics)
            .add_systems(Update, apply_gravity);
    }
}

#[derive(Resource)]
pub struct OrbitalParameters {
    pub earth_mass: f64,
    pub moon_mass: f64,
    pub gravitational_constant: f64,
    pub earth_radius: f64,
    pub moon_distance: f64,
}

impl Default for OrbitalParameters {
    fn default() -> Self {
        Self {
            earth_mass: 5.972e24,
            moon_mass: 7.342e22,
            gravitational_constant: 6.674e-11,
            earth_radius: 6371.0,
            moon_distance: 384400.0,
        }
    }
}

#[derive(Component)]
pub struct OrbitalBody {
    pub mass: f64,
    pub velocity: Vec3,
    pub position: Vec3,
}

fn update_orbital_mechanics(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut query: Query<(&mut Transform, &mut OrbitalBody)>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;
    for (mut transform, mut body) in query.iter_mut() {
        let velocity = body.velocity;
        body.position += velocity * dt;
        transform.translation = body.position;
    }
}

fn apply_gravity(
    params: Res<OrbitalParameters>,
    mut query: Query<(&mut OrbitalBody, &Transform)>,
) {
    let earth_pos = Vec3::ZERO;
    
    for (mut body, transform) in query.iter_mut() {
        let to_earth = earth_pos - transform.translation;
        let distance = to_earth.length().max(1.0);
        
        let g = (params.gravitational_constant * params.earth_mass) as f32;
        let force_magnitude = g / (distance * distance);
        let force = to_earth.normalize() * force_magnitude * 1e-6;
        
        body.velocity += force * 0.016;
    }
}

pub fn calculate_orbital_velocity(
    params: &OrbitalParameters,
    altitude_km: f64,
) -> f64 {
    let r = (params.earth_radius + altitude_km) * 1000.0;
    ((params.gravitational_constant * params.earth_mass) / r).sqrt()
}

pub fn calculate_escape_velocity(
    params: &OrbitalParameters,
    altitude_km: f64,
) -> f64 {
    let r = (params.earth_radius + altitude_km) * 1000.0;
    (2.0 * params.gravitational_constant * params.earth_mass / r).sqrt()
}
