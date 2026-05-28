use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_launch_site)
            .add_systems(Startup, spawn_earth)
            .add_systems(Startup, spawn_moon)
            .add_systems(Startup, spawn_launch_complex)
            .add_systems(Update, rotate_earth.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, orbit_moon.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, toggle_earth_visibility);
    }
}

fn init_launch_site(mut commands: Commands) {
    let lat = LAUNCH_SITE_LAT;
    let lon = LAUNCH_SITE_LON;
    let r = EARTH_RADIUS;
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.sin();
    let normal = Vec3::new(x, y, z).normalize();
    let position = Vec3::new(x, y, z) + normal * 0.02;
    commands.insert_resource(LaunchSitePosition(position));
}

fn toggle_earth_visibility(
    launch_query: Query<&crate::spacecraft::LaunchController>,
    mut earth_query: Query<&mut Visibility, With<Earth>>,
) {
    let show_earth = if let Ok(launch) = launch_query.get_single() {
        match launch.state {
            crate::spacecraft::LaunchState::OnPad | crate::spacecraft::LaunchState::Countdown => false,
            _ => true,
        }
    } else {
        false
    };
    
    for mut visibility in earth_query.iter_mut() {
        *visibility = if show_earth { Visibility::Visible } else { Visibility::Hidden };
    }
}

#[derive(Component)]
pub struct Earth;

#[derive(Component)]
pub struct Moon {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_of_periapsis: f64,
    pub mean_anomaly_at_epoch: f64,
    pub epoch_jd: f64,
    pub mean_motion: f64,
}

#[derive(Component)]
pub struct LaunchComplex;

#[derive(Resource)]
pub struct LaunchSitePosition(pub Vec3);

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
        emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: earth_mesh,
            material: earth_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        Earth,
        Name::new("Earth"),
        NotShadowCaster,
        NotShadowReceiver,
    ));

    let atmo_mesh = meshes.add(Sphere::new(EARTH_RADIUS * 1.05).mesh().ico(8).unwrap());
    let atmo_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.8, 1.0, 0.008),
        emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: atmo_mesh,
            material: atmo_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        Earth,
        Name::new("Earth Atmosphere"),
        NotShadowCaster,
        NotShadowReceiver,
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
    let normal = Vec3::new(x, y, z).normalize();
    let position = Vec3::new(x, y, z) + normal * 0.02;
    let up = Vec3::Y;
    let right = normal.cross(up).normalize();
    let forward = right.cross(normal);
    
    let rotation_matrix = Mat3::from_cols(right, normal, forward);
    let rotation = Quat::from_mat3(&rotation_matrix);

    let concrete_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.70, 0.70, 0.70),
        metallic: 0.0,
        perceptual_roughness: 0.95,
        reflectance: 0.02,
        ..default()
    });

    let dark_concrete_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.45, 0.45),
        metallic: 0.0,
        perceptual_roughness: 0.95,
        reflectance: 0.02,
        ..default()
    });

    let refractory_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.22, 0.16),
        metallic: 0.1,
        perceptual_roughness: 0.85,
        reflectance: 0.0,
        ..default()
    });

    let steel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.52, 0.32, 0.22),
        metallic: 0.0,
        perceptual_roughness: 0.75,
        reflectance: 0.04,
        ..default()
    });

    let dark_steel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.28, 0.20),
        metallic: 0.0,
        perceptual_roughness: 0.80,
        reflectance: 0.04,
        ..default()
    });

    let ground_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.45, 0.18),
        metallic: 0.0,
        perceptual_roughness: 0.95,
        reflectance: 0.0,
        ..default()
    });

    let crawlerway_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.58, 0.55, 0.50),
        metallic: 0.0,
        perceptual_roughness: 0.9,
        reflectance: 0.0,
        ..default()
    });

    let vab_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.68, 0.68, 0.65),
        metallic: 0.08,
        perceptual_roughness: 0.82,
        reflectance: 0.0,
        ..default()
    });

    let tree_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.45, 0.12),
        metallic: 0.0,
        perceptual_roughness: 0.9,
        reflectance: 0.0,
        ..default()
    });

    let warning_red_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.12, 0.08),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        reflectance: 0.0,
        ..default()
    });

    let launch_complex = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position).with_rotation(rotation),
            ..default()
        },
        LaunchComplex,
        Name::new("Launch Complex 39A"),
    )).id();

    commands.insert_resource(LaunchSitePosition(position));

    commands.entity(launch_complex).with_children(|parent| {
        let terrain_mesh = meshes.add(create_flat_terrain_mesh(160.0, 32));
    let sand_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.72, 0.58),
        metallic: 0.0,
        perceptual_roughness: 0.92,
        reflectance: 0.0,
        ..default()
    });
        parent.spawn(PbrBundle {
            mesh: terrain_mesh,
            material: sand_mat.clone(),
            transform: Transform::from_xyz(0.0, -0.02, 0.0),
            ..default()
        });

        let beach_mesh = meshes.add(create_flat_terrain_mesh(160.0, 16));
    let beach_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.80, 0.68),
        metallic: 0.0,
        perceptual_roughness: 0.88,
        reflectance: 0.0,
        ..default()
    });
        parent.spawn(PbrBundle {
            mesh: beach_mesh,
            material: beach_mat,
            transform: Transform::from_xyz(90.0, -0.03, 0.0),
            ..default()
        });

        let water_mesh = meshes.add(create_flat_terrain_mesh(200.0, 8));
    let water_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.50, 0.65),
        metallic: 0.15,
        perceptual_roughness: 0.15,
        reflectance: 0.0,
        ..default()
    });
        parent.spawn(PbrBundle {
            mesh: water_mesh,
            material: water_mat,
            transform: Transform::from_xyz(180.0, -0.15, 0.0),
            ..default()
        });

        let hardstand_radius = 5.5_f32;
        let hardstand_thickness = 0.35_f32;
        let hardstand_mesh = meshes.add(create_octagonal_prism_mesh(hardstand_radius, hardstand_thickness));
        parent.spawn(PbrBundle {
            mesh: hardstand_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(0.0, hardstand_thickness * 0.5, 0.0),
            ..default()
        });

        let mlp_w = 4.9_f32;
        let mlp_d = 4.1_f32;
        let mlp_h = 0.76_f32;
        let hole_w = 1.4_f32;
        let hole_d = 1.4_f32;

        let rim_n_s_depth = (mlp_d - hole_d) * 0.5;
        let rim_e_w_width = (mlp_w - hole_w) * 0.5;

        let mlp_n_mesh = meshes.add(Cuboid::new(mlp_w, mlp_h, rim_n_s_depth));
        parent.spawn(PbrBundle {
            mesh: mlp_n_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(0.0, mlp_h * 0.5, -(hole_d + rim_n_s_depth) * 0.5),
            ..default()
        });

        let mlp_s_mesh = meshes.add(Cuboid::new(mlp_w, mlp_h, rim_n_s_depth));
        parent.spawn(PbrBundle {
            mesh: mlp_s_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(0.0, mlp_h * 0.5, (hole_d + rim_n_s_depth) * 0.5),
            ..default()
        });

        let mlp_w_mesh = meshes.add(Cuboid::new(rim_e_w_width, mlp_h, hole_d));
        parent.spawn(PbrBundle {
            mesh: mlp_w_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(-(hole_w + rim_e_w_width) * 0.5, mlp_h * 0.5, 0.0),
            ..default()
        });

        let mlp_e_mesh = meshes.add(Cuboid::new(rim_e_w_width, mlp_h, hole_d));
        parent.spawn(PbrBundle {
            mesh: mlp_e_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz((hole_w + rim_e_w_width) * 0.5, mlp_h * 0.5, 0.0),
            ..default()
        });

        let post_radius = 0.06_f32;
        let post_height = 0.25_f32;
        let post_mesh = meshes.add(Cylinder::new(post_radius, post_height));
        let post_cap_mesh = meshes.add(Cylinder::new(post_radius * 1.4, 0.04));

        let post_positions = [
            (-mlp_w * 0.45, -mlp_d * 0.45),
            ( mlp_w * 0.45, -mlp_d * 0.45),
            (-mlp_w * 0.45,  mlp_d * 0.45),
            ( mlp_w * 0.45,  mlp_d * 0.45),
            ( 0.0,          -mlp_d * 0.45),
            ( 0.0,           mlp_d * 0.45),
        ];

        for (px, pz) in post_positions {
            parent.spawn(PbrBundle {
                mesh: post_mesh.clone(),
                material: dark_steel_mat.clone(),
                transform: Transform::from_xyz(px, mlp_h + post_height * 0.5, pz),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: post_cap_mesh.clone(),
                material: warning_red_mat.clone(),
                transform: Transform::from_xyz(px, mlp_h + post_height + 0.02, pz),
                ..default()
            });
        }

        let trench_w = 1.8_f32;
        let trench_d = 1.35_f32;
        let trench_l = 8.0_f32;
        let wall_thick = 0.25_f32;

        let trench_floor_mesh = meshes.add(Cuboid::new(trench_w, 0.15, trench_l));
        parent.spawn(PbrBundle {
            mesh: trench_floor_mesh,
            material: dark_concrete_mat.clone(),
            transform: Transform::from_xyz(0.0, -trench_d + 0.075, trench_l * 0.30),
            ..default()
        });

        let brick_floor_mesh = meshes.add(Cuboid::new(trench_w - 0.10, 0.08, trench_l - 0.10));
        parent.spawn(PbrBundle {
            mesh: brick_floor_mesh,
            material: refractory_mat.clone(),
            transform: Transform::from_xyz(0.0, -trench_d + 0.12, trench_l * 0.30),
            ..default()
        });

        let wall_e_mesh = meshes.add(Cuboid::new(wall_thick, trench_d, trench_l));
        parent.spawn(PbrBundle {
            mesh: wall_e_mesh.clone(),
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(
                (trench_w + wall_thick) * 0.5,
                -trench_d * 0.5 + 0.02,
                trench_l * 0.30,
            ),
            ..default()
        });

        let brick_e_mesh = meshes.add(Cuboid::new(0.08, trench_d - 0.15, trench_l - 0.10));
        parent.spawn(PbrBundle {
            mesh: brick_e_mesh,
            material: refractory_mat.clone(),
            transform: Transform::from_xyz(
                (trench_w - wall_thick) * 0.5 + 0.02,
                -trench_d * 0.5 + 0.02,
                trench_l * 0.30,
            ),
            ..default()
        });

        let wall_w_mesh = meshes.add(Cuboid::new(wall_thick, trench_d, trench_l));
        parent.spawn(PbrBundle {
            mesh: wall_w_mesh.clone(),
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(
                -(trench_w + wall_thick) * 0.5,
                -trench_d * 0.5 + 0.02,
                trench_l * 0.30,
            ),
            ..default()
        });

        let brick_w_mesh = meshes.add(Cuboid::new(0.08, trench_d - 0.15, trench_l - 0.10));
        parent.spawn(PbrBundle {
            mesh: brick_w_mesh,
            material: refractory_mat.clone(),
            transform: Transform::from_xyz(
                -(trench_w - wall_thick) * 0.5 - 0.02,
                -trench_d * 0.5 + 0.02,
                trench_l * 0.30,
            ),
            ..default()
        });

        let wall_n_mesh = meshes.add(Cuboid::new(trench_w + wall_thick * 2.0, trench_d, wall_thick));
        parent.spawn(PbrBundle {
            mesh: wall_n_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(
                0.0,
                -trench_d * 0.5 + 0.02,
                -trench_l * 0.20 + wall_thick * 0.5,
            ),
            ..default()
        });

        let deflector_w = trench_w * 0.85;
        let deflector_h = 1.6_f32;
        let deflector_d = 1.0_f32;
        let deflector_mesh = meshes.add(Cuboid::new(deflector_w, deflector_h, deflector_d));
        parent.spawn(PbrBundle {
            mesh: deflector_mesh,
            material: refractory_mat.clone(),
            transform: Transform::from_xyz(0.0, -trench_d * 0.35, trench_l * 0.72)
                .with_rotation(Quat::from_rotation_x(-0.55)),
            ..default()
        });

        let tower_height = 13.5_f32;
        let tower_offset_x = -5.2_f32;
        let tower_base_w = 2.4_f32;
        let tower_base_d = 2.4_f32;
        let column_radius = 0.12_f32;

        let main_col_mesh = meshes.add(Cylinder::new(column_radius, tower_height));
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let half_w = tower_base_w * 0.5;
            let half_d = tower_base_d * 0.5;
            let cx = tower_offset_x + angle.cos() * half_w;
            let cz = angle.sin() * half_d;
            parent.spawn(PbrBundle {
                mesh: main_col_mesh.clone(),
                material: steel_mat.clone(),
                transform: Transform::from_xyz(cx, tower_height * 0.5, cz),
                ..default()
            });
        }

        let brace_levels = 18;
        for level in 0..brace_levels {
            let level_y = (level as f32 / brace_levels as f32) * tower_height + 0.4;
            let taper = 1.0 - (level as f32 / brace_levels as f32) * 0.18;
            let level_w = tower_base_w * taper;
            let level_d = tower_base_d * taper;
            let half_w = level_w * 0.5;
            let half_d = level_d * 0.5;

            let brace_thick = 0.045_f32;

            for face in 0..4 {
                let angle = (face as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
                let next_angle = ((face + 1) as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;

                let x1 = tower_offset_x + angle.cos() * half_w;
                let z1 = angle.sin() * half_d;
                let x2 = tower_offset_x + next_angle.cos() * half_w;
                let z2 = next_angle.sin() * half_d;

                let dx = x2 - x1;
                let dz = z2 - z1;
                let edge_len = (dx * dx + dz * dz).sqrt();
                let edge_mesh = meshes.add(Cuboid::new(edge_len, brace_thick, brace_thick));
                let mid_x = (x1 + x2) * 0.5;
                let mid_z = (z1 + z2) * 0.5;
                let edge_yaw = dz.atan2(dx);

                parent.spawn(PbrBundle {
                    mesh: edge_mesh,
                    material: steel_mat.clone(),
                    transform: Transform::from_xyz(mid_x, level_y, mid_z)
                        .with_rotation(Quat::from_rotation_y(-edge_yaw)),
                    ..default()
                });
            }

            if level < brace_levels - 1 {
                let v_height = tower_height / brace_levels as f32;
                let v_col_mesh = meshes.add(Cylinder::new(0.035, v_height));
                for i in 0..4 {
                    let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
                    let cx = tower_offset_x + angle.cos() * half_w;
                    let cz = angle.sin() * half_d;
                    parent.spawn(PbrBundle {
                        mesh: v_col_mesh.clone(),
                        material: steel_mat.clone(),
                        transform: Transform::from_xyz(cx, level_y + v_height * 0.5, cz),
                        ..default()
                    });
                }
            }

            let diag_len = ((half_w * 2.0).powi(2) + (tower_height / brace_levels as f32).powi(2)).sqrt();
            let diag_mesh = meshes.add(Cuboid::new(diag_len, 0.03, 0.03));
            let diag_angle = (tower_height / brace_levels as f32).atan2(half_w * 2.0);

            for face in 0..2 {
                let face_angle = face as f32 * std::f32::consts::FRAC_PI_2;
                let fx = tower_offset_x + face_angle.cos() * half_w;
                let fz = face_angle.sin() * half_d;

                parent.spawn(PbrBundle {
                    mesh: diag_mesh.clone(),
                    material: steel_mat.clone(),
                    transform: Transform::from_xyz(
                        if face == 0 { tower_offset_x } else { fx },
                        level_y + tower_height / brace_levels as f32 * 0.5,
                        if face == 0 { fz } else { 0.0 },
                    ).with_rotation(
                        Quat::from_rotation_y(face_angle) * Quat::from_rotation_z(diag_angle)
                    ),
                    ..default()
                });
                parent.spawn(PbrBundle {
                    mesh: diag_mesh.clone(),
                    material: steel_mat.clone(),
                    transform: Transform::from_xyz(
                        if face == 0 { tower_offset_x } else { fx },
                        level_y + tower_height / brace_levels as f32 * 0.5,
                        if face == 0 { fz } else { 0.0 },
                    ).with_rotation(
                        Quat::from_rotation_y(face_angle) * Quat::from_rotation_z(-diag_angle)
                    ),
                    ..default()
                });
            }
        }

        let deck_levels = [
            (2.5_f32, 1.6_f32),
            (4.2_f32, 1.5_f32),
            (5.8_f32, 1.4_f32),
            (7.5_f32, 1.3_f32),
            (9.2_f32, 1.2_f32),
        ];

        for (deck_y, deck_size) in deck_levels {
            let deck_mesh = meshes.add(Cuboid::new(deck_size, 0.06, deck_size));
            parent.spawn(PbrBundle {
                mesh: deck_mesh,
                material: dark_steel_mat.clone(),
                transform: Transform::from_xyz(tower_offset_x, deck_y, 0.0),
                ..default()
            });
        }

        let top_room_w = 1.6_f32;
        let top_room_h = 1.2_f32;
        let top_room_d = 1.6_f32;
        let top_room_mesh = meshes.add(Cuboid::new(top_room_w, top_room_h, top_room_d));
        parent.spawn(PbrBundle {
            mesh: top_room_mesh,
            material: dark_steel_mat.clone(),
            transform: Transform::from_xyz(tower_offset_x, tower_height + top_room_h * 0.5, 0.0),
            ..default()
        });

        let top_cap_mesh = meshes.add(Cuboid::new(top_room_w + 0.2, 0.15, top_room_d + 0.2));
        parent.spawn(PbrBundle {
            mesh: top_cap_mesh,
            material: steel_mat.clone(),
            transform: Transform::from_xyz(tower_offset_x, tower_height + top_room_h + 0.075, 0.0),
            ..default()
        });

        let lightning_mesh = meshes.add(Cylinder::new(0.04, 0.8));
        parent.spawn(PbrBundle {
            mesh: lightning_mesh,
            material: dark_steel_mat.clone(),
            transform: Transform::from_xyz(tower_offset_x, tower_height + top_room_h + 0.4, 0.0),
            ..default()
        });

        let arm_data: [(f32, f32, f32, &str); 9] = [
            (2.2,  4.75, 0.26, "Arm 1: S-IC intertank"),
            (3.5,  4.75, 0.28, "Arm 2: S-IC forward"),
            (4.8,  4.75, 0.26, "Arm 3: S-II aft"),
            (5.9,  4.75, 0.28, "Arm 4: S-II forward"),
            (7.0,  4.85, 0.24, "Arm 5: S-IVB aft"),
            (8.1,  4.90, 0.22, "Arm 6: S-IVB forward/IU"),
            (9.0,  4.90, 0.20, "Arm 7: CM/SM access"),
            (9.9,  4.90, 0.20, "Arm 8: CM hatch (white room)"),
            (10.6, 4.90, 0.18, "Arm 9: Emergency egress"),
        ];

        for (idx, (arm_y, arm_length, arm_width, _name)) in arm_data.iter().enumerate() {
            let arm_thickness = 0.07_f32;
            let arm_mesh = meshes.add(Cuboid::new(*arm_length, arm_thickness, *arm_width));
            let truss_len = arm_length * 0.45;
            let truss_height = arm_length * 0.18;
            let truss_mesh = meshes.add(Cuboid::new(truss_len, 0.04, arm_width * 0.6));
            let cable_radius = 0.018_f32;
            let cable_mesh = meshes.add(Cylinder::new(cable_radius, arm_length * 0.9));

            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(tower_offset_x + arm_length * 0.5, *arm_y, 0.0),
                    ..default()
                },
                crate::spacecraft::SwingArm {
                    arm_index: idx as u32,
                    retracted: 0.0,
                    tower_offset: tower_offset_x,
                    arm_length: *arm_length,
                    arm_y: *arm_y,
                },
            )).with_children(|arm_parent| {
                arm_parent.spawn(PbrBundle {
                    mesh: arm_mesh,
                    material: steel_mat.clone(),
                    transform: Transform::default(),
                    ..default()
                });

                arm_parent.spawn(PbrBundle {
                    mesh: truss_mesh,
                    material: dark_steel_mat.clone(),
                    transform: Transform::from_xyz(-arm_length * 0.15, -truss_height * 0.5 - 0.04, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.35)),
                    ..default()
                });

                arm_parent.spawn(PbrBundle {
                    mesh: cable_mesh.clone(),
                    material: dark_steel_mat.clone(),
                    transform: Transform::from_xyz(0.0, arm_thickness * 0.5 + cable_radius, arm_width * 0.22)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
                arm_parent.spawn(PbrBundle {
                    mesh: cable_mesh.clone(),
                    material: dark_steel_mat.clone(),
                    transform: Transform::from_xyz(0.0, arm_thickness * 0.5 + cable_radius, -arm_width * 0.22)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            });

            let hinge_w = 0.45_f32;
            let hinge_h = 0.38_f32;
            let hinge_d = arm_width + 0.12;
            let hinge_mesh = meshes.add(Cuboid::new(hinge_w, hinge_h, hinge_d));
            parent.spawn(PbrBundle {
                mesh: hinge_mesh,
                material: dark_steel_mat.clone(),
                transform: Transform::from_xyz(tower_offset_x, *arm_y, 0.0),
                ..default()
            });
        }

        let service_building_w = 1.8_f32;
        let service_building_h = 1.4_f32;
        let service_building_d = 2.2_f32;
        let service_mesh = meshes.add(Cuboid::new(service_building_w, service_building_h, service_building_d));
        parent.spawn(PbrBundle {
            mesh: service_mesh,
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(tower_offset_x - 1.6, service_building_h * 0.5, 0.0),
            ..default()
        });

        for i in 0..3 {
            let shelter_w = 1.0_f32;
            let shelter_h = 0.7_f32;
            let shelter_d = 1.0_f32;
            let shelter_mesh = meshes.add(Cuboid::new(shelter_w, shelter_h, shelter_d));
            let sz = (i as f32 - 1.0) * 3.2;
            parent.spawn(PbrBundle {
                mesh: shelter_mesh,
                material: concrete_mat.clone(),
                transform: Transform::from_xyz(tower_offset_x - 2.4, shelter_h * 0.5, sz),
                ..default()
            });
        }

        let utility_mesh = meshes.add(Cuboid::new(0.7, 0.5, 0.9));
        parent.spawn(PbrBundle {
            mesh: utility_mesh.clone(),
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(3.2, 0.25, 2.5),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: utility_mesh.clone(),
            material: concrete_mat.clone(),
            transform: Transform::from_xyz(3.2, 0.25, -2.5),
            ..default()
        });

        let vab_width = 15.8_f32;
        let vab_height = 16.0_f32;
        let vab_depth = 21.8_f32;
        let vab_mesh = meshes.add(Cuboid::new(vab_width, vab_height, vab_depth));

        let vab_distance = 55.0_f32;
        let vab_x = -vab_distance;
        let vab_z = vab_distance * 0.25;

        parent.spawn(PbrBundle {
            mesh: vab_mesh,
            material: vab_mat.clone(),
            transform: Transform::from_xyz(vab_x, vab_height * 0.5, vab_z),
            ..default()
        });

        let vab_door_mesh = meshes.add(Cuboid::new(0.5, 10.5, 7.5));
        let vab_door_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.52, 0.49),
            metallic: 0.1,
            perceptual_roughness: 0.85,
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: vab_door_mesh,
            material: vab_door_mat,
            transform: Transform::from_xyz(vab_x + vab_width * 0.5 + 0.1, 5.25, vab_z),
            ..default()
        });

        let crawlerway_width = 3.5_f32;
        let crawlerway_thickness = 0.15_f32;
        let crawlerway_length = (vab_x.abs().powi(2) + vab_z.powi(2)).sqrt();
        let crawlerway_mesh = meshes.add(Cuboid::new(crawlerway_width, crawlerway_thickness, crawlerway_length));

        let crawlerway_angle = vab_z.atan2(vab_x);
        let crawlerway_mid_x = vab_x * 0.5;
        let crawlerway_mid_z = vab_z * 0.5;

        parent.spawn(PbrBundle {
            mesh: crawlerway_mesh,
            material: crawlerway_mat.clone(),
            transform: Transform::from_xyz(crawlerway_mid_x, crawlerway_thickness * 0.5, crawlerway_mid_z)
                .with_rotation(Quat::from_rotation_y(crawlerway_angle + std::f32::consts::FRAC_PI_2)),
            ..default()
        });

        let road_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.43, 0.40),
            metallic: 0.0,
            perceptual_roughness: 0.92,
            reflectance: 0.0,
            ..default()
        });

        let road_mesh = meshes.add(Cuboid::new(2.5, 0.08, 18.0));
        parent.spawn(PbrBundle {
            mesh: road_mesh.clone(),
            material: road_mat.clone(),
            transform: Transform::from_xyz(8.0, 0.04, 0.0),
            ..default()
        });

        let light_pole_mesh = meshes.add(Cylinder::new(0.04, 4.5));
        let light_fixture_mesh = meshes.add(Cuboid::new(0.15, 0.06, 0.08));
        let light_pole_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.50, 0.52, 0.55),
            metallic: 0.7,
            perceptual_roughness: 0.35,
            reflectance: 0.0,
            ..default()
        });

        let light_positions = [
            (6.5, 6.0), (6.5, -6.0), (6.5, 12.0), (6.5, -12.0),
            (-6.5, 8.0), (-6.5, -8.0), (10.0, 3.0), (10.0, -3.0),
        ];
        for (lx, lz) in light_positions {
            parent.spawn(PbrBundle {
                mesh: light_pole_mesh.clone(),
                material: light_pole_mat.clone(),
                transform: Transform::from_xyz(lx, 2.25, lz),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: light_fixture_mesh.clone(),
                material: light_pole_mat.clone(),
                transform: Transform::from_xyz(lx - 0.08, 4.5, lz),
                ..default()
            });
        }

        let fence_post_mesh = meshes.add(Cylinder::new(0.025, 1.2));
        let fence_rail_mesh = meshes.add(Cuboid::new(0.04, 0.04, 2.5));
        let fence_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.55, 0.57),
            metallic: 0.6,
            perceptual_roughness: 0.4,
            reflectance: 0.0,
            ..default()
        });

        for i in -6..=6 {
            let fz = i as f32 * 2.5;
            parent.spawn(PbrBundle {
                mesh: fence_post_mesh.clone(),
                material: fence_mat.clone(),
                transform: Transform::from_xyz(14.0, 0.6, fz),
                ..default()
            });
            if i < 6 {
                parent.spawn(PbrBundle {
                    mesh: fence_rail_mesh.clone(),
                    material: fence_mat.clone(),
                    transform: Transform::from_xyz(14.0, 1.0, fz + 1.25),
                    ..default()
                });
                parent.spawn(PbrBundle {
                    mesh: fence_rail_mesh.clone(),
                    material: fence_mat.clone(),
                    transform: Transform::from_xyz(14.0, 0.5, fz + 1.25),
                    ..default()
                });
            }
        }

        let palm_trunk_mesh = meshes.add(Cylinder::new(0.07, 2.2));
        let palm_crown_mesh = meshes.add(Sphere::new(0.35));
        let scrub_mesh = meshes.add(Sphere::new(0.38));
        let palmetto_mesh = meshes.add(Sphere::new(0.22));

        let palm_trunk_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.42, 0.28),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            reflectance: 0.0,
            ..default()
        });
        let palm_frond_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.58, 0.18),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            reflectance: 0.0,
            ..default()
        });
        let scrub_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.48, 0.14),
            metallic: 0.0,
            perceptual_roughness: 0.85,
            reflectance: 0.0,
            ..default()
        });
        let palmetto_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.52, 0.16),
            metallic: 0.0,
            perceptual_roughness: 0.82,
            reflectance: 0.0,
            ..default()
        });

        let pad_exclusion_radius = 6.0_f32;

        let palm_positions: [(f32, f32); 16] = [
            (18.0, 10.0), (22.0, -8.0), (-14.0, 18.0), (-18.0, -14.0),
            (28.0, 4.0), (32.0, -12.0), (-24.0, 8.0), (-28.0, -6.0),
            (16.0, 22.0), (10.0, -26.0), (-30.0, 16.0), (-22.0, -20.0),
            (36.0, 0.0), (34.0, 14.0), (-36.0, -10.0), (-32.0, 10.0),
        ];

        for (px, pz) in palm_positions {
            if px.abs() < pad_exclusion_radius && pz.abs() < pad_exclusion_radius {
                continue;
            }
            let trunk_height = 2.0 + ((px * 0.7 + pz * 0.3).sin() * 0.4).abs();
            parent.spawn(PbrBundle {
                mesh: palm_trunk_mesh.clone(),
                material: palm_trunk_mat.clone(),
                transform: Transform::from_xyz(px, trunk_height * 0.5, pz),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: palm_crown_mesh.clone(),
                material: palm_frond_mat.clone(),
                transform: Transform::from_xyz(px, trunk_height + 0.15, pz)
                    .with_scale(Vec3::new(1.1, 0.5, 1.1)),
                ..default()
            });
        }

        let scrub_positions: [(f32, f32); 40] = [
            (8.0, 5.0), (10.0, -3.0), (-6.0, 10.0), (-8.0, -8.0),
            (14.0, 2.0), (16.0, -8.0), (-12.0, 4.0), (-14.0, -2.0),
            (7.0, 12.0), (4.0, -14.0), (-16.0, 10.0), (-10.0, -12.0),
            (18.0, 0.0), (16.0, 10.0), (-20.0, -4.0), (-18.0, 6.0),
            (9.0, 16.0), (-5.0, 18.0), (12.0, -12.0), (-14.0, 14.0),
            (24.0, -8.0), (22.0, 6.0), (-26.0, 10.0), (-24.0, -6.0),
            (6.0, 20.0), (-4.0, -18.0), (14.0, -16.0), (-16.0, 18.0),
            (26.0, 4.0), (-28.0, -2.0), (24.0, -14.0), (-22.0, 12.0),
            (4.0, 24.0), (-8.0, -22.0), (18.0, 18.0), (-20.0, -16.0),
            (28.0, 10.0), (-30.0, -12.0), (32.0, 0.0), (-32.0, 6.0),
        ];

        for (sx, sz) in scrub_positions {
            if sx.abs() < pad_exclusion_radius && sz.abs() < pad_exclusion_radius {
                continue;
            }
            let scale_y = 0.55 + ((sx * 0.5 + sz * 0.7).sin() * 0.15);
            let scale_xz = 0.9 + ((sx * 0.3 + sz * 0.4).cos() * 0.2);
            parent.spawn(PbrBundle {
                mesh: scrub_mesh.clone(),
                material: scrub_mat.clone(),
                transform: Transform::from_xyz(sx, 0.2, sz)
                    .with_scale(Vec3::new(scale_xz, scale_y, scale_xz)),
                ..default()
            });
        }

        let palmetto_positions: [(f32, f32); 32] = [
            (9.0, 6.0), (11.0, -4.0), (-7.0, 11.0), (-9.0, -9.0),
            (15.0, 3.0), (17.0, -9.0), (-13.0, 5.0), (-15.0, -3.0),
            (8.0, 13.0), (5.0, -15.0), (-17.0, 11.0), (-11.0, -13.0),
            (19.0, 1.0), (17.0, 11.0), (-21.0, -3.0), (-19.0, 7.0),
            (10.0, 17.0), (-6.0, 19.0), (13.0, -13.0), (-15.0, 15.0),
            (25.0, -7.0), (23.0, 7.0), (-27.0, 11.0), (-25.0, -5.0),
            (7.0, 21.0), (-3.0, -19.0), (15.0, -15.0), (-17.0, 19.0),
            (27.0, 5.0), (-29.0, -1.0), (25.0, -13.0), (-23.0, 13.0),
        ];

        for (px, pz) in palmetto_positions {
            if px.abs() < pad_exclusion_radius && pz.abs() < pad_exclusion_radius {
                continue;
            }
            parent.spawn(PbrBundle {
                mesh: palmetto_mesh.clone(),
                material: palmetto_mat.clone(),
                transform: Transform::from_xyz(px, 0.08, pz)
                    .with_scale(Vec3::new(1.3, 0.35, 1.3)),
                ..default()
            });
        }
    });
}

fn create_flat_terrain_mesh(size: f32, subdivisions: u32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let steps = subdivisions + 1;
    let half = size * 0.5;

    for z in 0..=subdivisions {
        for x in 0..=subdivisions {
            let px = (x as f32 / subdivisions as f32) * size - half;
            let pz = (z as f32 / subdivisions as f32) * size - half;
            let dist_from_center = (px * px + pz * pz).sqrt();
            let falloff = (1.0 - (dist_from_center / (half * 0.8)).clamp(0.0, 1.0)).max(0.0);
            let py = ((px * 0.15).sin() * (pz * 0.12).cos() * 0.04
                + (px * 0.08 + pz * 0.1).sin() * 0.03)
                * falloff;

            positions.push([px, py, pz]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([x as f32 / subdivisions as f32, z as f32 / subdivisions as f32]);
        }
    }

    for z in 0..subdivisions {
        for x in 0..subdivisions {
            let a = (z * steps + x) as u32;
            let b = (z * steps + x + 1) as u32;
            let c = ((z + 1) * steps + x) as u32;
            let d = ((z + 1) * steps + x + 1) as u32;

            indices.push(a);
            indices.push(c);
            indices.push(b);
            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn create_octagonal_prism_mesh(radius: f32, height: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let half_h = height * 0.5;
    let sides = 8;

    let top_center_idx = 0;
    positions.push([0.0, half_h, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    let bottom_center_idx = 1;
    positions.push([0.0, -half_h, 0.0]);
    normals.push([0.0, -1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    let rim_start = 2;
    for i in 0..sides {
        let angle = (i as f32 / sides as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        positions.push([x, half_h, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5 + angle.cos() * 0.5, 0.5 + angle.sin() * 0.5]);

        positions.push([x, -half_h, z]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5 + angle.cos() * 0.5, 0.5 + angle.sin() * 0.5]);
    }

    let side_start = positions.len() as u32;
    for i in 0..sides {
        let angle = (i as f32 / sides as f32) * std::f32::consts::TAU;
        let next_angle = ((i + 1) as f32 / sides as f32) * std::f32::consts::TAU;
        let mid_angle = (angle + next_angle) * 0.5;
        let nx = mid_angle.cos();
        let nz = mid_angle.sin();

        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        positions.push([x, half_h, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([i as f32 / sides as f32, 1.0]);

        positions.push([x, -half_h, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([i as f32 / sides as f32, 0.0]);
    }

    for i in 0..sides {
        let a = rim_start + i * 2;
        let b = rim_start + ((i + 1) % sides) * 2;
        indices.push(top_center_idx);
        indices.push(b as u32);
        indices.push(a as u32);
    }

    for i in 0..sides {
        let a = rim_start + i * 2 + 1;
        let b = rim_start + ((i + 1) % sides) * 2 + 1;
        indices.push(bottom_center_idx);
        indices.push(a as u32);
        indices.push(b as u32);
    }

    for i in 0..sides {
        let a = side_start + i * 2;
        let b = side_start + i * 2 + 1;
        let c = side_start + ((i + 1) % sides) * 2 + 1;
        let d = side_start + ((i + 1) % sides) * 2;
        indices.push(a);
        indices.push(b);
        indices.push(c);
        indices.push(a);
        indices.push(c);
        indices.push(d);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

const APOLLO_11_JULIAN_DATE: f64 = 2440423.0638889;

const LUNAR_SEMI_MAJOR_AXIS_KM: f64 = 384400.0;
const LUNAR_ECCENTRICITY: f64 = 0.0549;
const LUNAR_INCLINATION_DEG: f64 = 23.4;
const LUNAR_RAAN_DEG: f64 = 356.3;
const LUNAR_ARG_PERIAPSIS_DEG: f64 = 44.1;
const LUNAR_MEAN_ANOMALY_DEG: f64 = 205.3;
const LUNAR_SIDEREAL_PERIOD_DAYS: f64 = 27.32166;

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
        emissive: LinearRgba::new(0.05, 0.05, 0.06, 0.15),
        ..default()
    });

    let semi_major_axis = (LUNAR_SEMI_MAJOR_AXIS_KM / EARTH_RADIUS_KM) as f64 * crate::world::EARTH_RADIUS as f64;
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
