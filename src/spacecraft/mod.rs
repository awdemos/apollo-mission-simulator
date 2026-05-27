use bevy::prelude::*;

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CmMaterialCache>()
            .init_resource::<CmMeshCache>()
            .add_systems(Startup, init_cm_mesh_cache)
            .add_systems(Startup, spawn_apollo_stack.after(init_cm_mesh_cache))
            .add_systems(Update, update_spacecraft_transform);
    }
}
#[derive(Component)]
pub struct Spacecraft {
    pub vessel_type: VesselType,
    pub velocity: Vec3,
    pub altitude: f32,
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cache: Res<CmMeshCache>,
) {
    let lat = crate::world::LAUNCH_SITE_LAT;
    let lon = crate::world::LAUNCH_SITE_LON;
    let r = crate::world::EARTH_RADIUS;
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.sin();
    let surface_position = Vec3::new(x, y, z);
    let normal = surface_position.normalize();
    let rocket_height = 5.5;
    let rocket_position = surface_position + normal * rocket_height * 0.5;
    let up = Vec3::Y;
    let right = normal.cross(up).normalize();
    let forward = right.cross(normal);
    let rotation_matrix = Mat3::from_cols(right, normal, forward);
    let rotation = Quat::from_mat3(&rotation_matrix);
    spawn_saturn_v(&mut commands, &mut meshes, &mut materials, &cache, rocket_position, rotation);
}
fn spawn_saturn_v(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    cache: &CmMeshCache,
    position: Vec3,
    rotation: Quat,
) {
    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.92),
        metallic: 0.2,
        perceptual_roughness: 0.6,
        ..default()
    });
    let black = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.05, 0.05),
        metallic: 0.3,
        perceptual_roughness: 0.4,
        ..default()
    });
    let silver = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.75, 0.78),
        metallic: 0.7,
        perceptual_roughness: 0.25,
        ..default()
    });
    let rust = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.3, 0.15),
        metallic: 0.4,
        perceptual_roughness: 0.5,
        ..default()
    });
    let gold = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.65, 0.2),
        metallic: 0.9,
        perceptual_roughness: 0.15,
        ..default()
    });
    let s_ic_diameter = 0.5;
    let s_ic_height = 2.1;
    let s_ii_diameter = 0.5;
    let s_ii_height = 1.25;
    let s_ivb_diameter = 0.33;
    let s_ivb_height = 0.9;
    let spacecraft_height = 0.9;
    let s_ic_mesh = meshes.add(Cylinder::new(s_ic_diameter, s_ic_height));
    let s_ii_mesh = meshes.add(Cylinder::new(s_ii_diameter, s_ii_height));
    let s_ivb_mesh = meshes.add(Cylinder::new(s_ivb_diameter, s_ivb_height));
    let interstage_mesh = meshes.add(Cylinder::new(s_ic_diameter, 0.25));
    let cone_mesh = meshes.add(Cone { radius: s_ivb_diameter, height: 0.35 });
    let engine_mesh = meshes.add(Cylinder::new(s_ic_diameter * 1.05, 0.2));
    let nozzle_mesh = meshes.add(Cone { radius: s_ic_diameter * 0.8, height: 0.3 });
    let escape_tower_mesh = meshes.add(Cylinder::new(0.03, 0.6));
    let engine_skirt = meshes.add(Cylinder::new(s_ii_diameter * 1.02, 0.15));
    let ivb_skirt = meshes.add(Cylinder::new(s_ivb_diameter * 1.02, 0.12));
    let saturn_v = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position).with_rotation(rotation),
            ..default()
        },
        Spacecraft {
            vessel_type: VesselType::SaturnV,
            velocity: Vec3::ZERO,
            altitude: 0.0,
        },
        Name::new("Saturn V - Apollo 11 (SA-506)"),
    )).id();
    commands.entity(saturn_v).with_children(|parent| {
        let mut current_y = -2.5;
        parent.spawn(PbrBundle {
            mesh: engine_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, current_y, 0.0),
            ..default()
        });
        current_y += 0.15;
        parent.spawn(PbrBundle {
            mesh: nozzle_mesh.clone(),
            material: black.clone(),
            transform: Transform::from_xyz(0.0, current_y, 0.0),
            ..default()
        });
        current_y += 0.25;
        parent.spawn(PbrBundle {
            mesh: s_ic_mesh,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, current_y + s_ic_height * 0.5, 0.0),
            ..default()
        });
        current_y += s_ic_height;
        parent.spawn(PbrBundle {
            mesh: interstage_mesh.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.125, 0.0),
            ..default()
        });
        current_y += 0.25;
        parent.spawn(PbrBundle {
            mesh: engine_skirt.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.075, 0.0),
            ..default()
        });
        current_y += 0.15;
        parent.spawn(PbrBundle {
            mesh: s_ii_mesh,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, current_y + s_ii_height * 0.5, 0.0),
            ..default()
        });
        current_y += s_ii_height;
        parent.spawn(PbrBundle {
            mesh: ivb_skirt.clone(),
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.06, 0.0),
            ..default()
        });
        current_y += 0.12;
        parent.spawn(PbrBundle {
            mesh: s_ivb_mesh,
            material: silver.clone(),
            transform: Transform::from_xyz(0.0, current_y + s_ivb_height * 0.5, 0.0),
            ..default()
        });
        current_y += s_ivb_height;
        parent.spawn(PbrBundle {
            mesh: interstage_mesh.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.125, 0.0),
            ..default()
        });
        current_y += 0.25;
        parent.spawn(PbrBundle {
            mesh: cone_mesh,
            material: white.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.175, 0.0),
            ..default()
        });
        current_y += 0.35;
        parent.spawn(PbrBundle {
            mesh: escape_tower_mesh.clone(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.3, 0.0),
            ..default()
        });
        let tower_cap = meshes.add(Sphere::new(0.04));
        parent.spawn(PbrBundle {
            mesh: tower_cap,
            material: black.clone(),
            transform: Transform::from_xyz(0.0, current_y + 0.62, 0.0),
            ..default()
        });
    });
    spawn_cm_interior(commands, meshes, materials, cache, saturn_v);
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
        base_color: Color::srgb(0.8, 0.7, 0.3),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });
    let silver = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.75),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });
    let body = meshes.add(Cylinder::new(2.0, 2.5));
    let leg = meshes.add(Cylinder::new(0.1, 3.0));
    let lm = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position),
            ..default()
        },
        Spacecraft {
            vessel_type: VesselType::LunarModule,
            velocity: Vec3::ZERO,
            altitude: 0.0,
        },
        Name::new("Lunar Module"),
    )).id();
    commands.entity(lm).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: body,
            material: gold.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        });
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU + std::f32::consts::FRAC_PI_4;
            let x = angle.cos() * 2.5;
            let z = angle.sin() * 2.5;
            parent.spawn(PbrBundle {
                mesh: leg.clone(),
                material: silver.clone(),
                transform: Transform::from_xyz(x, -0.5, z),
                ..default()
            });
        }
    });
    lm
}
#[derive(Component)]
pub struct CmInterior;
fn spawn_cm_interior(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    cache: &CmMeshCache,
    parent: Entity,
) {
    use crate::config::*;
    
    let material_cache = CmMaterialCache {
        wall: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.62, 0.6, 0.58),
            metallic: 0.05,
            perceptual_roughness: 0.8,
            ..default()
        })),
        panel: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.2),
            metallic: 0.4,
            perceptual_roughness: 0.5,
            ..default()
        })),
        panel_highlight: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.25, 0.28),
            metallic: 0.3,
            perceptual_roughness: 0.6,
            ..default()
        })),
        seat: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.38, 0.42),
            metallic: 0.0,
            perceptual_roughness: 0.95,
            ..default()
        })),
        dsky: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.12, 0.12),
            metallic: 0.6,
            perceptual_roughness: 0.3,
            ..default()
        })),
        fdai: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            metallic: 0.5,
            perceptual_roughness: 0.4,
            ..default()
        })),
        display_glow: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.85, 0.9),
            emissive: Color::srgb(0.4, 0.45, 0.5).into(),
            metallic: 0.0,
            perceptual_roughness: 0.2,
            ..default()
        })),
        window: Some(materials.add(StandardMaterial {
            base_color: Color::srgba(0.5, 0.7, 0.9, 0.25),
            metallic: 0.95,
            perceptual_roughness: 0.05,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        switch: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 0.72),
            metallic: 0.8,
            perceptual_roughness: 0.3,
            ..default()
        })),
        tunnel: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.52),
            metallic: 0.3,
            perceptual_roughness: 0.6,
            ..default()
        })),
        heat_shield: Some(materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.35, 0.25),
            metallic: 0.1,
            perceptual_roughness: 0.9,
            ..default()
        })),
    };
    
    let m = |opt: &Option<Handle<StandardMaterial>>| opt.as_ref().unwrap().clone();
    
    let interior = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, CM_HEIGHT * 0.5, 0.0),
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
        spawn_windows(parent, meshes, m(&material_cache.window));
        spawn_interior_lighting(parent);
        
        crate::panels::spawn_historical_panels(
            parent, meshes, materials,
            CONSOLE_Y, CONSOLE_Z, CONSOLE_WIDTH, CONSOLE_HEIGHT, CONSOLE_DEPTH
        );
        
        crate::panels::spawn_electrical_connectors(parent, meshes, materials);
    });
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
    window_mat: Handle<StandardMaterial>,
) {
    use crate::config::*;
    
    let rendezvous_radius = get_conical_radius_at_height(CM_BASE_RADIUS, CM_TOP_RADIUS, FLOOR_Y, CEILING_Y, RENDEZVOUS_WINDOW_Y);
    for side in [-1.0f32, 1.0] {
        let window_frame = meshes.add(Cuboid::new(RENDEZVOUS_WINDOW_WIDTH, RENDEZVOUS_WINDOW_HEIGHT, 0.04));
        let angle = side * RENDEZVOUS_WINDOW_ANGLE_DEG.to_radians();
        let wx = angle.cos() * rendezvous_radius;
        let wz = angle.sin() * rendezvous_radius - 0.1;
        parent.spawn(PbrBundle {
            mesh: window_frame,
            material: window_mat.clone(),
            transform: Transform::from_xyz(wx, RENDEZVOUS_WINDOW_Y, wz)
                .with_rotation(Quat::from_rotation_y(angle)),
            ..default()
        });
    }

    let side_radius = get_conical_radius_at_height(CM_BASE_RADIUS, CM_TOP_RADIUS, FLOOR_Y, CEILING_Y, SIDE_WINDOW_Y);
    for side in [-1.0f32, 1.0] {
        let window_frame = meshes.add(Cuboid::new(0.04, SIDE_WINDOW_SIZE, SIDE_WINDOW_SIZE));
        let angle = side * std::f32::consts::FRAC_PI_2;
        let wx = angle.cos() * side_radius;
        let wz = angle.sin() * side_radius;
        parent.spawn(PbrBundle {
            mesh: window_frame,
            material: window_mat.clone(),
            transform: Transform::from_xyz(wx, SIDE_WINDOW_Y, wz)
                .with_rotation(Quat::from_rotation_y(angle)),
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
    
    // Generate vertices for bottom and top rings
    for i in 0..=segments {
        let angle = i as f32 * angle_step;
        let cos = angle.cos();
        let sin = angle.sin();
        
        // Bottom ring
        vertices.push([base_radius * cos, bottom_y, base_radius * sin]);
        let bottom_normal = Vec3::new(cos, slope, sin).normalize();
        normals.push([bottom_normal.x, bottom_normal.y, bottom_normal.z]);
        uvs.push([i as f32 / segments as f32, 0.0]);
        
        // Top ring
        vertices.push([top_radius * cos, top_y, top_radius * sin]);
        let top_normal = Vec3::new(cos, slope, sin).normalize();
        normals.push([top_normal.x, top_normal.y, top_normal.z]);
        uvs.push([i as f32 / segments as f32, 1.0]);
    }
    
    // Generate indices for triangles
    for i in 0..segments {
        let base = i * 2;
        // First triangle
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        // Second triangle
        indices.push(base + 1);
        indices.push(base + 3);
        indices.push(base + 2);
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
