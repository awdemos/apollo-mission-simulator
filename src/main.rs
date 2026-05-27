mod config;
mod spacecraft;
mod agc;
mod mission;
mod world;
mod sky;
mod ui;
mod physics;
mod audio;
mod communications;
mod systems;
mod planning;
mod virtual_agc;
mod faults;
mod panels;
mod panel_wiring;

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Apollo Mission Simulator".to_string(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CameraMode>()
        .init_resource::<TimeScale>()
        .add_plugins(sky::SkyPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(spacecraft::SpacecraftPlugin)
        .add_plugins(agc::AgcPlugin)
        .add_plugins(mission::MissionPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(communications::CommunicationsPlugin)
        .add_plugins(systems::SystemsPlugin)
        .add_plugins(planning::PlanningPlugin)
        .add_plugins(faults::FaultsPlugin)
        .add_plugins(panels::PanelsPlugin)
        .add_plugins(panel_wiring::PanelWiringPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(audio::AudioPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lighting)
        .add_systems(Update, camera_controller)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(50.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera {
            radius: 100.0,
            theta: 45f32.to_radians(),
            phi: 60f32.to_radians(),
            target: Vec3::ZERO,
            exterior_offset: Vec3::new(20.0, 10.0, 20.0),
        },
        InteriorCamera {
            yaw: 0.0,
            pitch: 0.0,
            position_offset: Vec3::new(0.0, 0.35, 0.2),
        },
    ));
}

fn setup_lighting(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.02, 0.02, 0.04),
        brightness: 0.1,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(200.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    #[default]
    Interior,
    Exterior,
    Free,
}

impl CameraMode {
    pub fn next(&self) -> Self {
        match self {
            CameraMode::Interior => CameraMode::Exterior,
            CameraMode::Exterior => CameraMode::Free,
            CameraMode::Free => CameraMode::Interior,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            CameraMode::Exterior => "EXTERIOR",
            CameraMode::Interior => "INTERIOR",
            CameraMode::Free => "FREE",
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct TimeScale {
    pub multiplier: f32,
}

impl Default for TimeScale {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

impl TimeScale {
    pub const MIN: f32 = 0.25;
    pub const MAX: f32 = 100.0;
    pub const STEP: f32 = 0.25;
    
    pub fn increase(&mut self) {
        self.multiplier = (self.multiplier + Self::STEP).min(Self::MAX);
    }
    
    pub fn decrease(&mut self) {
        self.multiplier = (self.multiplier - Self::STEP).max(Self::MIN);
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
    pub target: Vec3,
    pub exterior_offset: Vec3,
}

#[derive(Component)]
pub struct InteriorCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub position_offset: Vec3,
}

pub fn camera_controller(
    camera_mode: Res<CameraMode>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrbitCamera, &mut InteriorCamera), With<Camera3d>>,
    spacecraft_query: Query<&Transform, (With<crate::spacecraft::Spacecraft>, Without<Camera3d>)>,
    mut windows: Query<&mut Window>,
) {
    let orbit_speed = 0.005;
    let zoom_speed = 2.0;
    let look_speed = crate::config::CAMERA_LOOK_SPEED;
    let move_speed = crate::config::CAMERA_MOVE_SPEED;

    let motions: Vec<_> = mouse_motion.read().collect();
    let wheels: Vec<_> = mouse_wheel.read().collect();

    for (mut transform, mut orbit, mut interior) in query.iter_mut() {
        match *camera_mode {
            CameraMode::Free => {
                let is_orbiting = mouse_button.pressed(MouseButton::Middle)
                    || (keyboard.pressed(KeyCode::ControlLeft) && mouse_button.pressed(MouseButton::Left));

                if is_orbiting {
                    for motion in &motions {
                        orbit.theta -= motion.delta.x * orbit_speed;
                        orbit.phi = (orbit.phi + motion.delta.y * orbit_speed)
                            .clamp(5f32.to_radians(), 175f32.to_radians());
                    }
                }

                let zoom_input: f32 = wheels.iter().map(|w| w.y).sum();
                if zoom_input != 0.0 {
                    orbit.radius -= zoom_input * zoom_speed;
                    orbit.radius = orbit.radius.clamp(15.0, 2000.0);
                }

                let x = orbit.radius * orbit.phi.sin() * orbit.theta.cos();
                let y = orbit.radius * orbit.phi.cos();
                let z = orbit.radius * orbit.phi.sin() * orbit.theta.sin();
                transform.translation = orbit.target + Vec3::new(x, y, z);
                transform.look_at(orbit.target, Vec3::Y);
            }
            CameraMode::Exterior => {
                if let Ok(spacecraft_transform) = spacecraft_query.get_single() {
                    let is_orbiting = mouse_button.pressed(MouseButton::Middle)
                        || (keyboard.pressed(KeyCode::ControlLeft) && mouse_button.pressed(MouseButton::Left));

                    if is_orbiting {
                        for motion in &motions {
                            orbit.theta -= motion.delta.x * orbit_speed;
                            orbit.phi = (orbit.phi + motion.delta.y * orbit_speed)
                                .clamp(5f32.to_radians(), 175f32.to_radians());
                        }
                    }

                    let zoom_input: f32 = wheels.iter().map(|w| w.y).sum();
                    if zoom_input != 0.0 {
                        orbit.radius -= zoom_input * zoom_speed;
                        orbit.radius = orbit.radius.clamp(12.0, 500.0);
                    }

                    let x = orbit.radius * orbit.phi.sin() * orbit.theta.cos();
                    let y = orbit.radius * orbit.phi.cos();
                    let z = orbit.radius * orbit.phi.sin() * orbit.theta.sin();
                    
                    let offset = Vec3::new(x, y, z);
                    transform.translation = spacecraft_transform.translation + offset;
                    transform.look_at(spacecraft_transform.translation, Vec3::Y);
                }
            }
            CameraMode::Interior => {
                if let Ok(spacecraft_transform) = spacecraft_query.get_single() {
                    let rocket_up = spacecraft_transform.rotation * Vec3::Y;
                    let rocket_forward = spacecraft_transform.rotation * Vec3::Z;
                    let rocket_right = spacecraft_transform.rotation * Vec3::X;
                    
                    let is_looking = mouse_button.pressed(MouseButton::Right);
                    let mut window = windows.single_mut();
                    
                    if is_looking {
                        window.cursor.visible = false;
                        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
                        
                        for motion in &motions {
                            interior.yaw -= motion.delta.x * look_speed;
                            interior.pitch += motion.delta.y * look_speed;
                        }
                    } else {
                        window.cursor.visible = true;
                        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
                    }
                    
                    if keyboard.just_pressed(KeyCode::Escape) {
                        window.cursor.visible = true;
                        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
                    }
                    
                    // Wrap yaw to [-PI, PI] using proper modulus
                    let pi = std::f32::consts::PI;
                    let two_pi = 2.0 * pi;
                    interior.yaw = interior.yaw - ((interior.yaw + pi) / two_pi).floor() * two_pi;
                    
                    let pitch_limit = crate::config::CAMERA_PITCH_CLAMP_DEG.to_radians();
                    interior.pitch = interior.pitch.clamp(-pitch_limit, pitch_limit);
                    
                    // Build look direction from yaw/pitch (like a standard FPS camera)
                    let look_dir = {
                        let cos_pitch = interior.pitch.cos();
                        let sin_pitch = interior.pitch.sin();
                        let cos_yaw = interior.yaw.cos();
                        let sin_yaw = interior.yaw.sin();
                        
                        // Start with rocket_forward as "forward", apply yaw then pitch
                        let forward_flat = rocket_forward * cos_yaw - rocket_right * sin_yaw;
                        let right_flat = rocket_forward * sin_yaw + rocket_right * cos_yaw;
                        
                        (forward_flat * cos_pitch + rocket_up * sin_pitch).normalize()
                    };
                    
                    let actual_right = look_dir.cross(rocket_up).normalize();
                    let actual_forward = actual_right.cross(rocket_up).normalize();
                    
                    let dt = time.delta_seconds();
                    if keyboard.pressed(KeyCode::KeyW) {
                        interior.position_offset += actual_forward * move_speed * dt;
                    }
                    if keyboard.pressed(KeyCode::KeyS) {
                        interior.position_offset -= actual_forward * move_speed * dt;
                    }
                    if keyboard.pressed(KeyCode::KeyA) {
                        interior.position_offset -= actual_right * move_speed * dt;
                    }
                    if keyboard.pressed(KeyCode::KeyD) {
                        interior.position_offset += actual_right * move_speed * dt;
                    }
                    if keyboard.pressed(KeyCode::KeyE) || keyboard.pressed(KeyCode::Space) {
                        interior.position_offset += rocket_up * move_speed * dt;
                    }
                    if keyboard.pressed(KeyCode::KeyQ) || keyboard.pressed(KeyCode::ShiftLeft) {
                        interior.position_offset -= rocket_up * move_speed * dt;
                    }
                    
                    let cm_base_radius = crate::config::CM_BASE_RADIUS;
                    let cm_top_radius = crate::config::CM_TOP_RADIUS;
                    let cm_height = crate::config::CM_HEIGHT;
                    let margin = 0.25f32;
                    
                    // Height bounds
                    interior.position_offset.y = interior.position_offset.y.clamp(0.1, cm_height - 0.3);
                    
                    // Conical radius at current height
                    let height_frac = interior.position_offset.y / cm_height;
                    let hull_radius = cm_base_radius - (cm_base_radius - cm_top_radius) * height_frac;
                    let max_radius = (hull_radius - margin).max(0.2);
                    
                    // Clamp horizontal position to inside cone
                    let h_dist_sq = interior.position_offset.x * interior.position_offset.x
                        + interior.position_offset.z * interior.position_offset.z;
                    if h_dist_sq > max_radius * max_radius {
                        let h_dist = h_dist_sq.sqrt();
                        let scale = max_radius / h_dist;
                        interior.position_offset.x *= scale;
                        interior.position_offset.z *= scale;
                    }
                    
                    // Position camera in world space
                    let cm_origin = spacecraft_transform.translation + rocket_up * cm_height * 0.5;
                    let world_pos = cm_origin 
                        + rocket_up * interior.position_offset.y
                        + rocket_forward * interior.position_offset.z
                        + rocket_right * interior.position_offset.x;
                    transform.translation = world_pos;
                    
                    // Look at point
                    transform.look_at(world_pos + look_dir, rocket_up);
                }
            }
        }
    }
}
