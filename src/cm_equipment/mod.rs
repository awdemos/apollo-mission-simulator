use bevy::prelude::*;

pub struct CmEquipmentPlugin;

impl Plugin for CmEquipmentPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct EquipmentBay;

#[derive(Component)]
pub struct LebStructure;

#[derive(Component)]
pub struct AftEquipmentBay;

#[derive(Component)]
pub struct RhebStructure;

#[derive(Component, Debug, Clone)]
pub struct EquipmentModule {
    pub name: String,
    pub drawing_ref: String,
    pub description: String,
}

#[derive(Component, Debug, Clone)]
pub struct StowageLocker {
    pub id: String,
    pub contents: String,
}

#[derive(Component, Debug, Clone)]
pub struct CircuitBreakerPanel {
    pub panel_id: String,
    pub description: String,
}

#[derive(Component, Debug, Clone)]
pub struct BatteryUnit {
    pub id: String,
    pub voltage: f32,
    pub capacity_ah: f32,
    pub battery_type: BatteryType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryType {
    EntryPostlanding,
    Pyrotechnic,
}

#[derive(Component, Debug, Clone)]
pub struct InverterUnit {
    pub id: String,
    pub voltage_out: f32,
    pub frequency_hz: f32,
    pub va_rating: f32,
}

#[derive(Component, Debug, Clone)]
pub struct GseMarker;

#[derive(Component, Debug, Clone)]
pub struct RemoveBeforeFlightTag;

#[derive(Component, Debug, Clone)]
pub struct DrawingReference {
    pub v36_number: String,
}

pub fn spawn_equipment_bays(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    cm_interior: Entity,
) {
    let bay_materials = BayMaterials {
        leb_wall: materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.55, 0.58),
            metallic: 0.0,
            perceptual_roughness: 0.75,
            ..default()
        }),
        equipment_chassis: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.25, 0.28),
            metallic: 0.0,
            perceptual_roughness: 0.6,
            ..default()
        }),
        aluminum_rack: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.72, 0.74),
            metallic: 1.0,
            perceptual_roughness: 0.3,
            ..default()
        }),
        stowage_bag: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.30, 0.25),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        }),
        battery_case: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.20),
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        }),
        wiring_harness: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.15, 0.15),
            metallic: 0.0,
            perceptual_roughness: 0.4,
            ..default()
        }),
        yellow_tag: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.85, 0.15),
            metallic: 0.0,
            perceptual_roughness: 0.7,
            emissive: LinearRgba::new(0.3, 0.25, 0.05, 0.3),
            ..default()
        }),
        label_plate: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.85, 0.85),
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        }),
    };

    commands.entity(cm_interior).with_children(|parent| {
        spawn_leb_structure(parent, meshes, materials, &bay_materials);
        spawn_aft_equipment_bay(parent, meshes, materials, &bay_materials);
        spawn_rheb_structure(parent, meshes, materials, &bay_materials);
        spawn_structural_markings(parent, meshes, materials);
    });
}

pub struct BayMaterials {
    pub leb_wall: Handle<StandardMaterial>,
    pub equipment_chassis: Handle<StandardMaterial>,
    pub aluminum_rack: Handle<StandardMaterial>,
    pub stowage_bag: Handle<StandardMaterial>,
    pub battery_case: Handle<StandardMaterial>,
    pub wiring_harness: Handle<StandardMaterial>,
    pub yellow_tag: Handle<StandardMaterial>,
    pub label_plate: Handle<StandardMaterial>,
}

fn spawn_leb_structure(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mats: &BayMaterials,
) {
    parent.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, crate::config::FLOOR_Y + 0.3, 0.45),
            ..default()
        },
        EquipmentBay,
        LebStructure,
        Name::new("Lower Forward Equipment Bay"),
    )).with_children(|leb_parent| {
        let floor = meshes.add(Cuboid::new(1.2, 0.04, 1.0));
        leb_parent.spawn(PbrBundle {
            mesh: floor,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, -0.3, 0.0),
            ..default()
        });

        let back_wall = meshes.add(Cuboid::new(1.2, 0.8, 0.04));
        leb_parent.spawn(PbrBundle {
            mesh: back_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, 0.1, 0.5),
            ..default()
        });

        let left_wall = meshes.add(Cuboid::new(0.04, 0.8, 1.0));
        leb_parent.spawn(PbrBundle {
            mesh: left_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(-0.6, 0.1, 0.0),
            ..default()
        });

        let right_wall = meshes.add(Cuboid::new(0.04, 0.8, 1.0));
        leb_parent.spawn(PbrBundle {
            mesh: right_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.6, 0.1, 0.0),
            ..default()
        });

        for shelf_y in [0.0, 0.25, 0.5] {
            let shelf = meshes.add(Cuboid::new(1.1, 0.02, 0.9));
            leb_parent.spawn(PbrBundle {
                mesh: shelf,
                material: mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(0.0, shelf_y, 0.0),
                ..default()
            });
        }

        for rail_x in [-0.5, -0.25, 0.0, 0.25, 0.5] {
            let rail = meshes.add(Cylinder::new(0.01, 0.8));
            leb_parent.spawn(PbrBundle {
                mesh: rail,
                material: mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(rail_x, 0.1, -0.4),
                ..default()
            });
        }

        spawn_leb_equipment(leb_parent, meshes, materials, mats);
    });
}

fn spawn_aft_equipment_bay(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mats: &BayMaterials,
) {
    parent.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, crate::config::FLOOR_Y + 0.2, -0.3),
            ..default()
        },
        EquipmentBay,
        AftEquipmentBay,
        Name::new("Lower Aft Equipment Bay"),
    )).with_children(|aft_parent| {
        let floor = meshes.add(Cuboid::new(1.0, 0.04, 0.8));
        aft_parent.spawn(PbrBundle {
            mesh: floor,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, -0.2, 0.0),
            ..default()
        });

        let back_wall = meshes.add(Cuboid::new(1.0, 0.6, 0.04));
        aft_parent.spawn(PbrBundle {
            mesh: back_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, 0.1, -0.4),
            ..default()
        });

        for shelf_z in [-0.2, 0.1] {
            let shelf = meshes.add(Cuboid::new(0.9, 0.02, 0.35));
            aft_parent.spawn(PbrBundle {
                mesh: shelf,
                material: mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(0.0, 0.0, shelf_z),
                ..default()
            });
        }

        spawn_aft_equipment(aft_parent, meshes, materials, mats);
    });
}

fn spawn_rheb_structure(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mats: &BayMaterials,
) {
    parent.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(
                crate::config::CM_BASE_RADIUS * 0.55,
                crate::config::FLOOR_Y + 0.5,
                -0.2,
            ),
            ..default()
        },
        EquipmentBay,
        RhebStructure,
        Name::new("Right-Hand Equipment Bay"),
    )).with_children(|rheb_parent| {
        let floor = meshes.add(Cuboid::new(0.6, 0.04, 1.4));
        rheb_parent.spawn(PbrBundle {
            mesh: floor,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        });

        let outer_wall = meshes.add(Cuboid::new(0.04, 1.2, 1.4));
        rheb_parent.spawn(PbrBundle {
            mesh: outer_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.3, 0.1, 0.0),
            ..default()
        });

        let front_wall = meshes.add(Cuboid::new(0.6, 1.2, 0.04));
        rheb_parent.spawn(PbrBundle {
            mesh: front_wall,
            material: mats.leb_wall.clone(),
            transform: Transform::from_xyz(0.0, 0.1, 0.7),
            ..default()
        });

        for shelf_z in [-0.4, 0.0, 0.4] {
            let shelf = meshes.add(Cuboid::new(0.55, 0.02, 0.5));
            rheb_parent.spawn(PbrBundle {
                mesh: shelf,
                material: mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(-0.02, 0.2, shelf_z),
                ..default()
            });
        }

        let breaker_panel = meshes.add(Cuboid::new(0.5, 0.7, 0.03));
        rheb_parent.spawn((
            PbrBundle {
                mesh: breaker_panel,
                material: mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(-0.05, 0.3, -0.65)
                    .with_rotation(Quat::from_rotation_y(0.15)),
                ..default()
            },
            CircuitBreakerPanel {
                panel_id: "RHEB-229".to_string(),
                description: "Auxiliary Circuit Breaker Panel".to_string(),
            },
            EquipmentModule {
                name: "Panel 229".to_string(),
                drawing_ref: "V36-762023".to_string(),
                description: "Auxiliary Circuit Breaker Panel".to_string(),
            },
        ));

        spawn_rheb_equipment(rheb_parent, meshes, materials, mats);
    });
}

pub fn spawn_leb_equipment(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    bay_mats: &BayMaterials,
) {
    let imu_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.22, 0.25),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    });

    let agc_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        metallic: 0.0,
        perceptual_roughness: 0.4,
        ..default()
    });

    let psa_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.22, 0.25),
        metallic: 0.1,
        perceptual_roughness: 0.5,
        ..default()
    });

    let optics_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.38),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    let battery_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.18, 0.20),
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    let nav_base = meshes.add(Cylinder::new(0.18, 0.06));
        parent.spawn((
            PbrBundle {
                mesh: nav_base,
                material: bay_mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(0.0, 0.55, 0.0),
                ..default()
            },
            EquipmentModule {
                name: "Navigation Base".to_string(),
                drawing_ref: "2899982-041".to_string(),
                description: "IMU Navigation Base with shock isolation hardmounts".to_string(),
            },
        ));

        let imu = meshes.add(Cylinder::new(0.16, 0.22));
        parent.spawn((
            PbrBundle {
                mesh: imu,
                material: imu_material,
                transform: Transform::from_xyz(0.0, 0.72, 0.0),
                ..default()
            },
            EquipmentModule {
                name: "IMU".to_string(),
                drawing_ref: "2018601-201".to_string(),
                description: "Inertial Measurement Unit - Size 12.5 Block II".to_string(),
            },
        ));

        let agc = meshes.add(Cuboid::new(0.25, 0.2, 0.3));
        parent.spawn((
            PbrBundle {
                mesh: agc,
                material: agc_material,
                transform: Transform::from_xyz(0.0, 0.15, 0.1),
                ..default()
            },
            EquipmentModule {
                name: "AGC (CMC)".to_string(),
                drawing_ref: "2003993-031".to_string(),
                description: "Apollo Guidance Computer - Command Module Computer".to_string(),
            },
        ));

        let psa = meshes.add(Cuboid::new(0.4, 0.25, 0.35));
        parent.spawn((
            PbrBundle {
                mesh: psa,
                material: psa_material,
                transform: Transform::from_xyz(0.0, -0.05, 0.05),
                ..default()
            },
            EquipmentModule {
                name: "PSA".to_string(),
                drawing_ref: "2007203-101".to_string(),
                description: "Power and Servo Assembly - 42 modular components".to_string(),
            },
        ));

        let sextant = meshes.add(Cylinder::new(0.06, 0.25));
        parent.spawn((
            PbrBundle {
                mesh: sextant,
                material: optics_material.clone(),
                transform: Transform::from_xyz(-0.15, 0.45, -0.15)
                    .with_rotation(Quat::from_rotation_x(-0.3)),
                ..default()
            },
            EquipmentModule {
                name: "Sextant (SXT)".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Two line-of-sight optical navigation instrument".to_string(),
            },
        ));

        let telescope = meshes.add(Cylinder::new(0.04, 0.2));
        parent.spawn((
            PbrBundle {
                mesh: telescope,
                material: optics_material,
                transform: Transform::from_xyz(0.15, 0.42, -0.1)
                    .with_rotation(Quat::from_rotation_x(-0.2)),
                ..default()
            },
            EquipmentModule {
                name: "Scanning Telescope (SCT)".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Wide field single line-of-sight telescope".to_string(),
            },
        ));

        for (i, (x, z)) in [
            (-0.25, 0.35),
            (-0.15, 0.45),
            (-0.05, 0.35),
            (0.05, 0.45),
            (0.15, 0.35),
        ].iter().enumerate() {
            let cdu = meshes.add(Cuboid::new(0.06, 0.08, 0.06));
            parent.spawn((
                PbrBundle {
                    mesh: cdu,
                    material: bay_mats.equipment_chassis.clone(),
                    transform: Transform::from_xyz(*x, 0.35, *z),
                    ..default()
                },
                EquipmentModule {
                    name: format!("CDU {}", i + 1),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: "Coupling Data Unit - angle data interface".to_string(),
                },
            ));
        }

        let gn_panel = meshes.add(Cuboid::new(0.35, 0.08, 0.02));
        parent.spawn((
            PbrBundle {
                mesh: gn_panel,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.0, 0.55, -0.45),
                ..default()
            },
            EquipmentModule {
                name: "G&N Indicator Control Panel".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "ISS, CGC, PGNS, MASTER ALARM, STAR ACQUIRED".to_string(),
            },
        ));

        for (i, x) in [-0.3, -0.1, 0.1].iter().enumerate() {
            let bat = meshes.add(Cuboid::new(0.12, 0.1, 0.18));
            parent.spawn((
                PbrBundle {
                    mesh: bat,
                    material: battery_mat.clone(),
                    transform: Transform::from_xyz(*x, -0.15, 0.25),
                    ..default()
                },
                BatteryUnit {
                    id: format!("BAT {}", (b'A' + i as u8) as char),
                    voltage: 29.0,
                    capacity_ah: 40.0,
                    battery_type: BatteryType::EntryPostlanding,
                },
                EquipmentModule {
                    name: format!("Entry Battery {}", (b'A' + i as u8) as char),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: "Silver oxide-zinc entry/postlanding battery".to_string(),
                },
            ));
        }

        for (i, x) in [-0.2, 0.0].iter().enumerate() {
            let pyro = meshes.add(Cuboid::new(0.1, 0.08, 0.15));
            parent.spawn((
                PbrBundle {
                    mesh: pyro,
                    material: battery_mat.clone(),
                    transform: Transform::from_xyz(*x, -0.12, 0.0),
                    ..default()
                },
                BatteryUnit {
                    id: format!("PYRO BAT {}", (b'A' + i as u8) as char),
                    voltage: 23.0,
                    capacity_ah: 0.75,
                    battery_type: BatteryType::Pyrotechnic,
                },
                EquipmentModule {
                    name: format!("Pyro Battery {}", (b'A' + i as u8) as char),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: "Dedicated pyrotechnic firing battery".to_string(),
                },
            ));
        }

        let sband = meshes.add(Cuboid::new(0.15, 0.12, 0.2));
        parent.spawn((
            PbrBundle {
                mesh: sband,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.35, 0.25, 0.1),
                ..default()
            },
            EquipmentModule {
                name: "S-Band Power Amplifier".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Communications S-band power amplifier".to_string(),
            },
        ));

        let audio = meshes.add(Cuboid::new(0.12, 0.08, 0.15));
        parent.spawn((
            PbrBundle {
                mesh: audio,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(-0.35, 0.2, 0.0),
                ..default()
            },
            EquipmentModule {
                name: "Audio Center".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Crew audio communications interface".to_string(),
            },
        ));

        let panel_101 = meshes.add(Cuboid::new(0.2, 0.15, 0.03));
        parent.spawn((
            PbrBundle {
                mesh: panel_101,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(-0.4, 0.5, -0.2)
                    .with_rotation(Quat::from_rotation_y(0.3)),
                ..default()
            },
            EquipmentModule {
                name: "Panel 101".to_string(),
                drawing_ref: "V36-764111".to_string(),
                description: "System Test Meter - Auxiliary Test Panel".to_string(),
            },
        ));

        let rhc = meshes.add(Cylinder::new(0.025, 0.12));
        parent.spawn((
            PbrBundle {
                mesh: rhc,
                material: bay_mats.aluminum_rack.clone(),
                transform: Transform::from_xyz(-0.55, 0.3, -0.1)
                    .with_rotation(Quat::from_rotation_z(0.4)),
                ..default()
            },
            EquipmentModule {
                name: "Rotation Hand Controller".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "LEB-mounted attitude control handle".to_string(),
            },
        ));
}

pub fn spawn_aft_equipment(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    bay_mats: &BayMaterials,
) {
    let inverter_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.32),
        metallic: 0.2,
        perceptual_roughness: 0.5,
        ..default()
    });

    let charger_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.25, 0.28),
        metallic: 0.1,
        perceptual_roughness: 0.5,
        ..default()
    });

    for (i, x) in [-0.2, 0.0, 0.2].iter().enumerate() {
            let inv = meshes.add(Cuboid::new(0.15, 0.12, 0.25));
            parent.spawn((
                PbrBundle {
                    mesh: inv,
                    material: inverter_mat.clone(),
                    transform: Transform::from_xyz(*x, 0.1, -0.1),
                    ..default()
                },
                InverterUnit {
                    id: format!("INV {}", i + 1),
                    voltage_out: 115.0,
                    frequency_hz: 400.0,
                    va_rating: 1250.0,
                },
                EquipmentModule {
                    name: format!("Inverter No. {}", i + 1),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: "DC-to-AC solid-state inverter".to_string(),
                },
            ));
        }

        let charger = meshes.add(Cuboid::new(0.2, 0.1, 0.15));
        parent.spawn((
            PbrBundle {
                mesh: charger,
                material: charger_mat,
                transform: Transform::from_xyz(0.0, -0.05, 0.15),
                ..default()
            },
            EquipmentModule {
                name: "Battery Charger".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Battery charging unit".to_string(),
            },
        ));

        for (i, (start, end)) in [
            (Vec3::new(-0.3, 0.0, -0.3), Vec3::new(0.3, 0.2, 0.2)),
            (Vec3::new(-0.25, -0.1, -0.25), Vec3::new(0.25, 0.1, 0.15)),
        ].iter().enumerate() {
            let harness = meshes.add(Cylinder::new(0.008, start.distance(*end)));
            let mid = (*start + *end) / 2.0;
            let dir = (*end - *start).normalize();
            let rot = Quat::from_rotation_arc(Vec3::Y, dir);
            parent.spawn((
                PbrBundle {
                    mesh: harness,
                    material: bay_mats.wiring_harness.clone(),
                    transform: Transform::from_translation(mid).with_rotation(rot),
                    ..default()
                },
                EquipmentModule {
                    name: format!("Wiring Harness {}", i + 1),
                    drawing_ref: if i == 0 {
                        "V36-440088".to_string()
                    } else {
                        "V36-444022".to_string()
                    },
                    description: "Electrical distribution harness".to_string(),
                },
            ));
        }
}

pub fn spawn_rheb_equipment(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    bay_mats: &BayMaterials,
) {
    let panel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.32, 0.32, 0.35),
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    let docking_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.28, 0.30),
        metallic: 0.2,
        perceptual_roughness: 0.5,
        ..default()
    });

    let rheb_panels = [
            ("RHEB-225", "Telecommunications Circuit Breakers", -0.15, 0.35, -0.5),
            ("RHEB-226", "Cryogenics, Fuel Cell Purge, Lighting", -0.15, 0.15, -0.5),
            ("RHEB-250", "Battery Power Controls", 0.05, 0.35, -0.5),
            ("RHEB-275", "Main Bus Tie, Inverter Power", 0.05, 0.15, -0.5),
        ];

        for (panel_id, desc, x, y, z) in rheb_panels.iter() {
            let panel = meshes.add(Cuboid::new(0.2, 0.15, 0.02));
            parent.spawn((
                PbrBundle {
                    mesh: panel,
                    material: panel_mat.clone(),
                    transform: Transform::from_xyz(*x, *y, *z),
                    ..default()
                },
                CircuitBreakerPanel {
                    panel_id: panel_id.to_string(),
                    description: desc.to_string(),
                },
                EquipmentModule {
                    name: panel_id.to_string(),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: desc.to_string(),
                },
            ));
        }

        let waste_panel = meshes.add(Cuboid::new(0.25, 0.2, 0.03));
        parent.spawn((
            PbrBundle {
                mesh: waste_panel,
                material: panel_mat.clone(),
                transform: Transform::from_xyz(-0.1, -0.1, 0.5),
                ..default()
            },
            EquipmentModule {
                name: "Waste Management System".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Urine and chemical waste management controls".to_string(),
            },
        ));

        for (i, z) in [-0.3, 0.0, 0.3].iter().enumerate() {
            let locker = meshes.add(Cuboid::new(0.4, 0.25, 0.2));
            parent.spawn((
                PbrBundle {
                    mesh: locker,
                    material: bay_mats.stowage_bag.clone(),
                    transform: Transform::from_xyz(-0.05, -0.2, *z),
                    ..default()
                },
                StowageLocker {
                    id: format!("R{}", [2, 3, 5][i]),
                    contents: "Crew supplies and equipment".to_string(),
                },
                EquipmentModule {
                    name: format!("Stowage Locker R{}", [2, 3, 5][i]),
                    drawing_ref: "Zone 105H".to_string(),
                    description: "Crew stowage compartment".to_string(),
                },
            ));
        }

        for (i, x) in [-0.15, 0.05].iter().enumerate() {
            let docking = meshes.add(Cuboid::new(0.12, 0.1, 0.15));
            parent.spawn((
                PbrBundle {
                    mesh: docking,
                    material: docking_mat.clone(),
                    transform: Transform::from_xyz(*x, 0.05, -0.2),
                    ..default()
                },
                EquipmentModule {
                    name: format!("Lunar Docking Events Controller {}", ['A', 'B'][i]),
                    drawing_ref: "V36-XXXXXX".to_string(),
                    description: "CSM-LM docking sequence control unit".to_string(),
                },
            ));
        }

        let pyro_box = meshes.add(Cuboid::new(0.1, 0.08, 0.12));
        parent.spawn((
            PbrBundle {
                mesh: pyro_box,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.15, -0.05, -0.3),
                ..default()
            },
            EquipmentModule {
                name: "Pyro Continuity Verification Box".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Pyrotechnic circuit testing unit".to_string(),
            },
        ));

        let uprighting = meshes.add(Cuboid::new(0.15, 0.12, 0.08));
        parent.spawn((
            PbrBundle {
                mesh: uprighting,
                material: panel_mat.clone(),
                transform: Transform::from_xyz(-0.2, 0.0, -0.35),
                ..default()
            },
            EquipmentModule {
                name: "Panel 278 Uprighting System".to_string(),
                drawing_ref: "V36-762278-11".to_string(),
                description: "Post-landing flotation uprighting control".to_string(),
            },
        ));

        let els = meshes.add(Cuboid::new(0.12, 0.08, 0.1));
        parent.spawn((
            PbrBundle {
                mesh: els,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.2, 0.0, -0.35),
                ..default()
            },
            EquipmentModule {
                name: "Earth Landing System Controls".to_string(),
                drawing_ref: "V36-XXXXXX".to_string(),
                description: "Parachute deployment and recovery control".to_string(),
            },
        ));

        let current_limiter = meshes.add(Cuboid::new(0.08, 0.06, 0.08));
        parent.spawn((
            PbrBundle {
                mesh: current_limiter,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.0, -0.05, -0.4),
                ..default()
            },
            EquipmentModule {
                name: "Current Limiter Operational".to_string(),
                drawing_ref: "V36-752121".to_string(),
                description: "Electrical current protection device".to_string(),
            },
        ));

        let instrumentation = meshes.add(Cuboid::new(0.2, 0.15, 0.2));
        parent.spawn((
            PbrBundle {
                mesh: instrumentation,
                material: bay_mats.equipment_chassis.clone(),
                transform: Transform::from_xyz(0.1, 0.25, 0.2),
                ..default()
            },
            EquipmentModule {
                name: "Instrumentation Complete".to_string(),
                drawing_ref: "V36-750906".to_string(),
                description: "Telemetry and data recording package".to_string(),
            },
        ));
}

pub fn spawn_structural_markings(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let tag_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.85, 0.15),
        metallic: 0.0,
        perceptual_roughness: 0.7,
        emissive: LinearRgba::new(0.2, 0.18, 0.03, 0.2),
        ..default()
    });

    let label_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.85),
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    let tag_positions = [
            (0.5, 1.2, 0.3, "REMOVE BEFORE FLIGHT"),
            (-0.5, 1.5, -0.2, "GROUND SUPPORT EQUIPMENT"),
            (0.3, 0.8, 0.5, "REMOVE BEFORE FLIGHT"),
        ];

        for (x, y, z, text) in tag_positions.iter() {
            let tag = meshes.add(Cuboid::new(0.15, 0.04, 0.01));
            parent.spawn((
                PbrBundle {
                    mesh: tag,
                    material: tag_mat.clone(),
                    transform: Transform::from_xyz(*x, *y, *z),
                    ..default()
                },
                RemoveBeforeFlightTag,
                EquipmentModule {
                    name: text.to_string(),
                    drawing_ref: String::new(),
                    description: text.to_string(),
                },
            ));
        }

        let label_positions = [
            (0.0, 0.05, 0.55, "V36-334501"),
            (0.6, 0.5, -0.1, "V36-440088"),
            (-0.6, 0.3, 0.0, "V36-444022"),
            (0.4, 0.2, 0.4, "W 178 SH 9"),
        ];

        for (x, y, z, label) in label_positions.iter() {
            let plate = meshes.add(Cuboid::new(0.12, 0.03, 0.005));
            parent.spawn((
                PbrBundle {
                    mesh: plate,
                    material: label_mat.clone(),
                    transform: Transform::from_xyz(*x, *y, *z),
                    ..default()
                },
                DrawingReference {
                    v36_number: label.to_string(),
                },
                EquipmentModule {
                    name: label.to_string(),
                    drawing_ref: label.to_string(),
                    description: "Component identification tag".to_string(),
                },
            ));
        }
}
