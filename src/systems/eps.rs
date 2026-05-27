use bevy::prelude::*;
use super::SystemStatus;

#[derive(Component, Debug, Clone)]
pub struct ElectricalSystem {
    pub fuel_cells: Vec<FuelCell>,
    pub batteries: Vec<Battery>,
    pub inverters: Vec<Inverter>,
    pub power_distribution: PowerDistribution,
    pub bus_voltage: f32,
    pub total_power_kw: f32,
    pub main_bus_a: bool,
    pub main_bus_b: bool,
}

impl Default for ElectricalSystem {
    fn default() -> Self {
        let buses = vec![
            DcBus::new("MAIN_BUS_A", 28.0),
            DcBus::new("MAIN_BUS_B", 28.0),
            DcBus::new("BAT_BUS_A", 28.0),
            DcBus::new("BAT_BUS_B", 28.0),
        ];
        let ac_buses = vec![
            AcBus::new("AC_BUS_1", 115.0, 400.0),
            AcBus::new("AC_BUS_2", 115.0, 400.0),
            AcBus::new("AC_BUS_3", 115.0, 400.0),
        ];

        Self {
            fuel_cells: vec![
                FuelCell::new("FC1", 1.42),
                FuelCell::new("FC2", 1.42),
                FuelCell::new("FC3", 1.42),
            ],
            batteries: vec![
                Battery::new("BAT A", 40.0),
                Battery::new("BAT B", 40.0),
                Battery::new("BAT C", 40.0),
            ],
            inverters: vec![
                Inverter::new("INV1", "MAIN_BUS_A"),
                Inverter::new("INV2", "MAIN_BUS_B"),
                Inverter::new("INV3", "MAIN_BUS_A"),
            ],
            power_distribution: PowerDistribution {
                buses: buses.clone(),
                ac_buses: ac_buses.clone(),
                total_load_kw: 0.0,
            },
            bus_voltage: 28.0,
            total_power_kw: 2.0,
            main_bus_a: true,
            main_bus_b: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuelCell {
    pub id: String,
    pub output_kw: f32,
    pub temp_c: f32,
    pub h2_pressure_psi: f32,
    pub o2_pressure_psi: f32,
    pub n2_pressure_psi: f32,
    pub water_production_rate_lb_hr: f32,
    pub radiator_bypass_pct: f32,
    pub condenser_exhaust_temp_f: f32,
    pub status: SystemStatus,
}

impl FuelCell {
    pub fn new(id: &str, output_kw: f32) -> Self {
        Self {
            id: id.to_string(),
            output_kw,
            temp_c: 200.0,
            h2_pressure_psi: 61.5,
            o2_pressure_psi: 61.5,
            n2_pressure_psi: 53.0,
            water_production_rate_lb_hr: 0.77,
            radiator_bypass_pct: 0.0,
            condenser_exhaust_temp_f: 150.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Battery {
    pub id: String,
    pub capacity_ah: f32,
    pub charge_pct: f32,
    pub open_circuit_voltage: f32,
    pub max_voltage: f32,
    pub min_voltage: f32,
    pub temp_f: f32,
    pub charge_rate_a: f32,
    pub status: SystemStatus,
}

impl Battery {
    pub fn new(id: &str, capacity_ah: f32) -> Self {
        Self {
            id: id.to_string(),
            capacity_ah,
            charge_pct: 100.0,
            open_circuit_voltage: 37.2,
            max_voltage: 37.8,
            min_voltage: 27.0,
            temp_f: 70.0,
            charge_rate_a: 40.0,
            status: SystemStatus::Nominal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inverter {
    pub id: String,
    pub input_voltage_vdc: f32,
    pub output_voltage_vac: f32,
    pub frequency_hz: f32,
    pub capacity_va: f32,
    pub phase_displacement_deg: f32,
    pub status: SystemStatus,
    pub powered_by: String,
}

impl Inverter {
    pub fn new(id: &str, powered_by: &str) -> Self {
        Self {
            id: id.to_string(),
            input_voltage_vdc: 28.0,
            output_voltage_vac: 115.0,
            frequency_hz: 400.0,
            capacity_va: 1250.0,
            phase_displacement_deg: 120.0,
            status: SystemStatus::Nominal,
            powered_by: powered_by.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DcBus {
    pub id: String,
    pub voltage: f32,
    pub current_draw_a: f32,
    pub powered: bool,
}

impl DcBus {
    pub fn new(id: &str, voltage: f32) -> Self {
        Self {
            id: id.to_string(),
            voltage,
            current_draw_a: 0.0,
            powered: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AcBus {
    pub id: String,
    pub voltage: f32,
    pub frequency_hz: f32,
    pub current_draw_a: f32,
    pub powered: bool,
}

impl AcBus {
    pub fn new(id: &str, voltage: f32, frequency_hz: f32) -> Self {
        Self {
            id: id.to_string(),
            voltage,
            frequency_hz,
            current_draw_a: 0.0,
            powered: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerDistribution {
    pub buses: Vec<DcBus>,
    pub ac_buses: Vec<AcBus>,
    pub total_load_kw: f32,
}

pub fn update_electrical_system(eps: &mut ElectricalSystem, dt: f32) {
    for fc in &mut eps.fuel_cells {
        if fc.status != SystemStatus::Failed && fc.status != SystemStatus::Off {
            let target_temp_c = 196.0 + (fc.output_kw - 1.4) * 15.0;
            fc.temp_c = fc.temp_c.lerp(target_temp_c, 0.1 * dt);

            if fc.temp_c > 260.0 {
                fc.status = SystemStatus::Warning;
            }
            if fc.temp_c > 300.0 {
                fc.status = SystemStatus::Critical;
            }

            fc.h2_pressure_psi = (61.5 - (fc.output_kw - 1.4) * 2.0).max(20.0);
            fc.o2_pressure_psi = (61.5 - (fc.output_kw - 1.4) * 2.0).max(20.0);
        }
    }

    let fuel_cell_power: f32 = eps.fuel_cells
        .iter()
        .filter(|fc| fc.status != SystemStatus::Failed && fc.status != SystemStatus::Off)
        .map(|fc| fc.output_kw)
        .sum();

    let battery_power: f32 = eps.batteries
        .iter()
        .filter(|b| b.status != SystemStatus::Failed && b.charge_pct > 0.0)
        .map(|b| {
            let voltage = b.open_circuit_voltage * (b.charge_pct / 100.0);
            voltage * b.charge_rate_a / 1000.0
        })
        .sum();

    eps.total_power_kw = fuel_cell_power + battery_power;

    if eps.main_bus_a {
        if let Some(bus) = eps.power_distribution.buses.iter_mut().find(|b| b.id == "MAIN_BUS_A") {
            bus.voltage = eps.bus_voltage;
            bus.powered = true;
        }
    } else {
        if let Some(bus) = eps.power_distribution.buses.iter_mut().find(|b| b.id == "MAIN_BUS_A") {
            bus.powered = false;
            bus.voltage = 0.0;
        }
    }

    if eps.main_bus_b {
        if let Some(bus) = eps.power_distribution.buses.iter_mut().find(|b| b.id == "MAIN_BUS_B") {
            bus.voltage = eps.bus_voltage;
            bus.powered = true;
        }
    } else {
        if let Some(bus) = eps.power_distribution.buses.iter_mut().find(|b| b.id == "MAIN_BUS_B") {
            bus.powered = false;
            bus.voltage = 0.0;
        }
    }

    for inverter in &mut eps.inverters {
        let bus_powered = match inverter.powered_by.as_str() {
            "MAIN_BUS_A" => eps.main_bus_a,
            "MAIN_BUS_B" => eps.main_bus_b,
            _ => false,
        };

        if bus_powered && inverter.status != SystemStatus::Failed {
            inverter.status = SystemStatus::Nominal;
            let ac_bus_id = match inverter.id.as_str() {
                "INV1" => "AC_BUS_1",
                "INV2" => "AC_BUS_2",
                "INV3" => "AC_BUS_3",
                _ => "",
            };
            if let Some(ac_bus) = eps.power_distribution.ac_buses.iter_mut().find(|b| b.id == ac_bus_id) {
                ac_bus.powered = true;
                ac_bus.voltage = inverter.output_voltage_vac;
                ac_bus.frequency_hz = inverter.frequency_hz;
            }
        } else if inverter.status != SystemStatus::Failed {
            inverter.status = SystemStatus::Off;
            let ac_bus_id = match inverter.id.as_str() {
                "INV1" => "AC_BUS_1",
                "INV2" => "AC_BUS_2",
                "INV3" => "AC_BUS_3",
                _ => "",
            };
            if let Some(ac_bus) = eps.power_distribution.ac_buses.iter_mut().find(|b| b.id == ac_bus_id) {
                ac_bus.powered = false;
                ac_bus.voltage = 0.0;
            }
        }
    }

    let dc_load: f32 = eps.power_distribution.buses.iter().map(|b| b.current_draw_a * b.voltage / 1000.0).sum();
    let ac_load: f32 = eps.power_distribution.ac_buses.iter().map(|b| b.current_draw_a * b.voltage / 1000.0).sum();
    eps.power_distribution.total_load_kw = dc_load + ac_load;

    let load = eps.power_distribution.total_load_kw;
    if load > fuel_cell_power {
        let deficit = load - fuel_cell_power;
        let battery_count = eps.batteries.len();
        for battery in &mut eps.batteries {
            if battery.status != SystemStatus::Failed && battery.charge_pct > 0.0 {
                let discharge_rate = (deficit / battery_count as f32) * 100.0 / battery.capacity_ah * dt / 3600.0;
                battery.charge_pct -= discharge_rate;
                if battery.charge_pct < 10.0 {
                    battery.status = SystemStatus::Warning;
                }
                if battery.charge_pct <= 0.0 {
                    battery.status = SystemStatus::Failed;
                    battery.charge_pct = 0.0;
                }
            }
        }
    }
}
