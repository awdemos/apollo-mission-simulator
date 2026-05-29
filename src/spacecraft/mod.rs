use bevy::prelude::*;

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CmMaterialCache>()
            .init_resource::<CmMeshCache>()
            .add_systems(Startup, init_cm_mesh_cache)
            .add_systems(Update, update_spacecraft_transform.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, animate_swing_arm_retraction.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}
#[derive(Component)]
pub struct Spacecraft {
    pub vessel_type: VesselType,
    pub velocity: Vec3,
    pub altitude: f32,
    pub height_offset: f32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VesselType {
    SaturnV,
    CommandModule,
    ServiceModule,
    LunarModule,
    LMAscent,
    SIVB,
}

/// Marker for the active vehicle the camera should track.
#[derive(Component, Debug, Clone)]
pub struct PlayerVehicle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchState {
    OnPad,
    Countdown,
    Ignition,
    Liftoff,
    InFlight,
    OrbitInsertion,
    Tli,
    Translunar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchStage {
    FullStack,
    SIC_Burn,
    SII_Burn,
    SIVB_Burn1,
    SIVB_Burn2,
    Coast,
}

#[derive(Component, Debug, Clone)]
pub struct LaunchController {
    pub state: LaunchState,
    pub mission_time: f32,
    pub stage: LaunchStage,
}

#[derive(Component, Debug, Clone)]
pub struct SwingArm {
    pub arm_index: u32,
    pub retracted: f32,
    pub tower_offset: f32,
    pub arm_length: f32,
    pub arm_y: f32,
}

#[derive(Resource, Default)]
pub struct CmMaterialCache {
    pub wall: Option<Handle<StandardMaterial>>,
    pub panel: Option<Handle<StandardMaterial>>,
    pub panel_highlight: Option<Handle<StandardMaterial>>,
    pub seat: Option<Handle<StandardMaterial>>,
    pub dsky: Option<Handle<StandardMaterial>>,
    pub fdai: Option<Handle<StandardMaterial>>,
    pub display_glow: Option<Handle<StandardMaterial>>,
    pub window: Option<Handle<StandardMaterial>>,
    pub switch: Option<Handle<StandardMaterial>>,
    pub tunnel: Option<Handle<StandardMaterial>>,
    pub heat_shield: Option<Handle<StandardMaterial>>,
}

#[derive(Resource, Default)]
pub struct CmMeshCache {
    pub couch_seat: Option<Handle<Mesh>>,
    pub couch_back: Option<Handle<Mesh>>,
    pub couch_headrest: Option<Handle<Mesh>>,
    pub couch_armrest: Option<Handle<Mesh>>,
    pub switch: Option<Handle<Mesh>>,
    pub breaker: Option<Handle<Mesh>>,
    pub light: Option<Handle<Mesh>>,
}

fn init_cm_mesh_cache(
    mut cache: ResMut<CmMeshCache>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    use crate::config::*;
    
    cache.couch_seat = Some(meshes.add(Cuboid::new(COUCH_WIDTH, 0.08, COUCH_DEPTH)));
    cache.couch_back = Some(meshes.add(Cuboid::new(COUCH_WIDTH, 0.65, 0.06)));
    cache.couch_headrest = Some(meshes.add(Cuboid::new(COUCH_WIDTH * 0.75, 0.15, 0.05)));
    cache.couch_armrest = Some(meshes.add(Cuboid::new(0.08, 0.04, COUCH_DEPTH * 0.55)));
    cache.switch = Some(meshes.add(Cylinder::new(SWITCH_RADIUS, SWITCH_HEIGHT)));
    cache.breaker = Some(meshes.add(Cylinder::new(BREAKER_RADIUS, BREAKER_HEIGHT)));
    cache.light = Some(meshes.add(Sphere::new(0.025)));
}

pub fn spawn_apollo_stack(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    cache: &CmMeshCache,
) {
    let lat = crate::world::LAUNCH_SITE_LAT;
    let lon = crate::world::LAUNCH_SITE_LON;
    let r = crate::world::EARTH_RADIUS;
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.sin();
    let surface_position = Vec3::new(x, y, z);
    let normal = surface_position.normalize();
    let half_height = crate::config::SATURN_V_TOTAL_HEIGHT * 0.5;
    let mlp_offset = crate::config::MLP_HEIGHT + 0.02;
    let rocket_position = surface_position + normal * (half_height + mlp_offset);
    let up = Vec3::Y;
    let right = normal.cross(up).normalize();
    let forward = right.cross(normal);
    let rotation_matrix = Mat3::from_cols(right, normal, forward);
    let rotation = Quat::from_mat3(&rotation_matrix);
    spawn_saturn_v(commands, meshes, materials, cache, rocket_position, rotation);
}

fn spawn_saturn_v(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _cache: &CmMeshCache,
    position: Vec3,
    rotation: Quat,
) {
    use crate::config::*;

    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.93),
        metallic: 0.0,
        perceptual_roughness: 0.75,
        ..default()
    });
    let black = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.12),
        metallic: 0.0,
        perceptual_roughness: 0.45,
        ..default()
    });
    let silver = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.78, 0.8),
        metallic: 1.0,
        perceptual_roughness: 0.18,
        emissive: LinearRgba::new(0.10, 0.10, 0.11, 0.2),
        ..default()
    });
    let aluminum = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.72, 0.74),
        metallic: 1.0,
        perceptual_roughness: 0.28,
        emissive: LinearRgba::new(0.08, 0.08, 0.09, 0.15),
        ..default()
    });
    let rust = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.28, 0.12),
        metallic: 0.0,
        perceptual_roughness: 0.45,
        ..default()
    });
    let heat_shield = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.22, 0.14),
        metallic: 0.0,
        perceptual_roughness: 0.95,
        ..default()
    });
    let copper = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.45, 0.25),
        metallic: 1.0,
        perceptual_roughness: 0.4,
        emissive: LinearRgba::new(0.05, 0.03, 0.01, 0.1),
        ..default()
    });
    let copper_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.30, 0.15),
        metallic: 1.0,
        perceptual_roughness: 0.5,
        emissive: LinearRgba::new(0.03, 0.02, 0.01, 0.05),
        ..default()
    });
    let glow_ring = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.3, 0.1),
        emissive: LinearRgba::new(0.8, 0.2, 0.05, 0.3),
        ..default()
    });

    let saturn_v = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position).with_rotation(rotation),
            ..default()
        },
        Spacecraft {
            vessel_type: VesselType::SaturnV,
            velocity: Vec3::ZERO,
            altitude: 0.0,
            height_offset: SATURN_V_TOTAL_HEIGHT * 0.5,
        },
        LaunchController {
            state: LaunchState::OnPad,
            mission_time: 0.0,
            stage: LaunchStage::FullStack,
        },
        Name::new("Saturn V - Apollo 11 (SA-506)"),
        PlayerVehicle,
        crate::damage::StructuralIntegrity::new("Saturn V Full Stack", 10000.0),
        crate::damage::DamageControlSystem::default(),
        crate::damage::FireSuppressionSystem::default(),
        crate::damage::EmergencyEscapeSystem::default(),
    )).id();

    let half_height = SATURN_V_TOTAL_HEIGHT * 0.5;

    commands.entity(saturn_v).with_children(|parent| {
        parent.spawn(crate::cooling::create_f1_cooling());
        parent.spawn(crate::cooling::create_j2_cooling());
        let mut y = -half_height;

        // ============================================
        // S-IC FIRST STAGE (42.1m tall, 10.06m diameter)
        // ============================================
        let f1_nozzle_radius = 1.83 * SATURN_V_SCALE;
        let f1_nozzle_height = 0.56 * SATURN_V_SCALE * 1.2;
        let f1_mesh = meshes.add(Cone { radius: f1_nozzle_radius, height: f1_nozzle_height });
        let f1_throat_radius = f1_nozzle_radius * 0.45;
        let f1_throat_height = f1_nozzle_height * 0.5;
        let f1_throat_mesh = meshes.add(Cone { radius: f1_throat_radius, height: f1_throat_height });
        let f1_glow_radius = f1_nozzle_radius * 1.05;
        let f1_glow_height = 0.04 * SATURN_V_SCALE;
        let f1_glow_mesh = meshes.add(Cylinder::new(f1_glow_radius, f1_glow_height));
        let f1_outboard_offset = S_IC_RADIUS * 0.5;

        let f1_positions = [
            (0.0, 0.0, true),
            (f1_outboard_offset, 0.0, false),
            (-f1_outboard_offset, 0.0, false),
            (0.0, f1_outboard_offset, false),
            (0.0, -f1_outboard_offset, false),
        ];

        for (ex, ez, is_center) in f1_positions {
            let engine_y = y + f1_nozzle_height * 0.5;
            parent.spawn(PbrBundle {
                mesh: f1_mesh.clone(),
                material: copper.clone(),
                transform: Transform::from_xyz(ex, engine_y, ez),
                ..default()
            });

            let throat_y = engine_y + f1_nozzle_height * 0.5 - f1_throat_height * 0.5;
            parent.spawn(PbrBundle {
                mesh: f1_throat_mesh.clone(),
                material: copper_dark.clone(),
                transform: Transform::from_xyz(ex, throat_y, ez),
                ..default()
            });

            let glow_y = engine_y - f1_nozzle_height * 0.5 + f1_glow_height * 0.5;
            parent.spawn(PbrBundle {
                mesh: f1_glow_mesh.clone(),
                material: glow_ring.clone(),
                transform: Transform::from_xyz(ex, glow_y, ez),
                ..default()
            });

            let injector = meshes.add(Cylinder::new(f1_nozzle_radius * 0.85, 0.04 * SATURN_V_SCALE));
            parent.spawn(PbrBundle {
                mesh: injector,
                material: silver.clone(),
                transform: Transform::from_xyz(ex, engine_y + f1_nozzle_height * 0.42, ez),
                ..default()
            });

            let gimbal = meshes.add(Cylinder::new(f1_nozzle_radius * 1.05, 0.06 * SATURN_V_SCALE));
            parent.spawn(PbrBundle {
                mesh: gimbal,
                material: silver.clone(),
                transform: Transform::from_xyz(ex, engine_y - f1_nozzle_height * 0.35, ez),
                ..default()
            });

            if !is_center {
                let duct_radius = 0.12 * SATURN_V_SCALE;
                let duct_height = 0.35 * SATURN_V_SCALE;
                let duct = meshes.add(Cylinder::new(duct_radius, duct_height));
                let duct_x = ex + (ex.signum() * f1_nozzle_radius * 0.7);
                let duct_z = ez + (ez.signum() * f1_nozzle_radius * 0.7);
                parent.spawn(PbrBundle {
                    mesh: duct,
                    material: silver.clone(),
                    transform: Transform::from_xyz(duct_x, engine_y + f1_nozzle_height * 0.15, duct_z),
                    ..default()
                });

                let line_radius = 0.05 * SATURN_V_SCALE;
                let line_height = 0.8 * SATURN_V_SCALE;
                let feed_line = meshes.add(Cylinder::new(line_radius, line_height));
                let line_angle = ez.atan2(ex);
                let line_x = ex + line_angle.cos() * (f1_nozzle_radius + 0.15 * SATURN_V_SCALE);
                let line_z = ez + line_angle.sin() * (f1_nozzle_radius + 0.15 * SATURN_V_SCALE);
                parent.spawn(PbrBundle {
                    mesh: feed_line,
                    material: silver.clone(),
                    transform: Transform::from_xyz(line_x, engine_y + f1_nozzle_height * 0.25 + line_height * 0.3, line_z)
                        .with_rotation(Quat::from_rotation_z(0.15)),
                    ..default()
                });
            }
        }

        // 4 small ullage motor nozzles around the outboard engines
        let ullage_radius = 0.15 * SATURN_V_SCALE;
        let ullage_height = 0.25 * SATURN_V_SCALE;
        let ullage_mesh = meshes.add(Cone { radius: ullage_radius, height: ullage_height });
        let ullage_offset = f1_outboard_offset * 1.35;
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let ux = angle.cos() * ullage_offset;
            let uz = angle.sin() * ullage_offset;
            let ullage_y = y + ullage_height * 0.5;
            parent.spawn(PbrBundle {
                mesh: ullage_mesh.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(ux, ullage_y, uz),
                ..default()
            });
        }

        y += f1_nozzle_height;

        // S-IC thrust structure (short silver section)
        let thrust_struct_height = 0.6 * SATURN_V_SCALE;
        let thrust_struct = meshes.add(Cylinder::new(S_IC_RADIUS, thrust_struct_height));
        parent.spawn(PbrBundle {
            mesh: thrust_struct.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + thrust_struct_height * 0.5, 0.0),
            ..default()
        });
        y += thrust_struct_height;

        // S-IC aft skirt with stabilizing fins
        let fin_height = 1.2 * SATURN_V_SCALE;
        let fin_mesh = meshes.add(Cuboid::new(0.15 * SATURN_V_SCALE, fin_height, 0.8 * SATURN_V_SCALE));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let fin_dist = S_IC_RADIUS + 0.3 * SATURN_V_SCALE;
            let fx = angle.cos() * fin_dist;
            let fz = angle.sin() * fin_dist;
            parent.spawn(PbrBundle {
                mesh: fin_mesh.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(fx, y + fin_height * 0.5, fz)
                    .with_rotation(Quat::from_rotation_y(angle)),
                ..default()
            });
        }

        let ic_aft_skirt_height = 1.2 * SATURN_V_SCALE;
        let ic_aft_skirt = meshes.add(Cylinder::new(S_IC_RADIUS * 1.005, ic_aft_skirt_height));
        parent.spawn(PbrBundle {
            mesh: ic_aft_skirt,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height * 0.5, 0.0),
            ..default()
        });

        let junction_ring = meshes.add(Cylinder::new(S_IC_RADIUS * 1.01, 0.02 * SATURN_V_SCALE));
        parent.spawn(PbrBundle {
            mesh: junction_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + 0.01 * SATURN_V_SCALE, 0.0),
            ..default()
        });

        let ic_lox_height = S_IC_HEIGHT * 0.42;
        let ic_lox = meshes.add(Cylinder::new(S_IC_RADIUS, ic_lox_height));
        parent.spawn(PbrBundle {
            mesh: ic_lox,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height * 0.5, 0.0),
            ..default()
        });

        parent.spawn(PbrBundle {
            mesh: junction_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + 0.01 * SATURN_V_SCALE, 0.0),
            ..default()
        });

        let ic_intertank_height = S_IC_HEIGHT * 0.15;
        let ic_intertank = meshes.add(Cylinder::new(S_IC_RADIUS * 1.005, ic_intertank_height));
        parent.spawn(PbrBundle {
            mesh: ic_intertank,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + ic_intertank_height * 0.5, 0.0),
            ..default()
        });

        parent.spawn(PbrBundle {
            mesh: junction_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + ic_intertank_height + 0.01 * SATURN_V_SCALE, 0.0),
            ..default()
        });

        let ic_rp1_height = S_IC_HEIGHT * 0.32;
        let ic_rp1 = meshes.add(Cylinder::new(S_IC_RADIUS, ic_rp1_height));
        parent.spawn(PbrBundle {
            mesh: ic_rp1,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + ic_intertank_height + ic_rp1_height * 0.5, 0.0),
            ..default()
        });

        parent.spawn(PbrBundle {
            mesh: junction_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + ic_intertank_height + ic_rp1_height + 0.01 * SATURN_V_SCALE, 0.0),
            ..default()
        });

        let ic_forward_skirt_height = S_IC_HEIGHT * 0.12;
        let ic_forward_skirt = meshes.add(Cylinder::new(S_IC_RADIUS * 1.005, ic_forward_skirt_height));
        parent.spawn(PbrBundle {
            mesh: ic_forward_skirt,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + ic_aft_skirt_height + ic_lox_height + ic_intertank_height + ic_rp1_height + ic_forward_skirt_height * 0.5, 0.0),
            ..default()
        });

        // "USA" text-like representation using small black boxes on white body
        let usa_y = y + ic_aft_skirt_height + ic_lox_height * 0.45;
        let usa_x = S_IC_RADIUS * 1.01;
        let char_h = 0.5 * SATURN_V_SCALE;
        let char_w = 0.35 * SATURN_V_SCALE;
        let char_d = 0.06 * SATURN_V_SCALE;
        let char_thick = 0.08 * SATURN_V_SCALE;
        let letter_mesh = meshes.add(Cuboid::new(char_d, char_h, char_thick));

        // U
        parent.spawn(PbrBundle {
            mesh: letter_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, -char_w),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: letter_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, char_w),
            ..default()
        });
        let u_base = meshes.add(Cuboid::new(char_d, char_thick, char_w * 2.2));
        parent.spawn(PbrBundle {
            mesh: u_base,
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y - char_h * 0.4, 0.0),
            ..default()
        });

        // S
        let s_hbar = meshes.add(Cuboid::new(char_d, char_thick, char_w * 1.8));
        let s_vbar = meshes.add(Cuboid::new(char_d, char_h * 0.45, char_thick));
        parent.spawn(PbrBundle {
            mesh: s_hbar.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y + char_h * 0.35, -char_w * 0.2),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: s_hbar.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, char_w * 0.2),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: s_hbar.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y - char_h * 0.35, -char_w * 0.2),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: s_vbar.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y + char_h * 0.18, -char_w * 0.8),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: s_vbar.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y - char_h * 0.18, char_w * 0.8),
            ..default()
        });

        // A
        parent.spawn(PbrBundle {
            mesh: letter_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, char_w * 2.2)
                .with_rotation(Quat::from_rotation_y(0.12)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: letter_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, char_w * 3.4)
                .with_rotation(Quat::from_rotation_y(-0.12)),
            ..default()
        });
        let a_bar = meshes.add(Cuboid::new(char_d, char_thick, char_w));
        parent.spawn(PbrBundle {
            mesh: a_bar,
            material: black.clone(),
            transform: Transform::from_xyz(usa_x, usa_y, char_w * 2.8),
            ..default()
        });

        // USA flag on S-IC at ~70% height
        let flag_y = y + S_IC_HEIGHT * 0.7;
        let flag_x = S_IC_RADIUS * 1.01;
        let flag_w = 0.3 * SATURN_V_SCALE;
        let flag_h = 0.2 * SATURN_V_SCALE;
        let flag_d = 0.01 * SATURN_V_SCALE;
        let flag_bg = meshes.add(Cuboid::new(flag_d, flag_h, flag_w));
        let flag_white_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            emissive: Color::srgb(0.3, 0.3, 0.3).into(),
            unlit: true,
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: flag_bg,
            material: flag_white_mat.clone(),
            transform: Transform::from_xyz(flag_x, flag_y, 0.0),
            ..default()
        });

        let blue_w = flag_w * 0.4;
        let blue_h = flag_h * 0.5;
        let flag_blue_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.1, 0.6),
            emissive: Color::srgb(0.0, 0.05, 0.3).into(),
            unlit: true,
            ..default()
        });
        let blue_field = meshes.add(Cuboid::new(flag_d * 1.1, blue_h, blue_w));
        parent.spawn(PbrBundle {
            mesh: blue_field,
            material: flag_blue_mat,
            transform: Transform::from_xyz(flag_x, flag_y + flag_h * 0.25, -flag_w * 0.25),
            ..default()
        });

        let stripe_h = flag_h / 7.0;
        let flag_red_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.05, 0.05),
            emissive: Color::srgb(0.3, 0.02, 0.02).into(),
            unlit: true,
            ..default()
        });
        for i in [0, 2, 4, 6] {
            let stripe = meshes.add(Cuboid::new(flag_d * 1.1, stripe_h * 0.9, flag_w));
            let stripe_y = flag_y - flag_h * 0.5 + stripe_h * (i as f32 + 0.5);
            parent.spawn(PbrBundle {
                mesh: stripe,
                material: flag_red_mat.clone(),
                transform: Transform::from_xyz(flag_x, stripe_y, 0.0),
                ..default()
            });
        }
        let lox_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.9, 0.95),
            metallic: 0.6,
            perceptual_roughness: 0.3,
            ..default()
        });
        let rp1_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.85, 0.6),
            metallic: 0.2,
            perceptual_roughness: 0.5,
            ..default()
        });
        let lh2_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.95, 1.0),
            metallic: 0.5,
            perceptual_roughness: 0.35,
            ..default()
        });

        let ic_lox_tank = meshes.add(Cylinder::new(S_IC_RADIUS * 0.96, S_IC_HEIGHT * 0.55));
        parent.spawn(PbrBundle {
            mesh: ic_lox_tank,
            material: lox_mat.clone(),
            transform: Transform::from_xyz(0.0, y + S_IC_HEIGHT * 0.35, 0.0),
            ..default()
        });
        let ic_rp1_tank = meshes.add(Cylinder::new(S_IC_RADIUS * 0.96, S_IC_HEIGHT * 0.35));
        parent.spawn(PbrBundle {
            mesh: ic_rp1_tank,
            material: rp1_mat.clone(),
            transform: Transform::from_xyz(0.0, y + S_IC_HEIGHT * 0.78, 0.0),
            ..default()
        });

        let ic_top_band = meshes.add(Cylinder::new(S_IC_RADIUS, 0.2 * SATURN_V_SCALE));
        parent.spawn(PbrBundle {
            mesh: ic_top_band,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y + S_IC_HEIGHT - 0.1 * SATURN_V_SCALE, 0.0),
            ..default()
        });
        parent.spawn(crate::damage::StructuralIntegrity::new("S-IC First Stage", 3000.0));
        y += S_IC_HEIGHT;

        let interstage1_height = 0.8 * SATURN_V_SCALE;
        let interstage1 = meshes.add(Cylinder::new(S_IC_RADIUS * 1.01, interstage1_height));
        parent.spawn(PbrBundle {
            mesh: interstage1.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + interstage1_height * 0.5, 0.0),
            ..default()
        });
        let interstage_ring = meshes.add(Cylinder::new(S_IC_RADIUS * 1.015, 0.04 * SATURN_V_SCALE));
        parent.spawn(PbrBundle {
            mesh: interstage_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + interstage1_height * 0.25, 0.0),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: interstage_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + interstage1_height * 0.75, 0.0),
            ..default()
        });
        y += interstage1_height;

        // ============================================
        // S-II SECOND STAGE (24.8m tall, 10.06m diameter)
        // ============================================
        // S-II engine skirt (wider at base)
        let s2_skirt_height = 0.25 * SATURN_V_SCALE;
        let s2_skirt = meshes.add(Cylinder::new(S_II_RADIUS * 1.02, s2_skirt_height));
        parent.spawn(PbrBundle {
            mesh: s2_skirt.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + s2_skirt_height * 0.5, 0.0),
            ..default()
        });
        y += s2_skirt_height;

        // J-2 engines on S-II (5 in quincunx, smaller than F-1)
        let j2_nozzle_radius = 1.0 * SATURN_V_SCALE; // 2.0m diameter / 2
        let j2_nozzle_height = 0.34 * SATURN_V_SCALE; // 3.4m tall
        let j2_mesh = meshes.add(Cone { radius: j2_nozzle_radius, height: j2_nozzle_height });
        let j2_outboard_offset = S_II_RADIUS * 0.45;

        let j2_positions = [
            (0.0, 0.0),
            (j2_outboard_offset, 0.0),
            (-j2_outboard_offset, 0.0),
            (0.0, j2_outboard_offset),
            (0.0, -j2_outboard_offset),
        ];

        for (ex, ez) in j2_positions {
            parent.spawn(PbrBundle {
                mesh: j2_mesh.clone(),
                material: black.clone(),
                transform: Transform::from_xyz(ex, y - j2_nozzle_height * 0.3, ez),
                ..default()
            });
        }

        let s2_body_height = S_II_HEIGHT - s2_skirt_height;
        let s2_body = meshes.add(Cylinder::new(S_II_RADIUS, s2_body_height));
        parent.spawn(PbrBundle {
            mesh: s2_body.clone(),
            material: aluminum.clone(),
            transform: Transform::from_xyz(0.0, y + s2_body_height * 0.5, 0.0),
            ..default()
        });

        let s2_lox_tank = meshes.add(Cylinder::new(S_II_RADIUS * 0.96, s2_body_height * 0.55));
        parent.spawn(PbrBundle {
            mesh: s2_lox_tank,
            material: lox_mat.clone(),
            transform: Transform::from_xyz(0.0, y + s2_body_height * 0.35, 0.0),
            ..default()
        });
        let s2_lh2_tank = meshes.add(Cylinder::new(S_II_RADIUS * 0.96, s2_body_height * 0.35));
        parent.spawn(PbrBundle {
            mesh: s2_lh2_tank,
            material: lh2_mat.clone(),
            transform: Transform::from_xyz(0.0, y + s2_body_height * 0.72, 0.0),
            ..default()
        });

        parent.spawn(crate::damage::StructuralIntegrity::new("S-II Second Stage", 2500.0));
        y += s2_body_height;

        let interstage2_height = 0.6 * SATURN_V_SCALE;
        let interstage2 = meshes.add(Cylinder::new(S_II_RADIUS * 1.01, interstage2_height));
        parent.spawn(PbrBundle {
            mesh: interstage2.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + interstage2_height * 0.5, 0.0),
            ..default()
        });
        let interstage2_ring = meshes.add(Cylinder::new(S_II_RADIUS * 1.015, 0.03 * SATURN_V_SCALE));
        parent.spawn(PbrBundle {
            mesh: interstage2_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + interstage2_height * 0.3, 0.0),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: interstage2_ring.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + interstage2_height * 0.7, 0.0),
            ..default()
        });
        y += interstage2_height;

        // ============================================
        // S-IVB THIRD STAGE (17.8m tall, 6.6m diameter)
        // ============================================
        // S-IVB aft skirt
        let ivb_skirt_height = 0.2 * SATURN_V_SCALE;
        let ivb_skirt = meshes.add(Cylinder::new(S_IVb_RADIUS * 1.02, ivb_skirt_height));
        parent.spawn(PbrBundle {
            mesh: ivb_skirt.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + ivb_skirt_height * 0.5, 0.0),
            ..default()
        });
        y += ivb_skirt_height;

        // J-2 engine on S-IVB (single engine)
        let ivb_j2_mesh = meshes.add(Cone { radius: j2_nozzle_radius, height: j2_nozzle_height });
        parent.spawn(PbrBundle {
            mesh: ivb_j2_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y - j2_nozzle_height * 0.3, 0.0),
            ..default()
        });

        let ivb_lox_tank = meshes.add(Cylinder::new(S_IVb_RADIUS * 0.95, S_IVb_HEIGHT * 0.4));
        parent.spawn(PbrBundle {
            mesh: ivb_lox_tank,
            material: lox_mat.clone(),
            transform: Transform::from_xyz(0.0, y + S_IVb_HEIGHT * 0.25, 0.0),
            ..default()
        });
        let ivb_lh2_tank = meshes.add(Cylinder::new(S_IVb_RADIUS * 0.95, S_IVb_HEIGHT * 0.45));
        parent.spawn(PbrBundle {
            mesh: ivb_lh2_tank,
            material: lh2_mat.clone(),
            transform: Transform::from_xyz(0.0, y + S_IVb_HEIGHT * 0.68, 0.0),
            ..default()
        });

        let ivb_body_height = S_IVb_HEIGHT - ivb_skirt_height;
        let ivb_body = meshes.add(Cylinder::new(S_IVb_RADIUS, ivb_body_height));
        parent.spawn(PbrBundle {
            mesh: ivb_body.clone(),
            material: aluminum.clone(),
            transform: Transform::from_xyz(0.0, y + ivb_body_height * 0.5, 0.0),
            ..default()
        });
        parent.spawn(crate::damage::StructuralIntegrity::new("S-IVB Third Stage", 1800.0));
        y += ivb_body_height;

        // ============================================
        // INSTRUMENT UNIT (0.91m tall, 6.6m diameter)
        // ============================================
        let iu_body = meshes.add(Cylinder::new(IU_RADIUS, IU_HEIGHT));
        parent.spawn(PbrBundle {
            mesh: iu_body.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + IU_HEIGHT * 0.5, 0.0),
            ..default()
        });
        let lvdc_box = meshes.add(Cuboid::new(0.2 * SATURN_V_SCALE, 0.12 * SATURN_V_SCALE, 0.15 * SATURN_V_SCALE));
        let lvdc_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.45, 0.5),
            metallic: 0.7,
            perceptual_roughness: 0.3,
            ..default()
        });
        for i in 0..3 {
            let angle = (i as f32 / 3.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_6;
            let lx = angle.cos() * (IU_RADIUS * 0.7);
            let lz = angle.sin() * (IU_RADIUS * 0.7);
            parent.spawn(PbrBundle {
                mesh: lvdc_box.clone(),
                material: lvdc_mat.clone(),
                transform: Transform::from_xyz(lx, y + IU_HEIGHT * 0.5, lz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }

        let iu_bump = meshes.add(Cuboid::new(0.15 * SATURN_V_SCALE, 0.08 * SATURN_V_SCALE, 0.1 * SATURN_V_SCALE));
        for i in 0..8 {
            let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
            let bx = angle.cos() * (IU_RADIUS + 0.05 * SATURN_V_SCALE);
            let bz = angle.sin() * (IU_RADIUS + 0.05 * SATURN_V_SCALE);
            parent.spawn(PbrBundle {
                mesh: iu_bump.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(bx, y + IU_HEIGHT * 0.5, bz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }
        parent.spawn(crate::damage::StructuralIntegrity::new("Instrument Unit", 500.0));
        y += IU_HEIGHT;

        // ============================================
        // SLA - SPACECRAFT LUNAR MODULE ADAPTER
        // Conical frustum: 6.6m base -> 3.9m top
        // ============================================
        let sla_body = create_conical_frustum_mesh(
            meshes, SLA_BASE_RADIUS, SLA_TOP_RADIUS, 0.0, SLA_HEIGHT, 24,
        );
        let sla_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.92, 0.9),
            metallic: 0.1,
            perceptual_roughness: 0.7,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: sla_body,
            material: sla_mat.clone(),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..default()
        });
        // SLA panel separation lines (4 vertical black strips)
        let panel_line = meshes.add(Cuboid::new(0.02 * SATURN_V_SCALE, SLA_HEIGHT * 0.95, 0.02 * SATURN_V_SCALE));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let line_radius = SLA_BASE_RADIUS + 0.01 * SATURN_V_SCALE;
            let lx = angle.cos() * line_radius;
            let lz = angle.sin() * line_radius;
            parent.spawn(PbrBundle {
                mesh: panel_line.clone(),
                material: black.clone(),
                transform: Transform::from_xyz(lx, y + SLA_HEIGHT * 0.5, lz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }

        // LM visible inside SLA (gold foil ascent stage + silver descent stage)
        let lm_gold = materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.65, 0.25),
            metallic: 0.8,
            perceptual_roughness: 0.25,
            ..default()
        });
        let lm_silver = materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.72, 0.75),
            metallic: 0.85,
            perceptual_roughness: 0.15,
            ..default()
        });
        let lm_body = meshes.add(Cylinder::new(SLA_TOP_RADIUS * 0.75, SLA_HEIGHT * 0.55));
        parent.spawn(PbrBundle {
            mesh: lm_body,
            material: lm_gold.clone(),
            transform: Transform::from_xyz(0.0, y + SLA_HEIGHT * 0.35, 0.0),
            ..default()
        });
        let lm_legs = meshes.add(Cuboid::new(SLA_TOP_RADIUS * 1.2, 0.08 * SATURN_V_SCALE, SLA_TOP_RADIUS * 1.2));
        parent.spawn(PbrBundle {
            mesh: lm_legs,
            material: lm_silver.clone(),
            transform: Transform::from_xyz(0.0, y + SLA_HEIGHT * 0.08, 0.0),
            ..default()
        });
        y += SLA_HEIGHT;

        // ============================================
        // SERVICE MODULE (6.5m tall, 3.9m diameter)
        // ============================================
        let sm_body = meshes.add(Cylinder::new(SM_RADIUS, SM_HEIGHT));
        parent.spawn(PbrBundle {
            mesh: sm_body.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y + SM_HEIGHT * 0.5, 0.0),
            ..default()
        });

        // SPS engine bell at bottom of SM (~3.8m long, 1.5m diameter)
        let sps_nozzle_radius = 0.75 * SATURN_V_SCALE;
        let sps_nozzle_height = 3.8 * SATURN_V_SCALE;
        let sps_mesh = meshes.add(Cone { radius: sps_nozzle_radius, height: sps_nozzle_height });
        parent.spawn(PbrBundle {
            mesh: sps_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y - sps_nozzle_height * 0.4, 0.0),
            ..default()
        });

        // SM RCS quads (4 quads, 90° apart, offset 7° from axes)
        let quad_offset = SM_RADIUS + 0.08 * SATURN_V_SCALE;
        let quad_size = 0.12 * SATURN_V_SCALE;
        let quad_mesh = meshes.add(Cuboid::new(quad_size, quad_size * 2.0, quad_size));
        let quad_angle_offset = 7.25f32.to_radians();
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + quad_angle_offset;
            let qx = angle.cos() * quad_offset;
            let qz = angle.sin() * quad_offset;
            let quad_y = y + SM_HEIGHT * 0.3 + (i % 2) as f32 * SM_HEIGHT * 0.3;
            parent.spawn(PbrBundle {
                mesh: quad_mesh.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(qx, quad_y, qz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }
        parent.spawn(crate::damage::StructuralIntegrity::new("Service Module", 1500.0));
        y += SM_HEIGHT;

        // ============================================
        // COMMAND MODULE (3.0m tall, 3.9m base -> 1.2m top)
        // Blunt conical frustum with heat shield
        // ============================================
        let cm_body = create_conical_frustum_mesh(
            meshes, CSM_BASE_RADIUS, CSM_TOP_RADIUS, 0.0, CSM_HEIGHT, 24,
        );
        parent.spawn(PbrBundle {
            mesh: cm_body,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..default()
        });

        // Heat shield (dark brown disk at base)
        let heat_shield_radius = CSM_BASE_RADIUS * 1.02;
        let heat_shield_mesh = meshes.add(Cylinder::new(heat_shield_radius, 0.05 * SATURN_V_SCALE));
        parent.spawn(PbrBundle {
            mesh: heat_shield_mesh.clone(),
            material: heat_shield.clone(),
            transform: Transform::from_xyz(0.0, y + 0.025 * SATURN_V_SCALE, 0.0),
            ..default()
        });

        // Docking probe at apex (~0.8m long, 0.3m diameter)
        let probe_length = 0.8 * SATURN_V_SCALE;
        let probe_radius = 0.15 * SATURN_V_SCALE;
        let probe_mesh = meshes.add(Cylinder::new(probe_radius, probe_length));
        parent.spawn(PbrBundle {
            mesh: probe_mesh.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, y + CSM_HEIGHT + probe_length * 0.5, 0.0),
            ..default()
        });
        parent.spawn(crate::damage::StructuralIntegrity::new("Command Module", 1000.0));
        y += CSM_HEIGHT;

        // ============================================
        // LAUNCH ESCAPE SYSTEM (9.1m tall, 0.66m diameter)
        // ============================================
        let bpc = create_conical_frustum_mesh(
            meshes, CSM_TOP_RADIUS * 1.05, LES_RADIUS * 1.15, 0.0, BPC_HEIGHT, 16,
        );
        parent.spawn(PbrBundle {
            mesh: bpc,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..default()
        });
        y += BPC_HEIGHT;

        // Main LES tower (white lower section, black upper section)
        let les_lower_height = LES_HEIGHT * 0.65;
        let les_upper_height = LES_HEIGHT * 0.35;
        let les_lower = meshes.add(Cylinder::new(LES_RADIUS, les_lower_height));
        parent.spawn(PbrBundle {
            mesh: les_lower.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, y + les_lower_height * 0.5, 0.0),
            ..default()
        });
        let les_upper = meshes.add(Cylinder::new(LES_RADIUS * 0.85, les_upper_height));
        parent.spawn(PbrBundle {
            mesh: les_upper.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + les_lower_height + les_upper_height * 0.5, 0.0),
            ..default()
        });

        // Canard fins near top of LES (4 fins)
        let canard_height = 0.35 * SATURN_V_SCALE;
        let canard_length = 0.5 * SATURN_V_SCALE;
        let canard_mesh = meshes.add(Cuboid::new(0.02 * SATURN_V_SCALE, canard_height, canard_length));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let cx = angle.cos() * (LES_RADIUS + canard_length * 0.4);
            let cz = angle.sin() * (LES_RADIUS + canard_length * 0.4);
            parent.spawn(PbrBundle {
                mesh: canard_mesh.clone(),
                material: black.clone(),
                transform: Transform::from_xyz(cx, y + les_lower_height + les_upper_height * 0.6, cz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }

        // Q-ball at top of LES (black sphere, larger for visibility)
        let qball_radius = 0.25 * SATURN_V_SCALE;
        let qball_mesh = meshes.add(Sphere::new(qball_radius));
        parent.spawn(PbrBundle {
            mesh: qball_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, y + LES_HEIGHT + qball_radius, 0.0),
            ..default()
        });

        // LES engine nozzles (4 around base)
        let les_nozzle_radius = 0.12 * SATURN_V_SCALE;
        let les_nozzle_height = 0.25 * SATURN_V_SCALE;
        let les_nozzle = meshes.add(Cone { radius: les_nozzle_radius, height: les_nozzle_height });
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let nx = angle.cos() * (LES_RADIUS + les_nozzle_radius);
            let nz = angle.sin() * (LES_RADIUS + les_nozzle_radius);
            parent.spawn(PbrBundle {
                mesh: les_nozzle.clone(),
                material: black.clone(),
                transform: Transform::from_xyz(nx, y + les_nozzle_height * 0.3, nz)
                    .with_rotation(Quat::from_rotation_y(-angle)),
                ..default()
            });
        }
        y += LES_HEIGHT;

    });

    let interior = spawn_cm_interior(commands, meshes, materials, _cache, saturn_v);
    crate::cm_equipment::spawn_equipment_bays(commands, meshes, materials, interior);
}
pub fn spawn_command_module(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) -> Entity {
    let cone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.8),
        metallic: 0.4,
        perceptual_roughness: 0.4,
        ..default()
    });
    let cylinder = meshes.add(Cylinder::new(1.9, 3.0));
    let cone_mesh = meshes.add(Cone { radius: 1.9, height: 3.5 });
    commands.spawn((
        PbrBundle {
            mesh: cylinder,
            material: cone.clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        Spacecraft {
            vessel_type: VesselType::CommandModule,
            velocity: Vec3::ZERO,
            altitude: 0.0,
            height_offset: 0.0,
        },
        Name::new("Command Module"),
    )).id()
}
pub fn spawn_lunar_module(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) -> Entity {
    let gold = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.65, 0.25),
        metallic: 0.8,
        perceptual_roughness: 0.25,
        ..default()
    });
    let silver = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.72, 0.75),
        metallic: 0.85,
        perceptual_roughness: 0.15,
        ..default()
    });
    let black = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    });
    let foil = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.78, 0.45),
        metallic: 0.95,
        perceptual_roughness: 0.1,
        ..default()
    });
    
    let lm = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position),
            ..default()
        },
        Spacecraft {
            vessel_type: VesselType::LunarModule,
            velocity: Vec3::ZERO,
            altitude: 0.0,
            height_offset: 0.0,
        },
        Name::new("Lunar Module (Eagle)"),
    )).id();
    
    commands.entity(lm).with_children(|parent| {
        let desc_frame = meshes.add(Cuboid::new(3.2, 0.3, 3.9));
        parent.spawn(PbrBundle {
            mesh: desc_frame,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, 0.15, 0.0),
            ..default()
        });
        
        let tank1 = meshes.add(Sphere::new(0.65));
        parent.spawn(PbrBundle {
            mesh: tank1.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(-0.8, 0.5, 0.6),
            ..default()
        });
        let tank2 = meshes.add(Sphere::new(0.65));
        parent.spawn(PbrBundle {
            mesh: tank2,
            material: silver.clone(),
            transform: Transform::from_xyz(0.8, 0.5, -0.6),
            ..default()
        });
        
        let desc_engine = meshes.add(Cone { radius: 0.7, height: 1.2 });
        parent.spawn(PbrBundle {
            mesh: desc_engine,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, -0.4, 0.0),
            ..default()
        });
        
        let cabin = meshes.add(Cuboid::new(2.7, 2.1, 3.2));
        parent.spawn(PbrBundle {
            mesh: cabin,
            material: gold.clone(),
            transform: Transform::from_xyz(0.0, 1.4, 0.0),
            ..default()
        });
        
        let window = meshes.add(Cuboid::new(0.4, 0.3, 0.05));
        parent.spawn(PbrBundle {
            mesh: window.clone(),
            material: foil.clone(),
            transform: Transform::from_xyz(0.0, 1.8, 1.62),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: window,
            material: foil.clone(),
            transform: Transform::from_xyz(0.0, 1.8, -1.62),
            ..default()
        });
        
        let hatch = meshes.add(Cylinder::new(0.5, 0.15));
        parent.spawn(PbrBundle {
            mesh: hatch,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, 2.55, 0.0),
            ..default()
        });
        
        let asc_engine = meshes.add(Cone { radius: 0.4, height: 0.8 });
        parent.spawn(PbrBundle {
            mesh: asc_engine,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });
        
        let leg_strut = meshes.add(Cylinder::new(0.08, 2.8));
        let foot_pad = meshes.add(Cylinder::new(0.35, 0.08));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let leg_x = angle.cos() * 2.8;
            let leg_z = angle.sin() * 2.8;
            parent.spawn(PbrBundle {
                mesh: leg_strut.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(leg_x, -1.2, leg_z)
                    .with_rotation(Quat::from_rotation_x(0.15)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: foot_pad.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(leg_x * 1.15, -2.5, leg_z * 1.15),
                ..default()
            });
        }
        
        let rcs_quad = meshes.add(Cuboid::new(0.25, 0.25, 0.25));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU;
            let qx = angle.cos() * 1.8;
            let qz = angle.sin() * 1.8;
            parent.spawn(PbrBundle {
                mesh: rcs_quad.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(qx, 1.2, qz),
                ..default()
            });
        }
        
        let radar = meshes.add(Cylinder::new(0.25, 0.4));
        parent.spawn(PbrBundle {
            mesh: radar,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 1.2)
                .with_rotation(Quat::from_rotation_x(-0.3)),
            ..default()
        });
    });
    lm
}
#[derive(Component)]
pub struct CmInterior;

#[derive(Component)]
pub struct LaunchPad;
fn spawn_cm_interior(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    cache: &CmMeshCache,
    parent: Entity,
) -> Entity {
    use crate::config::*;
    
    let material_cache = CmMaterialCache {
        wall: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.53, 0.50),
            metallic: 0.0,
            perceptual_roughness: 0.85,
            cull_mode: None,
            ..default()
        })),
        panel: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.28, 0.30),
            metallic: 0.1,
            perceptual_roughness: 0.5,
            cull_mode: None,
            ..default()
        })),
        panel_highlight: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            metallic: 0.1,
            perceptual_roughness: 0.6,
            cull_mode: None,
            ..default()
        })),
        seat: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.22, 0.18),
            metallic: 0.0,
            perceptual_roughness: 0.95,
            cull_mode: None,
            ..default()
        })),
        dsky: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.22, 0.22),
            metallic: 0.0,
            perceptual_roughness: 0.3,
            cull_mode: None,
            ..default()
        })),
        fdai: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.18),
            metallic: 0.0,
            perceptual_roughness: 0.4,
            cull_mode: None,
            ..default()
        })),
        display_glow: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.35, 0.25),
            emissive: LinearRgba::new(0.25, 0.75, 0.50, 1.0),
            metallic: 0.0,
            perceptual_roughness: 0.2,
            cull_mode: None,
            ..default()
        })),
        window: Some(materials.add(StandardMaterial {
            base_color: Color::srgba(0.4, 0.6, 0.85, 0.25),
            metallic: 0.0,
            perceptual_roughness: 0.05,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        })),
        switch: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 0.72),
            metallic: 1.0,
            perceptual_roughness: 0.3,
            emissive: LinearRgba::new(0.03, 0.03, 0.03, 0.1),
            cull_mode: None,
            ..default()
        })),
        tunnel: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.45, 0.47),
            metallic: 0.0,
            perceptual_roughness: 0.6,
            cull_mode: None,
            ..default()
        })),
        heat_shield: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.30, 0.20),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            cull_mode: None,
            ..default()
        })),
    };
    
    let m = |opt: &Option<Handle<StandardMaterial>>| opt.as_ref().unwrap().clone();
    
    let cm_y_center = -SATURN_V_TOTAL_HEIGHT * 0.5 + crate::config::CM_CENTER_OFFSET;
    
    let interior = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, cm_y_center, 0.0)
                .with_scale(Vec3::splat(SATURN_V_SCALE)),
            ..default()
        },
        CmInterior,
        Name::new("CM Interior"),
    )).id();
    commands.entity(parent).add_child(interior);

    commands.entity(interior).with_children(|parent| {
        spawn_hull(parent, meshes, m(&material_cache.wall), m(&material_cache.heat_shield), m(&material_cache.tunnel));
        spawn_furniture(parent, cache, m(&material_cache.seat), m(&material_cache.switch));
        spawn_console(parent, meshes, materials, m(&material_cache.panel), m(&material_cache.panel_highlight), m(&material_cache.dsky), m(&material_cache.fdai), m(&material_cache.display_glow));
        spawn_windows(parent, meshes, materials, m(&material_cache.window));
        spawn_interior_lighting(parent);
        
        crate::panels::spawn_historical_panels(
            parent, meshes, materials,
            CONSOLE_Y, CONSOLE_Z, CONSOLE_WIDTH, CONSOLE_HEIGHT, CONSOLE_DEPTH
        );
        
        crate::panels::spawn_electrical_connectors(parent, meshes, materials);
    });
    
    interior
}

fn spawn_hull(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    wall_mat: Handle<StandardMaterial>,
    heat_shield_mat: Handle<StandardMaterial>,
    tunnel_mat: Handle<StandardMaterial>,
) {
    use crate::config::*;
    
    let heat_shield = meshes.add(Cylinder::new(CM_BASE_RADIUS, 0.15));
    parent.spawn(PbrBundle {
        mesh: heat_shield,
        material: heat_shield_mat.clone(),
        transform: Transform::from_xyz(0.0, FLOOR_Y - 0.075, 0.0),
        ..default()
    });

    let floor = meshes.add(Cylinder::new(CM_BASE_RADIUS * 0.95, 0.06));
    parent.spawn(PbrBundle {
        mesh: floor,
        material: wall_mat.clone(),
        transform: Transform::from_xyz(0.0, FLOOR_Y, 0.0),
        ..default()
    });

    let leb_floor = meshes.add(Cuboid::new(0.9, 0.05, 0.8));
    parent.spawn(PbrBundle {
        mesh: leb_floor,
        material: wall_mat.clone(),
        transform: Transform::from_xyz(0.0, FLOOR_Y + 0.025, 0.5),
        ..default()
    });

    let hull_mesh = create_conical_frustum_mesh(
        meshes,
        CM_BASE_RADIUS,
        CM_TOP_RADIUS,
        FLOOR_Y,
        CEILING_Y,
        32,
    );
    parent.spawn(PbrBundle {
        mesh: hull_mesh,
        material: wall_mat.clone(),
        transform: Transform::IDENTITY,
        ..default()
    });

    let forward_height = CM_HEIGHT - CEILING_Y;
    let ceiling_cone = meshes.add(Cone { radius: CM_TOP_RADIUS, height: forward_height * 0.7 });
    parent.spawn(PbrBundle {
        mesh: ceiling_cone,
        material: wall_mat.clone(),
        transform: Transform::from_xyz(0.0, CEILING_Y + forward_height * 0.35, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        ..default()
    });

    let tunnel = meshes.add(Cylinder::new(TUNNEL_RADIUS, TUNNEL_LENGTH));
    parent.spawn(PbrBundle {
        mesh: tunnel,
        material: tunnel_mat.clone(),
        transform: Transform::from_xyz(0.0, CM_HEIGHT - 0.35, 0.0),
        ..default()
    });
    
    let tunnel_hatch = meshes.add(Cylinder::new(TUNNEL_RADIUS + 0.02, 0.04));
    parent.spawn(PbrBundle {
        mesh: tunnel_hatch,
        material: tunnel_mat.clone(),
        transform: Transform::from_xyz(0.0, CM_HEIGHT - 0.02, 0.0),
        ..default()
    });

    let hatch_angle = -std::f32::consts::FRAC_PI_2;
    let hatch_radius = CM_BASE_RADIUS * 0.88;
    let hatch_x = hatch_angle.cos() * hatch_radius;
    let hatch_z = hatch_angle.sin() * hatch_radius;
    let side_hatch = meshes.add(Cuboid::new(0.04, HATCH_HEIGHT, HATCH_WIDTH));
    parent.spawn(PbrBundle {
        mesh: side_hatch,
        material: tunnel_mat.clone(),
        transform: Transform::from_xyz(hatch_x, HATCH_Y, hatch_z)
            .with_rotation(Quat::from_rotation_y(hatch_angle)),
        ..default()
    });
}

fn spawn_furniture(
    parent: &mut ChildBuilder,
    cache: &CmMeshCache,
    seat_mat: Handle<StandardMaterial>,
    switch_mat: Handle<StandardMaterial>,
) {
    use crate::config::*;
    
    spawn_couch(parent, cache, seat_mat.clone(), -COUCH_SPACING, COUCH_Y, COUCH_Z, 0.08, COUCH_WIDTH, COUCH_DEPTH);
    spawn_couch(parent, cache, seat_mat.clone(), 0.0, COUCH_Y, COUCH_Z + 0.15, 0.0, COUCH_WIDTH, COUCH_DEPTH);
    spawn_couch(parent, cache, seat_mat.clone(), COUCH_SPACING, COUCH_Y, COUCH_Z, -0.08, COUCH_WIDTH, COUCH_DEPTH);

    let rhc = cache.switch.as_ref().unwrap().clone();
    parent.spawn(PbrBundle {
        mesh: rhc.clone(),
        material: switch_mat.clone(),
        transform: Transform::from_xyz(-COUCH_SPACING + 0.32, COUCH_Y + 0.25, COUCH_Z + 0.1)
            .with_rotation(Quat::from_rotation_x(-0.15)),
        ..default()
    });
    parent.spawn(PbrBundle {
        mesh: rhc.clone(),
        material: switch_mat.clone(),
        transform: Transform::from_xyz(COUCH_SPACING - 0.32, COUCH_Y + 0.25, COUCH_Z + 0.1)
            .with_rotation(Quat::from_rotation_x(-0.15)),
        ..default()
    });

    let thc = cache.switch.as_ref().unwrap().clone();
    parent.spawn(PbrBundle {
        mesh: thc,
        material: switch_mat.clone(),
        transform: Transform::from_xyz(-COUCH_SPACING - 0.35, COUCH_Y + 0.28, COUCH_Z + 0.15)
            .with_rotation(Quat::from_rotation_z(0.35)),
        ..default()
    });

    let sextant = cache.switch.as_ref().unwrap().clone();
    parent.spawn(PbrBundle {
        mesh: sextant,
        material: switch_mat,
        transform: Transform::from_xyz(0.0, FLOOR_Y + 0.25, 0.6)
            .with_rotation(Quat::from_rotation_x(-0.5)),
        ..default()
    });
}

fn spawn_console(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    panel_mat: Handle<StandardMaterial>,
    highlight_mat: Handle<StandardMaterial>,
    dsky_mat: Handle<StandardMaterial>,
    fdai_mat: Handle<StandardMaterial>,
    glow_mat: Handle<StandardMaterial>,
) {
    use crate::config::*;
    
    let main_panel = meshes.add(Cuboid::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, CONSOLE_DEPTH));
    parent.spawn(PbrBundle {
        mesh: main_panel,
        material: panel_mat.clone(),
        transform: Transform::from_xyz(0.0, CONSOLE_Y, CONSOLE_Z)
            .with_rotation(Quat::from_rotation_x(-0.1)),
        ..default()
    });

    for side in [-1.0f32, 1.0] {
        let wing = meshes.add(Cuboid::new(WING_WIDTH, CONSOLE_HEIGHT, WING_DEPTH));
        let wing_angle = side * WING_ANGLE_DEG.to_radians();
        let wing_x = side * (CONSOLE_WIDTH * 0.5 + WING_WIDTH * 0.4 * wing_angle.cos());
        let wing_z = CONSOLE_Z + WING_WIDTH * 0.4 * wing_angle.sin().abs() + 0.15;
        parent.spawn(PbrBundle {
            mesh: wing,
            material: panel_mat.clone(),
            transform: Transform::from_xyz(wing_x, CONSOLE_Y, wing_z)
                .with_rotation(Quat::from_rotation_y(wing_angle)),
            ..default()
        });
    }

    let dsky_box = meshes.add(Cuboid::new(DSKY_WIDTH, DSKY_HEIGHT, DSKY_DEPTH));
    let dsky_entity = parent.spawn((
        PbrBundle {
            mesh: dsky_box,
            material: dsky_mat,
            transform: Transform::from_xyz(0.0, CONSOLE_Y - 0.05, CONSOLE_Z + CONSOLE_DEPTH * 0.5 + DSKY_DEPTH * 0.5)
                .with_rotation(Quat::from_rotation_x(-0.1)),
            ..default()
        },
        crate::panels::DskyDisplay::default(),
    )).id();

    spawn_dsky_keys(parent, meshes, materials, dsky_entity);

    let fdae_housing = meshes.add(Cuboid::new(FDAI_SIZE, FDAI_SIZE, 0.12));
    let fdae_sphere = meshes.add(Sphere::new(FDAI_SPHERE_RADIUS));
    for side in [-1.0f32, 1.0] {
        let fdae_x = side * 0.55;
        parent.spawn(PbrBundle {
            mesh: fdae_housing.clone(),
            material: fdai_mat.clone(),
            transform: Transform::from_xyz(fdae_x, CONSOLE_Y + 0.1, CONSOLE_Z + CONSOLE_DEPTH * 0.5 + 0.06)
                .with_rotation(Quat::from_rotation_x(-0.1)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: fdae_sphere.clone(),
            material: glow_mat.clone(),
            transform: Transform::from_xyz(fdae_x, CONSOLE_Y + 0.1, CONSOLE_Z + CONSOLE_DEPTH * 0.5 + 0.1)
                .with_rotation(Quat::from_rotation_x(-0.1)),
            ..default()
        });
    }

    let timer_box = meshes.add(Cuboid::new(TIMER_WIDTH, TIMER_HEIGHT, 0.08));
    parent.spawn(PbrBundle {
        mesh: timer_box,
        material: highlight_mat,
        transform: Transform::from_xyz(0.0, CONSOLE_Y + 0.32, CONSOLE_Z + CONSOLE_DEPTH * 0.5 + 0.04)
            .with_rotation(Quat::from_rotation_x(-0.1)),
        ..default()
    });
}

fn spawn_windows(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    window_mat: Handle<StandardMaterial>,
) {
    use crate::config::*;
    
    let frame_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.17),
        metallic: 0.8,
        perceptual_roughness: 0.4,
        cull_mode: None,
        ..default()
    });
    let bezel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.09),
        metallic: 0.5,
        perceptual_roughness: 0.6,
        cull_mode: None,
        ..default()
    });
    
    let rendezvous_radius = get_conical_radius_at_height(CM_BASE_RADIUS, CM_TOP_RADIUS, FLOOR_Y, CEILING_Y, RENDEZVOUS_WINDOW_Y);
    for side in [-1.0f32, 1.0] {
        let angle = side * RENDEZVOUS_WINDOW_ANGLE_DEG.to_radians();
        let wx = angle.cos() * rendezvous_radius;
        let wz = angle.sin() * rendezvous_radius - 0.1;
        let window_xform = Transform::from_xyz(wx, RENDEZVOUS_WINDOW_Y, wz)
            .with_rotation(Quat::from_rotation_y(angle));
        
        let frame = meshes.add(Cuboid::new(RENDEZVOUS_WINDOW_WIDTH + 0.08, RENDEZVOUS_WINDOW_HEIGHT + 0.08, 0.06));
        parent.spawn(PbrBundle {
            mesh: frame,
            material: frame_mat.clone(),
            transform: window_xform,
            ..default()
        });
        
        let bezel = meshes.add(Cuboid::new(RENDEZVOUS_WINDOW_WIDTH + 0.03, RENDEZVOUS_WINDOW_HEIGHT + 0.03, 0.08));
        parent.spawn(PbrBundle {
            mesh: bezel,
            material: bezel_mat.clone(),
            transform: window_xform,
            ..default()
        });
        
        let window_frame = meshes.add(Cuboid::new(RENDEZVOUS_WINDOW_WIDTH, RENDEZVOUS_WINDOW_HEIGHT, 0.04));
        parent.spawn(PbrBundle {
            mesh: window_frame,
            material: window_mat.clone(),
            transform: window_xform,
            ..default()
        });
    }

    let side_radius = get_conical_radius_at_height(CM_BASE_RADIUS, CM_TOP_RADIUS, FLOOR_Y, CEILING_Y, SIDE_WINDOW_Y);
    for side in [-1.0f32, 1.0] {
        let angle = side * std::f32::consts::FRAC_PI_2;
        let wx = angle.cos() * side_radius;
        let wz = angle.sin() * side_radius;
        let window_xform = Transform::from_xyz(wx, SIDE_WINDOW_Y, wz)
            .with_rotation(Quat::from_rotation_y(angle));
        
        let frame = meshes.add(Cuboid::new(0.06, SIDE_WINDOW_SIZE + 0.08, SIDE_WINDOW_SIZE + 0.08));
        parent.spawn(PbrBundle {
            mesh: frame,
            material: frame_mat.clone(),
            transform: window_xform,
            ..default()
        });
        
        let bezel = meshes.add(Cuboid::new(0.08, SIDE_WINDOW_SIZE + 0.03, SIDE_WINDOW_SIZE + 0.03));
        parent.spawn(PbrBundle {
            mesh: bezel,
            material: bezel_mat.clone(),
            transform: window_xform,
            ..default()
        });
        
        let window_frame = meshes.add(Cuboid::new(0.04, SIDE_WINDOW_SIZE, SIDE_WINDOW_SIZE));
        parent.spawn(PbrBundle {
            mesh: window_frame,
            material: window_mat.clone(),
            transform: window_xform,
            ..default()
        });
    }
}

fn spawn_interior_lighting(parent: &mut ChildBuilder) {
    use crate::config::*;
    
    parent.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: INTERIOR_LIGHT_INTENSITY,
            color: Color::srgb(0.95, 0.92, 0.85),
            range: INTERIOR_LIGHT_RANGE,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, CONSOLE_Y + 0.4, -0.3),
        ..default()
    });
    parent.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: OVERHEAD_LIGHT_INTENSITY,
            color: Color::srgb(0.9, 0.9, 0.85),
            range: OVERHEAD_LIGHT_RANGE,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, CEILING_Y * 0.7, 0.0),
        ..default()
    });
}

/// Create a smooth conical frustum mesh
fn create_conical_frustum_mesh(
    meshes: &mut Assets<Mesh>,
    base_radius: f32,
    top_radius: f32,
    bottom_y: f32,
    top_y: f32,
    segments: u32,
) -> Handle<Mesh> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    
    let height = top_y - bottom_y;
    let slope = (base_radius - top_radius) / height;
    let angle_step = std::f32::consts::TAU / segments as f32;
    
    for i in 0..=segments {
        let angle = i as f32 * angle_step;
        let cos = angle.cos();
        let sin = angle.sin();
        
        // Bottom ring
        vertices.push([base_radius * cos, bottom_y, base_radius * sin]);
        let inward_normal = Vec3::new(-cos, slope, -sin).normalize();
        normals.push([inward_normal.x, inward_normal.y, inward_normal.z]);
        uvs.push([i as f32 / segments as f32, 0.0]);
        
        // Top ring
        vertices.push([top_radius * cos, top_y, top_radius * sin]);
        let inward_normal = Vec3::new(-cos, slope, -sin).normalize();
        normals.push([inward_normal.x, inward_normal.y, inward_normal.z]);
        uvs.push([i as f32 / segments as f32, 1.0]);
    }
    
    for i in 0..segments {
        let base = i * 2;
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 1);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base + 3);
    }
    
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    meshes.add(mesh)
}

/// Get the radius of the conical hull at a given height
fn get_conical_radius_at_height(
    base_radius: f32,
    top_radius: f32,
    bottom_y: f32,
    top_y: f32,
    y: f32,
) -> f32 {
    let t = (y - bottom_y) / (top_y - bottom_y);
    base_radius + (top_radius - base_radius) * t
}

fn spawn_couch(
    parent: &mut ChildBuilder,
    cache: &CmMeshCache,
    material: Handle<StandardMaterial>,
    x: f32,
    y: f32,
    z: f32,
    rotation_y: f32,
    _width: f32,
    _depth: f32,
) {
    use crate::config::*;
    
    let seat = cache.couch_seat.as_ref().unwrap().clone();
    let back = cache.couch_back.as_ref().unwrap().clone();
    let headrest = cache.couch_headrest.as_ref().unwrap().clone();
    let armrest = cache.couch_armrest.as_ref().unwrap().clone();
    let rotation = Quat::from_rotation_y(rotation_y);
    
    // Seat pan
    parent.spawn(PbrBundle {
        mesh: seat,
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z).with_rotation(rotation),
        ..default()
    });
    
    // Backrest (angled at 85 degrees from horizontal = 5 degrees from vertical)
    let back_angle = 5f32.to_radians();
    parent.spawn(PbrBundle {
        mesh: back,
        material: material.clone(),
        transform: Transform::from_xyz(x, y + 0.32, z - COUCH_DEPTH * 0.4)
            .with_rotation(rotation * Quat::from_rotation_x(-back_angle)),
        ..default()
    });
    
    // Headrest (adjustable 16cm travel)
    parent.spawn(PbrBundle {
        mesh: headrest,
        material: material.clone(),
        transform: Transform::from_xyz(x, y + 0.68, z - COUCH_DEPTH * 0.42)
            .with_rotation(rotation * Quat::from_rotation_x(-back_angle)),
        ..default()
    });
    
    // Left armrest
    let left_arm_x = x - (rotation_y.cos() * (COUCH_WIDTH * 0.5 + 0.04));
    let left_arm_z = z + (rotation_y.sin() * (COUCH_WIDTH * 0.5 + 0.04));
    parent.spawn(PbrBundle {
        mesh: armrest.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(left_arm_x, y + 0.18, left_arm_z).with_rotation(rotation),
        ..default()
    });
    
    // Right armrest
    let right_arm_x = x + (rotation_y.cos() * (COUCH_WIDTH * 0.5 + 0.04));
    let right_arm_z = z - (rotation_y.sin() * (COUCH_WIDTH * 0.5 + 0.04));
    parent.spawn(PbrBundle {
        mesh: armrest.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(right_arm_x, y + 0.18, right_arm_z).with_rotation(rotation),
        ..default()
    });
}
fn spawn_dsky_keys(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _dsky_entity: Entity,
) {
    use crate::panels::{DskyKey, DskyKeyType, InteractivePanel};
    use crate::config::*;

    let key_mesh = meshes.add(Cuboid::new(0.04, 0.025, 0.015));
    let key_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.65, 0.7),
        metallic: 0.6,
        perceptual_roughness: 0.4,
        ..default()
    });

    let dsky_base_x = 0.0;
    let dsky_base_y = CONSOLE_Y - 0.05;
    let dsky_base_z = CONSOLE_Z + CONSOLE_DEPTH * 0.5 + DSKY_DEPTH * 0.5;
    let dsky_rotation = Quat::from_rotation_x(-0.1);

    let key_layout = [
        (DskyKeyType::Verb, -0.12, 0.12),
        (DskyKeyType::Noun, -0.04, 0.12),
        (DskyKeyType::Plus, 0.04, 0.12),
        (DskyKeyType::Minus, 0.12, 0.12),
        (DskyKeyType::Number(1), -0.12, 0.06),
        (DskyKeyType::Number(2), -0.04, 0.06),
        (DskyKeyType::Number(3), 0.04, 0.06),
        (DskyKeyType::Number(4), -0.12, 0.0),
        (DskyKeyType::Number(5), -0.04, 0.0),
        (DskyKeyType::Number(6), 0.04, 0.0),
        (DskyKeyType::Number(7), -0.12, -0.06),
        (DskyKeyType::Number(8), -0.04, -0.06),
        (DskyKeyType::Number(9), 0.04, -0.06),
        (DskyKeyType::Clear, -0.12, -0.12),
        (DskyKeyType::Number(0), -0.04, -0.12),
        (DskyKeyType::Enter, 0.04, -0.12),
        (DskyKeyType::Reset, -0.12, -0.18),
        (DskyKeyType::Pro, -0.04, -0.18),
        (DskyKeyType::KeyRel, 0.04, -0.18),
    ];

    for (key_type, lx, ly) in key_layout {
        let local_pos = Vec3::new(lx, ly, DSKY_DEPTH * 0.5 + 0.01);
        let world_pos = dsky_rotation * local_pos + Vec3::new(dsky_base_x, dsky_base_y, dsky_base_z);

        parent.spawn((
            PbrBundle {
                mesh: key_mesh.clone(),
                material: key_material.clone(),
                transform: Transform::from_translation(world_pos).with_rotation(dsky_rotation),
                ..default()
            },
            DskyKey { key: key_type },
            InteractivePanel,
        ));
    }
}

fn update_spacecraft_transform(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut query: Query<(&Spacecraft, &mut Transform)>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;
    for (spacecraft, mut transform) in query.iter_mut() {
        transform.translation += spacecraft.velocity * dt;
    }
}

fn animate_swing_arm_retraction(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut launch_query: Query<&mut LaunchController>,
    mut arm_query: Query<(&mut SwingArm, &mut Transform)>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;
    for mut controller in launch_query.iter_mut() {
        if controller.state != LaunchState::Countdown {
            continue;
        }
        controller.mission_time += dt;
        let retract_progress = (controller.mission_time / 2.0).min(1.0);
        for (mut arm, mut transform) in arm_query.iter_mut() {
            arm.retracted = retract_progress;
            let angle = retract_progress * std::f32::consts::FRAC_PI_2;
            let hinge_x = arm.tower_offset;
            let direction = if hinge_x < 0.0 { 1.0 } else { -1.0 };
            let arm_end_x = hinge_x + direction * arm.arm_length * angle.cos();
            let arm_end_z = -arm.arm_length * angle.sin();
            transform.translation.x = (hinge_x + arm_end_x) * 0.5;
            transform.translation.z = arm_end_z * 0.5;
            transform.rotation = Quat::from_rotation_y(direction * -angle);
        }
        if retract_progress >= 1.0 {
            controller.state = LaunchState::Ignition;
            controller.mission_time = 0.0;
            info!("LAUNCH: Swing arms retracted, ignition sequence starting");
        }
    }
}
