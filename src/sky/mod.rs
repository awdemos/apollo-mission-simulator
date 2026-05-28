use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_starfield)
            .add_systems(Startup, spawn_ground_sky)
            .add_systems(Startup, spawn_sun)
            .add_systems(Update, update_billboards.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, toggle_sky_visibility.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, update_sun_position.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Component)]
pub struct StarfieldSphere;

#[derive(Component)]
pub struct GroundSky;

#[derive(Component)]
pub struct SunLight;

#[derive(Component)]
pub struct StarBillboard {
    pub base_size: f32,
}

#[derive(Component)]
pub struct PlanetBillboard {
    pub name: String,
}

pub struct StarData {
    pub name: &'static str,
    pub ra: f32,
    pub dec: f32,
    pub magnitude: f32,
    pub color: [f32; 3],
}

pub struct PlanetData {
    pub name: &'static str,
    pub ra: f32,
    pub dec: f32,
    pub magnitude: f32,
    pub color: [f32; 3],
}

pub const PLANETS: &[PlanetData] = &[
    PlanetData { name: "Venus", ra: 8.33, dec: 20.0, magnitude: -4.0, color: [1.0, 0.95, 0.8] },
    PlanetData { name: "Mars", ra: 19.0, dec: -25.0, magnitude: 1.5, color: [1.0, 0.4, 0.2] },
    PlanetData { name: "Jupiter", ra: 14.83, dec: -16.0, magnitude: -2.0, color: [0.9, 0.8, 0.6] },
    PlanetData { name: "Saturn", ra: 3.83, dec: 18.0, magnitude: 0.5, color: [0.9, 0.85, 0.7] },
];

pub const BRIGHT_STARS: &[StarData] = &[
    StarData { name: "Sirius", ra: 6.7525, dec: -16.7161, magnitude: -1.46, color: [0.85, 0.95, 1.0] },
    StarData { name: "Canopus", ra: 6.3992, dec: -52.6956, magnitude: -0.74, color: [0.95, 0.95, 0.8] },
    StarData { name: "Arcturus", ra: 14.2611, dec: 19.1822, magnitude: -0.05, color: [1.0, 0.85, 0.6] },
    StarData { name: "Alpha Centauri", ra: 14.6608, dec: -60.8333, magnitude: -0.01, color: [1.0, 0.95, 0.8] },
    StarData { name: "Vega", ra: 18.6156, dec: 38.7833, magnitude: 0.03, color: [0.9, 0.95, 1.0] },
    StarData { name: "Capella", ra: 5.2781, dec: 45.9983, magnitude: 0.08, color: [1.0, 0.9, 0.7] },
    StarData { name: "Rigel", ra: 5.2422, dec: -8.2017, magnitude: 0.13, color: [0.8, 0.9, 1.0] },
    StarData { name: "Procyon", ra: 7.6553, dec: 5.2250, magnitude: 0.38, color: [1.0, 0.95, 0.85] },
    StarData { name: "Betelgeuse", ra: 5.9192, dec: 7.4071, magnitude: 0.50, color: [1.0, 0.6, 0.3] },
    StarData { name: "Achernar", ra: 1.6286, dec: -57.2367, magnitude: 0.46, color: [0.85, 0.95, 1.0] },
    StarData { name: "Hadar", ra: 14.0636, dec: -60.3731, magnitude: 0.61, color: [0.9, 0.95, 1.0] },
    StarData { name: "Altair", ra: 19.8464, dec: 8.8683, magnitude: 0.77, color: [1.0, 0.95, 0.85] },
    StarData { name: "Acrux", ra: 12.4433, dec: -63.0589, magnitude: 0.77, color: [0.9, 0.95, 1.0] },
    StarData { name: "Aldebaran", ra: 4.5986, dec: 16.5092, magnitude: 0.85, color: [1.0, 0.75, 0.5] },
    StarData { name: "Antares", ra: 16.4901, dec: -26.4320, magnitude: 0.96, color: [1.0, 0.5, 0.3] },
    StarData { name: "Spica", ra: 13.4199, dec: -11.1614, magnitude: 0.98, color: [0.9, 0.95, 1.0] },
    StarData { name: "Pollux", ra: 7.7553, dec: 28.0261, magnitude: 1.14, color: [1.0, 0.85, 0.6] },
    StarData { name: "Fomalhaut", ra: 22.9608, dec: -29.6222, magnitude: 1.16, color: [1.0, 0.95, 0.85] },
    StarData { name: "Deneb", ra: 20.6905, dec: 45.2803, magnitude: 1.25, color: [0.9, 0.95, 1.0] },
    StarData { name: "Mimosa", ra: 12.7953, dec: -59.6886, magnitude: 1.25, color: [0.9, 0.95, 1.0] },
    StarData { name: "Regulus", ra: 10.1392, dec: 11.9672, magnitude: 1.36, color: [1.0, 0.95, 0.85] },
    StarData { name: "Adhara", ra: 6.9772, dec: -28.9721, magnitude: 1.50, color: [0.85, 0.95, 1.0] },
    StarData { name: "Castor", ra: 7.5767, dec: 31.8886, magnitude: 1.58, color: [1.0, 0.95, 0.9] },
    StarData { name: "Gacrux", ra: 12.4433, dec: -57.1133, magnitude: 1.64, color: [1.0, 0.6, 0.4] },
    StarData { name: "Bellatrix", ra: 5.4186, dec: 6.3497, magnitude: 1.64, color: [0.9, 0.95, 1.0] },
    StarData { name: "Elnath", ra: 5.4386, dec: 28.6075, magnitude: 1.65, color: [0.9, 0.95, 1.0] },
    StarData { name: "Miaplacidus", ra: 9.2203, dec: -69.7172, magnitude: 1.68, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alnilam", ra: 5.6036, dec: -1.2017, magnitude: 1.69, color: [0.8, 0.9, 1.0] },
    StarData { name: "Alnair", ra: 22.1372, dec: -46.9608, magnitude: 1.74, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alnitak", ra: 5.6792, dec: -1.9425, magnitude: 1.74, color: [0.8, 0.85, 1.0] },
    StarData { name: "Dubhe", ra: 11.0622, dec: 61.7508, magnitude: 1.79, color: [1.0, 0.85, 0.6] },
    StarData { name: "Mirfak", ra: 3.4053, dec: 49.8612, magnitude: 1.81, color: [1.0, 0.95, 0.85] },
    StarData { name: "Wezen", ra: 6.9772, dec: -28.2306, magnitude: 1.83, color: [0.85, 0.95, 1.0] },
    StarData { name: "Sargas", ra: 17.5626, dec: -42.9979, magnitude: 1.84, color: [1.0, 0.9, 0.6] },
    StarData { name: "Kaus Australis", ra: 18.4020, dec: -34.3846, magnitude: 1.85, color: [1.0, 0.9, 0.7] },
    StarData { name: "Avior", ra: 8.3753, dec: -59.5092, magnitude: 1.86, color: [1.0, 0.8, 0.5] },
    StarData { name: "Menkalinan", ra: 5.9878, dec: 44.9474, magnitude: 1.90, color: [1.0, 0.95, 0.9] },
    StarData { name: "Atria", ra: 16.8111, dec: -69.0277, magnitude: 1.91, color: [1.0, 0.75, 0.5] },
    StarData { name: "Rigel Kentaurus", ra: 14.6608, dec: -60.8333, magnitude: -0.01, color: [1.0, 0.95, 0.8] },
];

fn celestial_to_cartesian(ra_hours: f32, dec_degrees: f32, radius: f32) -> Vec3 {
    let ra_rad = ra_hours * 15.0f32.to_radians();
    let dec_rad = dec_degrees.to_radians();

    let x = radius * dec_rad.cos() * ra_rad.cos();
    let y = radius * dec_rad.sin();
    let z = -radius * dec_rad.cos() * ra_rad.sin();

    Vec3::new(x, y, z)
}

fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sky_radius = 5000.0;
    let star_mesh = meshes.add(Sphere::new(1.0).mesh().ico(4).unwrap());

    for star in BRIGHT_STARS {
        let brightness = 10.0f32.powf(-0.4 * star.magnitude);
        let size = (brightness * 15.0).max(2.0).min(40.0);
        let pos = celestial_to_cartesian(star.ra, star.dec, sky_radius * 0.95);

        let emissive = brightness * 50.0;
        let star_material = materials.add(StandardMaterial {
            base_color: Color::srgb(star.color[0], star.color[1], star.color[2]),
            emissive: LinearRgba::new(
                star.color[0] * emissive,
                star.color[1] * emissive,
                star.color[2] * emissive,
                1.0,
            ),
            unlit: true,
            cull_mode: None,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(size)),
                ..default()
            },
            StarBillboard { base_size: size },
            StarfieldSphere,
            NotShadowCaster,
        ));
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..500 {
        let ra = rng.gen::<f32>() * 24.0;
        let dec = (rng.gen::<f32>() - 0.5) * 180.0;
        let magnitude = 3.5 + rng.gen::<f32>() * 3.0;
        let brightness = 10.0f32.powf(-0.4 * magnitude);
        let size = (brightness * 10.0).max(1.5).min(25.0);
        let pos = celestial_to_cartesian(ra, dec, sky_radius * 0.95);

        let t = rng.gen::<f32>();
        let color = if t < 0.3 {
            [0.85, 0.95, 1.0]
        } else if t < 0.6 {
            [1.0, 0.95, 0.85]
        } else if t < 0.8 {
            [1.0, 0.8, 0.6]
        } else {
            [1.0, 0.6, 0.4]
        };

        let emissive = brightness * 30.0;
        let star_material = materials.add(StandardMaterial {
            base_color: Color::srgb(color[0], color[1], color[2]),
            emissive: LinearRgba::new(
                color[0] * emissive,
                color[1] * emissive,
                color[2] * emissive,
                1.0,
            ),
            unlit: true,
            cull_mode: None,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(size)),
                ..default()
            },
            StarBillboard { base_size: size },
            StarfieldSphere,
            NotShadowCaster,
        ));
    }

    for planet in PLANETS {
        let pos = celestial_to_cartesian(planet.ra, planet.dec, sky_radius * 0.99);
        let brightness = 10.0f32.powf(-0.4 * planet.magnitude);
        let size = (2.0 + brightness * 0.8).min(12.0);

        let planet_mesh = meshes.add(Sphere::new(1.0).mesh().ico(4).unwrap());
        let planet_material = materials.add(StandardMaterial {
            base_color: Color::srgb(planet.color[0], planet.color[1], planet.color[2]),
            emissive: LinearRgba::new(
                planet.color[0] * brightness * 50.0,
                planet.color[1] * brightness * 50.0,
                planet.color[2] * brightness * 50.0,
                1.0,
            ),
            unlit: true,
            cull_mode: None,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: planet_mesh,
                material: planet_material,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(size)),
                ..default()
            },
            PlanetBillboard {
                name: planet.name.to_string(),
            },
            StarfieldSphere,
            NotShadowCaster,
        ));
    }
}

fn spawn_ground_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dome_mesh = meshes.add(create_overcast_sky_dome_mesh(500.0));
    let sky_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: dome_mesh,
            material: sky_material,
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        GroundSky,
        NotShadowCaster,
        NotShadowReceiver,
        Name::new("Ground Sky"),
    ));
}

fn create_overcast_sky_dome_mesh(radius: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    let rings = 24;
    let sectors = 48;

    let zenith_blue = [0.20_f32, 0.45, 0.75];
    let mid_blue = [0.40_f32, 0.60, 0.85];
    let horizon_blue = [0.60_f32, 0.72, 0.90];
    let ground_haze = [0.62_f32, 0.70, 0.85];

    let max_theta = std::f32::consts::FRAC_PI_2 + 0.20;

    positions.push([0.0, radius, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    colors.push(zenith_blue);

    for ring in 1..=rings {
        let t = ring as f32 / rings as f32;
        let theta = t * max_theta;
        let y = theta.cos() * radius;
        let ring_radius = theta.sin() * radius;

        let (r, g, b) = if t < 0.5 {
            let lt = t / 0.5;
            (
                zenith_blue[0] + (mid_blue[0] - zenith_blue[0]) * lt,
                zenith_blue[1] + (mid_blue[1] - zenith_blue[1]) * lt,
                zenith_blue[2] + (mid_blue[2] - zenith_blue[2]) * lt,
            )
        } else if t < 0.80 {
            let lt = (t - 0.5) / 0.30;
            (
                mid_blue[0] + (horizon_blue[0] - mid_blue[0]) * lt,
                mid_blue[1] + (horizon_blue[1] - mid_blue[1]) * lt,
                mid_blue[2] + (horizon_blue[2] - mid_blue[2]) * lt,
            )
        } else {
            let lt = (t - 0.80) / 0.20;
            (
                horizon_blue[0] + (ground_haze[0] - horizon_blue[0]) * lt,
                horizon_blue[1] + (ground_haze[1] - horizon_blue[1]) * lt,
                horizon_blue[2] + (ground_haze[2] - horizon_blue[2]) * lt,
            )
        };

        for sector in 0..sectors {
            let phi = (sector as f32 / sectors as f32) * std::f32::consts::TAU;
            let x = phi.cos() * ring_radius;
            let z = phi.sin() * ring_radius;

            let nx = x / radius;
            let ny = y / radius;
            let nz = z / radius;

            positions.push([x, y, z]);
            normals.push([nx, ny, nz]);
            colors.push([r, g, b]);
        }
    }

    for sector in 0..sectors {
        let a = 1 + sector;
        let b = 1 + (sector + 1) % sectors;
        indices.push(0);
        indices.push(a as u32);
        indices.push(b as u32);
    }

    for ring in 1..rings {
        for sector in 0..sectors {
            let a = 1 + (ring - 1) * sectors + sector;
            let b = 1 + (ring - 1) * sectors + (sector + 1) % sectors;
            let c = 1 + ring * sectors + sector;
            let d = 1 + ring * sectors + (sector + 1) % sectors;

            indices.push(a as u32);
            indices.push(c as u32);
            indices.push(b as u32);
            indices.push(b as u32);
            indices.push(c as u32);
            indices.push(d as u32);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.iter().map(|c| [c[0], c[1], c[2], 1.0f32]).collect::<Vec<_>>());
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_mesh = meshes.add(Sphere::new(1.0).mesh().ico(8).unwrap());
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.88),
        emissive: LinearRgba::new(0.8, 0.8, 0.75, 1.0),
        unlit: true,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: sun_mesh,
            material: sun_material,
            transform: Transform::from_translation(sun_direction() * 4000.0).with_scale(Vec3::splat(60.0)),
            ..default()
        },
        StarfieldSphere,
        NotShadowCaster,
        Name::new("Sun"),
    ));


}

pub fn sun_direction() -> Vec3 {
    // KSC, July 16, 1969, 9:32 AM EDT (13:32 UTC)
    // Solar azimuth ~105 deg (ESE), elevation ~50 deg
    let azimuth = 105.0_f32.to_radians();
    let elevation = 50.0_f32.to_radians();

    let x = elevation.cos() * azimuth.sin();
    let y = elevation.sin();
    let z = elevation.cos() * azimuth.cos();

    Vec3::new(x, y, z).normalize()
}

fn update_sun_position(
    mut query: Query<&mut Transform, With<SunLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_billboards(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut star_query: Query<&mut Transform, (With<StarBillboard>, Without<Camera3d>)>,
    mut planet_query: Query<&mut Transform, (With<PlanetBillboard>, Without<Camera3d>, Without<StarBillboard>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation;

        for mut star_transform in star_query.iter_mut() {
            star_transform.look_at(camera_pos, Vec3::Y);
        }

        for mut planet_transform in planet_query.iter_mut() {
            planet_transform.look_at(camera_pos, Vec3::Y);
        }
    }
}

fn toggle_sky_visibility(
    camera_query: Query<&Transform, With<Camera3d>>,
    launch_site: Res<crate::world::LaunchSitePosition>,
    mut starfield_query: Query<&mut Visibility, With<StarfieldSphere>>,
    mut ground_sky_query: Query<&mut Visibility, (With<GroundSky>, Without<StarfieldSphere>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let dist_from_site = (camera_transform.translation - launch_site.0).length();
        let show_ground = dist_from_site < 200.0;

        for mut visibility in starfield_query.iter_mut() {
            *visibility = if show_ground { Visibility::Hidden } else { Visibility::Visible };
        }

        for mut visibility in ground_sky_query.iter_mut() {
            *visibility = if show_ground { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}
