use bevy::prelude::*;
use crate::spacecraft::{Spacecraft, LaunchController, LaunchState, LaunchStage};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitalParameters::default())
            .add_systems(Update, update_orbital_mechanics.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, apply_gravity.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, player_launch_input.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_rocket_physics.run_if(in_state(crate::game_state::AppState::InGame)));
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
        // Game scale: Earth radius = 10 units, surface gravity = 0.04 units/s²
        // G*M = g * r² = 0.04 * 100 = 4.0 (in game units)
        Self {
            earth_mass: 400.0,
            moon_mass: 4.92,
            gravitational_constant: 0.01,
            earth_radius: 10.0,
            moon_distance: 603.4,
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
        let force = to_earth.normalize() * force_magnitude;
        
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

fn player_launch_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LaunchController>,
) {
    for mut controller in query.iter_mut() {
        if controller.state == LaunchState::OnPad && keyboard.just_pressed(KeyCode::Space) {
            controller.state = LaunchState::Countdown;
            info!("LAUNCH: Swing arm retraction initiated");
        }
    }
}

fn update_rocket_physics(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut query: Query<(&mut Transform, &mut Spacecraft, &mut LaunchController)>,
    params: Res<OrbitalParameters>,
    mission_state: Res<crate::mission::MissionState>,
) {
    let dt = (time.delta_seconds() * time_scale.multiplier) as f64;
    for (mut transform, mut spacecraft, mut controller) in query.iter_mut() {
        if spacecraft.vessel_type != crate::spacecraft::VesselType::SaturnV {
            continue;
        }
        match controller.state {
            crate::spacecraft::LaunchState::OnPad | crate::spacecraft::LaunchState::Countdown => {}
            _ => {
                controller.mission_time += dt as f32;
            }
        }
        let t = controller.mission_time as f64;
        let pos = transform.translation;
        let up = pos.normalize();
        let altitude = pos.length() as f64 - params.earth_radius - spacecraft.height_offset as f64;
        spacecraft.altitude = altitude.max(0.0) as f32;
        let surface_g = 0.04f32;
        let g = surface_g * (params.earth_radius as f32 / pos.length()).powi(2);
        match controller.state {
            LaunchState::OnPad | LaunchState::Countdown => {
            }
            LaunchState::Ignition => {
                controller.state = LaunchState::Liftoff;
                controller.stage = LaunchStage::SIC_Burn;
            }
            LaunchState::Liftoff | LaunchState::InFlight => {
                let (net_accel, pitch_deg) = get_apollo11_guidance(t);
                let forward = if t < 12.0 {
                    up
                } else {
                    let east = up.cross(Vec3::Y).normalize();
                    let pitch = pitch_deg.to_radians();
                    up * pitch.cos() + east * pitch.sin()
                };
                let accel = forward * net_accel - up * g;
                spacecraft.velocity += accel * dt as f32;
                if t >= 12.0 && spacecraft.velocity.length() > 1.0 {
                    let look_dir = spacecraft.velocity.normalize();
                    let current_up = transform.rotation * Vec3::Y;
                    let new_up = current_up.lerp(up, 0.02);
                    let target_pos = transform.translation + look_dir * 100.0;
                    let _ = transform.looking_at(target_pos, new_up);
                }
                if t >= 168.0 && controller.stage == LaunchStage::SIC_Burn {
                    controller.stage = LaunchStage::SII_Burn;
                }
                if t >= 480.0 && controller.stage == LaunchStage::SII_Burn {
                    controller.stage = LaunchStage::SIVB_Burn1;
                }
                if t >= 680.0 {
                    controller.state = LaunchState::OrbitInsertion;
                    controller.stage = LaunchStage::Coast;
                }
            }
            LaunchState::OrbitInsertion => {
                let r = pos.length();
                let v_circ = (params.gravitational_constant * params.earth_mass / r as f64).sqrt() as f32;
                let east = up.cross(Vec3::Y).normalize();
                let target_vel = east * v_circ;
                spacecraft.velocity = spacecraft.velocity.lerp(target_vel, 0.05);
            }
            _ => {}
        }
    }
}

fn get_apollo11_guidance(t: f64) -> (f32, f32) {
    if t < 168.0 {
        let net = 0.08 + t as f32 * 0.0005;
        let pitch = if t < 12.0 { 90.0 } else { 90.0 - (t - 12.0) * 0.55 };
        (net.min(0.25), pitch.clamp(15.0, 90.0) as f32)
    } else if t < 480.0 {
        let net = 0.15 + (t - 168.0) as f32 * 0.0002;
        let pitch = 35.0 - (t - 168.0) * 0.04;
        (net.min(0.30), pitch.clamp(5.0, 45.0) as f32)
    } else if t < 680.0 {
        let net = 0.10 + (t - 480.0) as f32 * 0.0001;
        (net.min(0.20), 0.0)
    } else {
        (0.0, 0.0)
    }
}
