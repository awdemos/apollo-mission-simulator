use bevy::prelude::*;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sky_dome)
            .add_systems(Update, update_sky_dome_rotation);
    }
}

#[derive(Component)]
pub struct SkyDome;

#[derive(Component)]
pub struct Star {
    pub ra: f32,
    pub dec: f32,
    pub magnitude: f32,
}

pub struct StarData {
    pub name: &'static str,
    pub ra: f32,
    pub dec: f32,
    pub magnitude: f32,
    pub color: [f32; 3],
}

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
    StarData { name: "Alnilam", ra: 5.6033, dec: -1.2017, magnitude: 1.69, color: [0.85, 0.95, 1.0] },
    StarData { name: "Alnair", ra: 22.1372, dec: -46.9608, magnitude: 1.74, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alnitak", ra: 5.6792, dec: -1.9426, magnitude: 1.74, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alioth", ra: 12.9006, dec: 55.9598, magnitude: 1.77, color: [1.0, 0.95, 0.9] },
    StarData { name: "Dubhe", ra: 11.0622, dec: 61.7511, magnitude: 1.79, color: [1.0, 0.85, 0.6] },
    StarData { name: "Mirfak", ra: 3.4053, dec: 49.8611, magnitude: 1.79, color: [1.0, 0.9, 0.75] },
    StarData { name: "Wezen", ra: 7.1397, dec: -26.3933, magnitude: 1.83, color: [1.0, 0.9, 0.75] },
    StarData { name: "Sargas", ra: 17.5606, dec: -42.9978, magnitude: 1.84, color: [1.0, 0.85, 0.5] },
    StarData { name: "Kaus Australis", ra: 18.4020, dec: -34.3847, magnitude: 1.85, color: [1.0, 0.95, 0.85] },
    StarData { name: "Avior", ra: 8.3753, dec: -59.5092, magnitude: 1.86, color: [1.0, 0.8, 0.5] },
    StarData { name: "Alkaid", ra: 13.7922, dec: 49.3133, magnitude: 1.86, color: [0.9, 0.95, 1.0] },
    StarData { name: "Menkalinan", ra: 5.9939, dec: 44.9475, magnitude: 1.90, color: [1.0, 0.95, 0.9] },
    StarData { name: "Atria", ra: 16.8111, dec: -69.0278, magnitude: 1.91, color: [1.0, 0.75, 0.5] },
    StarData { name: "Alhena", ra: 6.6283, dec: 16.3992, magnitude: 1.93, color: [1.0, 0.95, 0.9] },
    StarData { name: "Peacock", ra: 20.4275, dec: -56.7350, magnitude: 1.94, color: [0.9, 0.95, 1.0] },
    StarData { name: "Polaris", ra: 2.5303, dec: 89.2641, magnitude: 1.98, color: [1.0, 0.95, 0.9] },
    StarData { name: "Mirzam", ra: 6.3783, dec: -17.9558, magnitude: 1.98, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alphard", ra: 9.4598, dec: -8.6583, magnitude: 1.99, color: [1.0, 0.7, 0.45] },
    StarData { name: "Hamal", ra: 2.1194, dec: 23.4625, magnitude: 2.01, color: [1.0, 0.8, 0.6] },
    StarData { name: "Algieba", ra: 10.3328, dec: 19.8417, magnitude: 2.01, color: [1.0, 0.75, 0.45] },
    StarData { name: "Diphda", ra: 0.7261, dec: -17.9864, magnitude: 2.04, color: [1.0, 0.85, 0.6] },
    StarData { name: "Mizar", ra: 13.3987, dec: 54.9253, magnitude: 2.04, color: [1.0, 0.95, 0.9] },
    StarData { name: "Nunki", ra: 18.9218, dec: -26.2967, magnitude: 2.05, color: [0.9, 0.95, 1.0] },
    StarData { name: "Menkent", ra: 14.0606, dec: -36.3697, magnitude: 2.06, color: [1.0, 0.85, 0.55] },
    StarData { name: "Alpheratz", ra: 0.1397, dec: 29.0908, magnitude: 2.07, color: [0.9, 0.95, 1.0] },
    StarData { name: "Rigel Kentaurus B", ra: 14.6606, dec: -60.8358, magnitude: 1.35, color: [1.0, 0.9, 0.8] },
    StarData { name: "Saiph", ra: 5.7958, dec: -9.6696, magnitude: 2.07, color: [0.85, 0.95, 1.0] },
    StarData { name: "Kochab", ra: 14.8451, dec: 74.1555, magnitude: 2.08, color: [1.0, 0.8, 0.5] },
    StarData { name: "Denebola", ra: 11.8178, dec: 14.5717, magnitude: 2.14, color: [1.0, 0.95, 0.9] },
    StarData { name: "Algol", ra: 3.1361, dec: 40.9558, magnitude: 2.12, color: [0.9, 0.95, 1.0] },
    StarData { name: "Tiaki", ra: 5.9192, dec: -62.4898, magnitude: 2.15, color: [1.0, 0.6, 0.3] },
    StarData { name: "Muhlifain", ra: 12.1397, dec: -49.4256, magnitude: 2.17, color: [1.0, 0.95, 0.85] },
    StarData { name: "Aspidiske", ra: 9.2848, dec: -59.2750, magnitude: 2.21, color: [1.0, 0.85, 0.55] },
    StarData { name: "Suhail", ra: 9.1333, dec: -43.4323, magnitude: 2.23, color: [1.0, 0.6, 0.35] },
    StarData { name: "Alphecca", ra: 15.5781, dec: 26.7147, magnitude: 2.23, color: [1.0, 0.95, 0.9] },
    StarData { name: "Mintaka", ra: 5.5331, dec: -0.2991, magnitude: 2.25, color: [0.9, 0.95, 1.0] },
    StarData { name: "Sadr", ra: 20.3706, dec: 40.2567, magnitude: 2.23, color: [0.9, 0.95, 1.0] },
    StarData { name: "Eltanin", ra: 17.5075, dec: 51.4889, magnitude: 2.24, color: [1.0, 0.85, 0.55] },
    StarData { name: "Schedar", ra: 0.6750, dec: 56.5375, magnitude: 2.24, color: [1.0, 0.8, 0.55] },
    StarData { name: "Naos", ra: 8.3758, dec: -40.0031, magnitude: 2.25, color: [0.8, 0.9, 1.0] },
    StarData { name: "Almach", ra: 2.0644, dec: 42.3297, magnitude: 2.26, color: [1.0, 0.75, 0.5] },
    StarData { name: "Caph", ra: 0.1528, dec: 59.1497, magnitude: 2.28, color: [1.0, 0.95, 0.9] },
    StarData { name: "Deneb Kaitos", ra: 0.7261, dec: -17.9864, magnitude: 2.24, color: [1.0, 0.85, 0.6] },
    StarData { name: "Izar", ra: 14.7497, dec: 27.0742, magnitude: 2.35, color: [1.0, 0.7, 0.45] },
    StarData { name: "Enif", ra: 21.7364, dec: 9.8750, magnitude: 2.38, color: [1.0, 0.8, 0.5] },
    StarData { name: "Phecda", ra: 11.8967, dec: 53.6948, magnitude: 2.41, color: [0.95, 0.95, 1.0] },
    StarData { name: "Scheat", ra: 23.0629, dec: 28.0825, magnitude: 2.44, color: [1.0, 0.8, 0.6] },
    StarData { name: "Aldermin", ra: 21.4775, dec: 70.5608, magnitude: 2.45, color: [0.9, 0.95, 1.0] },
    StarData { name: "Markab", ra: 23.0794, dec: 15.2053, magnitude: 2.49, color: [0.9, 0.95, 1.0] },
    StarData { name: "Aljanah", ra: 20.3706, dec: 33.9708, magnitude: 2.48, color: [1.0, 0.75, 0.5] },
    StarData { name: "Acamar", ra: 2.9710, dec: -40.3042, magnitude: 2.88, color: [0.9, 0.95, 1.0] },
    StarData { name: "Menkar", ra: 2.9944, dec: 4.0897, magnitude: 2.54, color: [1.0, 0.75, 0.5] },
    StarData { name: "Zubenelgenubi", ra: 14.8483, dec: -16.0417, magnitude: 2.75, color: [1.0, 0.95, 0.9] },
    StarData { name: "Zubeneschamali", ra: 15.2831, dec: -9.3828, magnitude: 2.61, color: [0.9, 0.95, 1.0] },
    StarData { name: "Unukalhai", ra: 15.8458, dec: 6.4256, magnitude: 2.63, color: [1.0, 0.85, 0.55] },
    StarData { name: "Sheratan", ra: 1.9106, dec: 20.8083, magnitude: 2.64, color: [1.0, 0.95, 0.9] },
    StarData { name: "Hamal B", ra: 2.1194, dec: 23.4625, magnitude: 2.64, color: [1.0, 0.8, 0.6] },
    StarData { name: "Arneb", ra: 5.5455, dec: -17.8222, magnitude: 2.58, color: [1.0, 0.85, 0.6] },
    StarData { name: "Muphrid", ra: 13.9064, dec: 18.3978, magnitude: 2.68, color: [1.0, 0.95, 0.85] },
    StarData { name: "Algenib", ra: 0.2206, dec: 15.1836, magnitude: 2.83, color: [0.9, 0.95, 1.0] },
    StarData { name: "Deneb Algedi", ra: 21.7836, dec: -16.1275, magnitude: 2.85, color: [1.0, 0.95, 0.9] },
    StarData { name: "Graffias", ra: 16.0906, dec: -19.8056, magnitude: 2.56, color: [1.0, 0.85, 0.55] },
    StarData { name: "Markeb", ra: 9.2848, dec: -55.0106, magnitude: 2.58, color: [0.9, 0.95, 1.0] },
    StarData { name: "Sabik", ra: 17.1728, dec: -15.7247, magnitude: 2.43, color: [0.9, 0.95, 1.0] },
    StarData { name: "Alniyat", ra: 16.5981, dec: -28.2161, magnitude: 2.90, color: [1.0, 0.7, 0.4] },
    StarData { name: "Acrab", ra: 16.0906, dec: -19.8056, magnitude: 2.62, color: [1.0, 0.85, 0.55] },
    StarData { name: "Lesath", ra: 17.5606, dec: -37.9047, magnitude: 2.70, color: [0.85, 0.95, 1.0] },
    StarData { name: "Shaula", ra: 17.5606, dec: -37.1036, magnitude: 1.62, color: [0.9, 0.95, 1.0] },
    StarData { name: "Rasalhague", ra: 17.5822, dec: 12.5600, magnitude: 2.08, color: [1.0, 0.95, 0.9] },
    StarData { name: "Cebalrai", ra: 17.7247, dec: 4.5672, magnitude: 2.76, color: [1.0, 0.85, 0.55] },
    StarData { name: "Yildun", ra: 17.5369, dec: 86.5864, magnitude: 4.35, color: [1.0, 0.95, 0.9] },
    StarData { name: "Kornephoros", ra: 16.1397, dec: 21.4833, magnitude: 2.78, color: [1.0, 0.95, 0.85] },
    StarData { name: "Nekkar", ra: 15.2906, dec: 40.3906, magnitude: 3.49, color: [1.0, 0.9, 0.7] },
    StarData { name: "Seginus", ra: 14.7506, dec: 38.3181, magnitude: 3.04, color: [1.0, 0.95, 0.85] },
    StarData { name: "Izar B", ra: 14.7497, dec: 27.0742, magnitude: 4.80, color: [1.0, 0.7, 0.45] },
    StarData { name: "Merga", ra: 11.8178, dec: 24.4986, magnitude: 3.82, color: [1.0, 0.95, 0.9] },
    StarData { name: "Heze", ra: 13.5789, dec: -0.6769, magnitude: 3.38, color: [0.9, 0.95, 1.0] },
    StarData { name: "Vindemiatrix", ra: 13.0364, dec: 10.9592, magnitude: 2.85, color: [1.0, 0.95, 0.9] },
    StarData { name: "Zaniah", ra: 12.4433, dec: -1.9047, magnitude: 3.89, color: [1.0, 0.95, 0.9] },
    StarData { name: "Porrima", ra: 12.6944, dec: -1.4494, magnitude: 2.74, color: [1.0, 0.95, 0.9] },
    StarData { name: "Auva", ra: 12.4433, dec: 3.3975, magnitude: 3.38, color: [1.0, 0.75, 0.5] },
    StarData { name: "Zavijava", ra: 11.8447, dec: 1.7644, magnitude: 3.61, color: [1.0, 0.95, 0.9] },
    StarData { name: "Minelauva", ra: 12.3317, dec: -5.5358, magnitude: 4.07, color: [1.0, 0.75, 0.5] },
    StarData { name: "Ruchbah", ra: 1.4300, dec: 60.2353, magnitude: 2.66, color: [1.0, 0.95, 0.9] },
    StarData { name: "Segin", ra: 3.9033, dec: 57.8156, magnitude: 3.35, color: [0.9, 0.95, 1.0] },
    StarData { name: "Cih", ra: 0.1397, dec: 59.1497, magnitude: 2.15, color: [0.9, 0.95, 1.0] },
    StarData { name: "Tsih", ra: 0.1528, dec: 59.1497, magnitude: 2.28, color: [1.0,0.95,0.9] },
    StarData { name: "Ruchbah B", ra: 1.4300, dec: 60.2353, magnitude: 3.32, color: [1.0,0.95,0.9] },
    StarData { name: "Navi", ra: 0.3722, dec: 56.5375, magnitude: 2.15, color: [1.0,0.6,0.4] },
];

fn celestial_to_cartesian(ra_hours: f32, dec_degrees: f32, radius: f32) -> Vec3 {
    let ra_rad = ra_hours * 15.0f32.to_radians();
    let dec_rad = dec_degrees.to_radians();
    
    let x = radius * dec_rad.cos() * ra_rad.cos();
    let y = radius * dec_rad.sin();
    let z = radius * dec_rad.cos() * ra_rad.sin();
    
    Vec3::new(x, y, z)
}

fn spawn_sky_dome(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sky_radius = 800.0;
    
    for star in BRIGHT_STARS {
        let pos = celestial_to_cartesian(star.ra, star.dec, sky_radius);
        
        let brightness = 10.0f32.powf(-0.4 * star.magnitude);
        let size = 0.3 + brightness * 0.8;
        
        let star_mesh = meshes.add(Sphere::new(size).mesh().ico(4).unwrap());
        let star_material = materials.add(StandardMaterial {
            base_color: Color::srgb(star.color[0], star.color[1], star.color[2]),
            emissive: LinearRgba::new(
                star.color[0] * brightness * 2.0,
                star.color[1] * brightness * 2.0,
                star.color[2] * brightness * 2.0,
                1.0,
            ),
            ..default()
        });
        
        commands.spawn((
            PbrBundle {
                mesh: star_mesh,
                material: star_material,
                transform: Transform::from_translation(pos),
                ..default()
            },
            Star {
                ra: star.ra,
                dec: star.dec,
                magnitude: star.magnitude,
            },
            SkyDome,
        ));
    }
}

fn update_sky_dome_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SkyDome>>,
) {
    let rotation_speed = 0.004178f32.to_radians();
    for mut transform in query.iter_mut() {
        transform.rotate_y(rotation_speed * time.delta_seconds());
    }
}
