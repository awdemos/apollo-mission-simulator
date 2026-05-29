mod config;
mod spacecraft;
mod agc;
mod mission;
mod world;
mod sky;
mod weather;
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
mod game_state;
mod crew;
mod npc;
mod lvdc;
mod cooling;
mod gn2;
mod solar_system;
mod staging_animation;
mod damage;
mod effects;
mod cm_equipment;

use bevy::prelude::*;
use bevy::window::{Cursor, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Apollo Mission Simulator".to_string(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                cursor: Cursor {
                    visible: true,
                    grab_mode: bevy::window::CursorGrabMode::None,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CameraMode>()
        .init_resource::<TimeScale>()
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(sky::SkyPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(weather::WeatherPlugin)
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
        .add_plugins(ui::menu::MenuPlugin)
        .add_plugins(audio::AudioPlugin)
    .add_plugins(crew::CrewPlugin)
    .add_plugins(npc::NpcPlugin)
    .add_plugins(lvdc::LvdcPlugin)
    .add_plugins(gn2::Gn2Plugin)
    .add_plugins(solar_system::SolarSystemPlugin)
    .add_plugins(staging_animation::StagingAnimationPlugin)
        .add_plugins(damage::DamagePlugin)
            .add_plugins(effects::EngineEffectsPlugin)
            .add_plugins(cm_equipment::CmEquipmentPlugin)
        .add_systems(Update, cooling::update_cooling_systems.run_if(in_state(crate::game_state::AppState::InGame)))
        .add_systems(Startup, unlock_cursor_on_startup)
        .add_systems(Update, update_orbit_camera.run_if(in_state(game_state::AppState::InGame).and_then(is_orbit_mode)))
        .add_systems(Update, update_interior_camera.run_if(in_state(game_state::AppState::InGame).and_then(is_interior_mode)))
        .add_systems(Update, ensure_cursor_unlocked_in_menus.run_if(not(in_state(game_state::AppState::InGame))))
        .add_systems(Update, unlock_cursor_in_exterior.run_if(in_state(game_state::AppState::InGame).and_then(is_orbit_mode)))
        .run();
}

fn unlock_cursor_on_startup(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.cursor.visible = true;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
    }
}

fn ensure_cursor_unlocked_in_menus(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        if window.cursor.grab_mode != bevy::window::CursorGrabMode::None {
            window.cursor.visible = true;
            window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        }
    }
}

fn unlock_cursor_in_exterior(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        if window.cursor.grab_mode != bevy::window::CursorGrabMode::None {
            window.cursor.visible = true;
            window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        }
    }
}

fn is_orbit_mode(camera_mode: Res<CameraMode>) -> bool {
    matches!(*camera_mode, CameraMode::Free | CameraMode::Exterior)
}

fn is_interior_mode(camera_mode: Res<CameraMode>) -> bool {
    matches!(*camera_mode, CameraMode::Interior)
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    Interior,
    #[default]
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
    pub const MIN: f32 = 0.1;
    pub const MAX: f32 = 100.0;
    pub const STEP: f32 = 0.1;
    
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

pub fn update_orbit_camera(
    camera_mode: Res<CameraMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<Camera3d>>,
    spacecraft_query: Query<&Transform, (With<crate::spacecraft::PlayerVehicle>, Without<Camera3d>)>,
    launch_site_pos: Res<crate::world::LaunchSitePosition>,
) {
    let orbit_speed = 0.005;
    let zoom_speed = 2.0;

    let motions: Vec<_> = mouse_motion.read().collect();
    let wheels: Vec<_> = mouse_wheel.read().collect();

    for (mut transform, mut orbit) in query.iter_mut() {
        let is_orbiting = mouse_button.pressed(MouseButton::Middle)
            || (keyboard.pressed(KeyCode::AltLeft) && mouse_button.pressed(MouseButton::Left));

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
        }

        let x = orbit.radius * orbit.phi.sin() * orbit.theta.cos();
        let y = orbit.radius * orbit.phi.cos();
        let z = orbit.radius * orbit.phi.sin() * orbit.theta.sin();
        let offset = Vec3::new(x, y, z);

        let move_speed = 10.0;
        let dt = 0.016;
        let forward = transform.forward();
        let right = transform.right();
        
        if keyboard.pressed(KeyCode::KeyW) {
            orbit.target += forward * move_speed * dt;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            orbit.target -= forward * move_speed * dt;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            orbit.target -= right * move_speed * dt;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            orbit.target += right * move_speed * dt;
        }
        
        match *camera_mode {
            CameraMode::Free => {
                orbit.radius = orbit.radius.clamp(3.0, 2000.0);
                transform.translation = orbit.target + offset;
                transform.look_at(orbit.target, Vec3::Y);
            }
            CameraMode::Exterior => {
                orbit.radius = orbit.radius.clamp(3.0, 200.0);
                if let Ok(spacecraft_transform) = spacecraft_query.get_single() {
                    let target = spacecraft_transform.translation + Vec3::Y * 5.5;
                    transform.translation = target + offset;
                    transform.look_at(target, Vec3::Y);
                }
            }
            _ => {}
        }

        let ground_dist = crate::world::EARTH_RADIUS + crate::config::MLP_HEIGHT + 0.02;
        let surface_normal = launch_site_pos.0.normalize();
        let camera_height = transform.translation.dot(surface_normal);
        let min_height = ground_dist + 0.5;
        if camera_height < min_height {
            let correction = surface_normal * (min_height - camera_height);
            transform.translation += correction;
        }
    }
}

pub fn update_interior_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut query: Query<(&mut Transform, &mut InteriorCamera), With<Camera3d>>,
    spacecraft_query: Query<&Transform, (With<crate::spacecraft::PlayerVehicle>, Without<Camera3d>)>,
    mut windows: Query<&mut Window>,
) {
    let look_speed = crate::config::CAMERA_LOOK_SPEED;
    let move_speed = crate::config::CAMERA_MOVE_SPEED;

    let motions: Vec<_> = mouse_motion.read().collect();

    for (mut transform, mut interior) in query.iter_mut() {
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
            
            let pi = std::f32::consts::PI;
            let two_pi = 2.0 * pi;
            interior.yaw = interior.yaw - ((interior.yaw + pi) / two_pi).floor() * two_pi;
            
            let pitch_limit = crate::config::CAMERA_PITCH_CLAMP_DEG.to_radians();
            interior.pitch = interior.pitch.clamp(-pitch_limit, pitch_limit);
            
            let look_dir = {
                let cos_pitch = interior.pitch.cos();
                let sin_pitch = interior.pitch.sin();
                let cos_yaw = interior.yaw.cos();
                let sin_yaw = interior.yaw.sin();
                
                let forward_flat = rocket_forward * cos_yaw - rocket_right * sin_yaw;
                
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
            
            let scale = crate::config::SATURN_V_SCALE;
            let cm_base_radius = crate::config::CM_BASE_RADIUS * scale;
            let cm_top_radius = crate::config::CM_TOP_RADIUS * scale;
            let cm_height = crate::config::CM_HEIGHT * scale;
            let margin = 0.25f32 * scale;
            
            interior.position_offset.y = interior.position_offset.y.clamp(0.1 * scale, cm_height - 0.3 * scale);
            
            let height_frac = interior.position_offset.y / cm_height;
            let hull_radius = cm_base_radius - (cm_base_radius - cm_top_radius) * height_frac;
            let max_radius = (hull_radius - margin).max(0.2 * scale);
            
            let h_dist_sq = interior.position_offset.x * interior.position_offset.x
                + interior.position_offset.z * interior.position_offset.z;
            if h_dist_sq > max_radius * max_radius {
                let h_dist = h_dist_sq.sqrt();
                let clamp_scale = max_radius / h_dist;
                interior.position_offset.x *= clamp_scale;
                interior.position_offset.z *= clamp_scale;
            }
            
            let pos = interior.position_offset;
            let camera_radius = 0.15 * scale;
            
            let c1 = Vec3::new(-crate::config::COUCH_SPACING * scale, crate::config::COUCH_Y * scale + 0.25 * scale, crate::config::COUCH_Z * scale);
            let c1_half = Vec3::new(crate::config::COUCH_WIDTH * 0.5 * scale + camera_radius, 0.3 * scale + camera_radius, crate::config::COUCH_DEPTH * 0.5 * scale + camera_radius);
            if (pos.x - c1.x).abs() < c1_half.x && (pos.y - c1.y).abs() < c1_half.y && (pos.z - c1.z).abs() < c1_half.z {
                let dx = pos.x - c1.x;
                let dy = pos.y - c1.y;
                let dz = pos.z - c1.z;
                let rdx = dx.abs() / c1_half.x;
                let rdy = dy.abs() / c1_half.y;
                let rdz = dz.abs() / c1_half.z;
                if rdx >= rdy && rdx >= rdz { interior.position_offset.x = c1.x + c1_half.x * dx.signum(); }
                else if rdy >= rdx && rdy >= rdz { interior.position_offset.y = c1.y + c1_half.y * dy.signum(); }
                else { interior.position_offset.z = c1.z + c1_half.z * dz.signum(); }
            }
            
            let c2 = Vec3::new(0.0, crate::config::COUCH_Y * scale + 0.25 * scale, crate::config::COUCH_Z * scale + 0.15 * scale);
            let c2_half = Vec3::new(crate::config::COUCH_WIDTH * 0.5 * scale + camera_radius, 0.3 * scale + camera_radius, crate::config::COUCH_DEPTH * 0.5 * scale + camera_radius);
            if (pos.x - c2.x).abs() < c2_half.x && (pos.y - c2.y).abs() < c2_half.y && (pos.z - c2.z).abs() < c2_half.z {
                let dx = pos.x - c2.x;
                let dy = pos.y - c2.y;
                let dz = pos.z - c2.z;
                let rdx = dx.abs() / c2_half.x;
                let rdy = dy.abs() / c2_half.y;
                let rdz = dz.abs() / c2_half.z;
                if rdx >= rdy && rdx >= rdz { interior.position_offset.x = c2.x + c2_half.x * dx.signum(); }
                else if rdy >= rdx && rdy >= rdz { interior.position_offset.y = c2.y + c2_half.y * dy.signum(); }
                else { interior.position_offset.z = c2.z + c2_half.z * dz.signum(); }
            }
            
            let c3 = Vec3::new(crate::config::COUCH_SPACING * scale, crate::config::COUCH_Y * scale + 0.25 * scale, crate::config::COUCH_Z * scale);
            let c3_half = Vec3::new(crate::config::COUCH_WIDTH * 0.5 * scale + camera_radius, 0.3 * scale + camera_radius, crate::config::COUCH_DEPTH * 0.5 * scale + camera_radius);
            if (pos.x - c3.x).abs() < c3_half.x && (pos.y - c3.y).abs() < c3_half.y && (pos.z - c3.z).abs() < c3_half.z {
                let dx = pos.x - c3.x;
                let dy = pos.y - c3.y;
                let dz = pos.z - c3.z;
                let rdx = dx.abs() / c3_half.x;
                let rdy = dy.abs() / c3_half.y;
                let rdz = dz.abs() / c3_half.z;
                if rdx >= rdy && rdx >= rdz { interior.position_offset.x = c3.x + c3_half.x * dx.signum(); }
                else if rdy >= rdx && rdy >= rdz { interior.position_offset.y = c3.y + c3_half.y * dy.signum(); }
                else { interior.position_offset.z = c3.z + c3_half.z * dz.signum(); }
            }
            
            let console = Vec3::new(0.0, crate::config::CONSOLE_Y * scale, crate::config::CONSOLE_Z * scale);
            let console_half = Vec3::new(crate::config::CONSOLE_WIDTH * 0.5 * scale + camera_radius, crate::config::CONSOLE_HEIGHT * 0.5 * scale + camera_radius, crate::config::CONSOLE_DEPTH * 0.5 * scale + camera_radius);
            if (pos.x - console.x).abs() < console_half.x && (pos.y - console.y).abs() < console_half.y && (pos.z - console.z).abs() < console_half.z {
                let dx = pos.x - console.x;
                let dy = pos.y - console.y;
                let dz = pos.z - console.z;
                let rdx = dx.abs() / console_half.x;
                let rdy = dy.abs() / console_half.y;
                let rdz = dz.abs() / console_half.z;
                if rdx >= rdy && rdx >= rdz { interior.position_offset.x = console.x + console_half.x * dx.signum(); }
                else if rdy >= rdx && rdy >= rdz { interior.position_offset.y = console.y + console_half.y * dy.signum(); }
                else { interior.position_offset.z = console.z + console_half.z * dz.signum(); }
            }
            
            let cm_origin = spacecraft_transform.translation + rocket_up * crate::config::CM_CENTER_OFFSET * crate::config::SATURN_V_SCALE;
            let world_pos = cm_origin 
                + rocket_up * interior.position_offset.y
                + rocket_forward * interior.position_offset.z
                + rocket_right * interior.position_offset.x;
            transform.translation = world_pos;
            
            transform.look_at(world_pos + look_dir, Vec3::Y);
        }
    }
}
