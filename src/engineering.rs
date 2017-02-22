//! Engineering related constants
//

// https://en.wikipedia.org/wiki/List_of_materials_properties

// https://en.wikipedia.org/wiki/Electrical_resistivity_and_conductivity


pub struct ElectricalInfo {
    /// [ρ (Ω·m) at 20°C]
    resistivity: f64,
    /// [σ (S/m) at 20°C]
    conductivity: f64,
    /// [K^(-1)]
    temperature_coefficient: f64,
}

pub const carbon: ElectricalInfo = ElectricalInfo {
    resistivity: 1.00e-8,
    conductivity: 1.00e8,
    temperature_coefficient: -0.0002,
};

pub const silver: ElectricalInfo = ElectricalInfo {
    resistivity: 1.59e-8,
    conductivity: 6.30e7,
    temperature_coefficient: 0.0038,
};

pub const copper: ElectricalInfo = ElectricalInfo {
    resistivity: 1.68e-8,
    conductivity: 5.96e7,
    temperature_coefficient: 0.003862,
};

pub const calcium: ElectricalInfo = ElectricalInfo {
    resistivity: 3.36e-8,
    conductivity: 2.98e7,
    temperature_coefficient: 0.0041,
};

pub const tungsten: ElectricalInfo = ElectricalInfo {
    resistivity: 5.60e-8,
    conductivity: 1.79e7,
    temperature_coefficient: 0.0045,
};

pub const zinc: ElectricalInfo = ElectricalInfo {
    resistivity: 5.90e-8,
    conductivity: 1.69e7,
    temperature_coefficient: 0.0037,
};

pub const nickel: ElectricalInfo = ElectricalInfo {
    resistivity: 6.99e-8,
    conductivity: 1.43e7,
    temperature_coefficient: 0.006,
};

pub const lithium: ElectricalInfo = ElectricalInfo {
    resistivity: 9.28e-8,
    conductivity: 1.08e7,
    temperature_coefficient: 0.006,
};

pub const iron: ElectricalInfo = ElectricalInfo {
    resistivity: 9.71e-8,
    conductivity: 1.00e7,
    temperature_coefficient: 0.005,
};

pub const platinum: ElectricalInfo = ElectricalInfo {
    resistivity: 1.06e-7,
    conductivity: 9.43e6,
    temperature_coefficient: 0.00392,
};

pub const tin: ElectricalInfo = ElectricalInfo {
    resistivity: 1.09e-7,
    conductivity: 9.17e6,
    temperature_coefficient: 0.0045,
};

pub const lead: ElectricalInfo = ElectricalInfo {
    resistivity: 2.20e-7,
    conductivity: 4.55e6,
    temperature_coefficient: 0.0039,
};

pub const titanium: ElectricalInfo = ElectricalInfo {
    resistivity: 4.20e-7,
    conductivity: 2.38e6,
    temperature_coefficient: 0.0038,
};

pub const manganin: ElectricalInfo = ElectricalInfo {
    resistivity: 4.82e-7,
    conductivity: 2.07e6,
    temperature_coefficient: 0.000002,
};

pub const constantan: ElectricalInfo = ElectricalInfo {
    resistivity: 4.90e-7,
    conductivity: 2.04e6,
    temperature_coefficient: 0.000008,
};

pub const mercury: ElectricalInfo = ElectricalInfo {
    resistivity: 9.80e-7,
    conductivity: 1.02e6,
    temperature_coefficient: 0.0009,
};

pub const germanium: ElectricalInfo = ElectricalInfo {
    resistivity: 4.60e-1,
    conductivity: 2.17,
    temperature_coefficient: -0.048,
};

pub const silicon: ElectricalInfo = ElectricalInfo {
    resistivity: 6.40e2,
    conductivity: 1.56e-3,
    temperature_coefficient: -0.075,
};

pub const nichrome: ElectricalInfo = ElectricalInfo {
    resistivity: 1.10e-6,
    conductivity: 6.7e5,
    temperature_coefficient: 0.0004,
};

pub const stainless_steel: ElectricalInfo = ElectricalInfo {
    resistivity: 6.90e-7,
    conductivity: 1.45e6,
    temperature_coefficient: 0.00094,
};



// https://en.wikipedia.org/wiki/Thermal_diffusivity
pub const pyrolytic_graphite_parallel_to_layers: f64 = 1.22e-3; // [m²/s]
pub const silver_pure: f64 = 1.6563e-4; // [m²/s]
pub const gold: f64 = 1.27e-4; // [m²/s]
pub const copper_at_25_deg: f64 = 1.11e-4; // [m²/s]
pub const aluminium: f64 = 9.7e-5; // [m²/s]
pub const inconel_600_at_25_deg: f64 = 3.428e-6; // [m²/s]
pub const molybdenum_99_95_perc_at_25_deg: f64 = 54.3e-6; // [m²/s]
//pub const iron: f64 = 2.3e-5; // [m²/s]
//pub const silicon: f64 = 8.8e-5; // [m²/s]
pub const quartz: f64 = 1.4e-6; // [m²/s]
pub const aluminium_oxide_polycrystalline: f64 = 1.20e-5; // [m²/s]
pub const silicon_dioxide_polycrystalline: f64 = 8.3e-7; // [m²/s]
pub const alcohol: f64 = 7e-8; // [m²/s]
//pub const tin: f64 = 4.0e-5; // [m²/s]
pub const brick_common: f64 = 5.2e-7; // [m²/s]
pub const brick_adobe: f64 = 2.7e-7; // [m²/s]
pub const glass_window: f64 = 3.4e-7; // [m²/s]
pub const nylon: f64 = 9e-8; // [m²/s]
pub const wood_yellow_pine: f64 = 8.2e-8; // [m²/s]
