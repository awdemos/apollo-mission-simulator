use bevy::prelude::*;

pub struct Gn2Plugin;

impl Plugin for Gn2Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gn2System>()
            .add_systems(Update, update_gn2_system.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Gn2System {
    pub tanks: Vec<Gn2Tank>,
    pub distribution_pressure_psi: f32,
    pub regulator_pressure_psi: f32,
    pub manifold_valves: [bool; 4],
    pub total_consumption_kg: f32,
    pub ground_support_connected: bool,
    pub status: crate::systems::SystemStatus,
}

impl Default for Gn2System {
    fn default() -> Self {
        Self {
            tanks: vec![
                Gn2Tank::new("GN2-1", 15.0, 3000.0),
                Gn2Tank::new("GN2-2", 15.0, 3000.0),
            ],
            distribution_pressure_psi: 245.0,
            regulator_pressure_psi: 245.0,
            manifold_valves: [false; 4],
            total_consumption_kg: 0.0,
            ground_support_connected: true,
            status: crate::systems::SystemStatus::Off,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gn2Tank {
    pub name: String,
    pub capacity_kg: f32,
    pub current_mass_kg: f32,
    pub pressure_psi: f32,
    pub temperature_k: f32,
    pub max_pressure_psi: f32,
    pub valve_open: bool,
    pub heater_active: bool,
}

impl Gn2Tank {
    pub fn new(name: &str, capacity_kg: f32, max_pressure_psi: f32) -> Self {
        Self {
            name: name.to_string(),
            capacity_kg,
            current_mass_kg: capacity_kg,
            pressure_psi: 245.0,
            temperature_k: 293.0,
            max_pressure_psi,
            valve_open: false,
            heater_active: false,
        }
    }

    pub fn pressure_ratio(&self) -> f32 {
        self.pressure_psi / self.max_pressure_psi
    }

    pub fn mass_ratio(&self) -> f32 {
        self.current_mass_kg / self.capacity_kg
    }
}

#[derive(Component, Debug, Clone)]
pub struct PneumaticConsumer {
    pub name: String,
    pub consumption_rate_kg_s: f32,
    pub min_pressure_psi: f32,
    pub active: bool,
}

fn update_gn2_system(
    time: Res<Time>,
    time_scale: Res<crate::TimeScale>,
    mut gn2: ResMut<Gn2System>,
    mut consumers: Query<&mut PneumaticConsumer>,
) {
    let dt = time.delta_seconds() * time_scale.multiplier;

    if gn2.status == crate::systems::SystemStatus::Off {
        return;
    }

    let mut total_consumption = 0.0;
    for mut consumer in consumers.iter_mut() {
        if consumer.active && gn2.distribution_pressure_psi >= consumer.min_pressure_psi {
            total_consumption += consumer.consumption_rate_kg_s * dt;
        }
    }

    let tank_draw = total_consumption / gn2.tanks.iter().filter(|t| t.valve_open).count().max(1) as f32;

    for tank in &mut gn2.tanks {
        if tank.valve_open {
            tank.current_mass_kg -= tank_draw;
            tank.current_mass_kg = tank.current_mass_kg.max(0.0);
            tank.pressure_psi = 245.0 * tank.mass_ratio();

            if tank.heater_active && tank.pressure_psi < 200.0 {
                tank.temperature_k += 2.0 * dt;
                tank.pressure_psi += 5.0 * dt;
            }
        }
    }

    gn2.total_consumption_kg += total_consumption;
    gn2.distribution_pressure_psi = gn2.tanks.iter()
        .filter(|t| t.valve_open)
        .map(|t| t.pressure_psi)
        .fold(0.0, |a, b| a.max(b));

    if gn2.ground_support_connected {
        for tank in &mut gn2.tanks {
            if tank.pressure_psi < 240.0 {
                tank.pressure_psi += 10.0 * dt;
                tank.current_mass_kg = (tank.current_mass_kg + 0.5 * dt).min(tank.capacity_kg);
            }
        }
    }
}

pub fn create_propellant_tank_pressurization(name: &str) -> PneumaticConsumer {
    PneumaticConsumer {
        name: name.to_string(),
        consumption_rate_kg_s: 0.8,
        min_pressure_psi: 180.0,
        active: false,
    }
}

pub fn create_pneumatic_valve_actuation(name: &str) -> PneumaticConsumer {
    PneumaticConsumer {
        name: name.to_string(),
        consumption_rate_kg_s: 0.05,
        min_pressure_psi: 150.0,
        active: false,
    }
}

pub fn create_fire_suppression_system(name: &str) -> PneumaticConsumer {
    PneumaticConsumer {
        name: name.to_string(),
        consumption_rate_kg_s: 2.5,
        min_pressure_psi: 200.0,
        active: false,
    }
}
