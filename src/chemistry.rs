//! Chemistry related constants

pub const absolute_zero: f64 = -273.15; // [°C]
pub const avogadros_number: f64 = 6.024e23; // [molecules/mol]
pub const boltzmanns_constant: f64 = 1.380e-23; // [J/K * molecule]
pub const steffan_boltzmann_constant: f64 = 5.670e-8 ; // [W/m² * K⁴]
pub const universal_gas_constant: f64 = 8.31434e3; // [J/kmol * K]
pub const normal_atmospheric_pressure: f64 = 101325.0; // [N/m²]

pub enum StateOfMatter { // at 0 °C and 1 atm
    Solid,
    Liquid,
    Gas,
    Unknown,
}

pub enum SubCategory {
    AlkaliMetal,
    AlkalineEarthMetal,
    Lanthanide,
    Actinide,
    TransitionMetal,
    PostTransitionMetal,
    Metalloid,
    PolyatomicNonMetal,
    DiatomicNonMetal,
    NobleGas,
    Unknown,
}


pub struct AtomInfo {
    name: &'static str,
    atomic_number: u32,
    group: u32,
    period: u8,
    density: f64, // [g/cm³]
    melt: Option<f64>, // [K]
    boil: Option<f64>, // [K]
    electronegativity: Option<f64>,
    number_of_neutrons: u32,
    mass: f64, // https://en.wikipedia.org/wiki/Relative_atomic_mass
    state_of_matter: StateOfMatter,
    sub_category: SubCategory,
    // A lot of more stuff to be added...
}

pub const atom_h: AtomInfo = AtomInfo {
    name: "H",
    atomic_number: 1,
    group: 1,
    period: 1,
    density: 0.00008988,
    melt: Some(14.01),
    boil: Some(20.28),
    electronegativity: Some(2.20),
    number_of_neutrons: 0,
    mass: 1.008,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_he: AtomInfo = AtomInfo {
    name: "He",
    atomic_number: 2,
    group: 18,
    period: 1,
    density: 0.0001785,
    melt: Some(0.95),
    boil: Some(4.22),
    electronegativity: None,
    number_of_neutrons: 2,
    mass: 4.0026022,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};


pub const atom_li: AtomInfo = AtomInfo {
    name: "Li",
    atomic_number: 3,
    group: 1,
    period: 2,
    density: 0.534,
    melt: Some(453.69),
    boil: Some(1560.0),
    electronegativity: Some(0.98),
    number_of_neutrons: 4,
    mass: 6.94,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_be: AtomInfo = AtomInfo {
    name: "Be",
    atomic_number: 4,
    group: 2,
    period: 2,
    density: 1.85,
    melt: Some(1560.0),
    boil: Some(2742.0),
    electronegativity: Some(1.57),
    number_of_neutrons: 5,
    mass: 9.0121831,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkalineEarthMetal,
};

pub const atom_b: AtomInfo = AtomInfo {
    name: "B",
    atomic_number: 5,
    group: 13,
    period: 2,
    density: 2.34,
    melt: Some(2349.0),
    boil: Some(4200.0),
    electronegativity: Some(2.04),
    number_of_neutrons: 6,
    mass: 10.81,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_c: AtomInfo = AtomInfo {
    name: "C",
    atomic_number: 6,
    group: 14,
    period: 2,
    density: 2.267,
    melt: Some(3800.0),
    boil: Some(4300.0),
    electronegativity: Some(2.55),
    number_of_neutrons: 6,
    mass: 12.011,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PolyatomicNonMetal,
};

pub const atom_n: AtomInfo = AtomInfo {
    name: "N",
    atomic_number: 7,
    group: 15,
    period: 2,
    density: 0.0012506,
    melt: Some(63.15),
    boil: Some(77.36),
    electronegativity: Some(3.04),
    number_of_neutrons: 7,
    mass: 14.007,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_o: AtomInfo = AtomInfo {
    name: "O",
    atomic_number: 8,
    group: 16,
    period: 2,
    density: 0.001429,
    melt: Some(54.36),
    boil: Some(90.20),
    electronegativity: Some(3.44),
    number_of_neutrons: 8,
    mass: 15.999,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_f: AtomInfo = AtomInfo {
    name: "F",
    atomic_number: 9,
    group: 17,
    period: 2,
    density: 0.001696,
    melt: Some(53.53),
    boil: Some(85.03),
    electronegativity: Some(3.98),
    number_of_neutrons: 10,
    mass: 18.998403163,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_ne: AtomInfo = AtomInfo {
    name: "Ne",
    atomic_number: 10,
    group: 18,
    period: 2,
    density: 0.0008999,
    melt: Some(24.56),
    boil: Some( 27.07),
    electronegativity: None,
    number_of_neutrons: 10,
    mass: 20.1797,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};

pub const atom_na: AtomInfo = AtomInfo {
    name: "Na",
    atomic_number: 11,
    group: 1,
    period: 3,
    density: 0.971,
    melt: Some(370.87),
    boil: Some(1156.0),
    electronegativity: Some(0.93),
    number_of_neutrons: 12,
    mass: 22.98976928,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_mg: AtomInfo = AtomInfo {
    name: "Mg",
    atomic_number: 12,
    group: 2,
    period: 3,
    density: 1.738,
    melt: Some(923.0),
    boil: Some(1363.0),
    electronegativity: Some(1.31),
    number_of_neutrons: 12,
    mass: 24.305,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkalineEarthMetal,
};

pub const atom_al: AtomInfo = AtomInfo {
    name: "Al",
    atomic_number: 13,
    group: 13,
    period: 3,
    density: 2.698,
    melt: Some(933.47),
    boil: Some(2792.0),
    electronegativity: Some(1.61),
    number_of_neutrons: 14,
    mass: 26.9815385,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_si: AtomInfo = AtomInfo {
    name: "Si",
    atomic_number: 14,
    group: 14,
    period: 3,
    density: 2.3296,
    melt: Some(1687.0),
    boil: Some(3538.0),
    electronegativity: Some(1.9),
    number_of_neutrons: 14,
    mass: 28.085,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_p: AtomInfo = AtomInfo {
    name: "P",
    atomic_number: 15,
    group: 15,
    period: 3,
    density: 1.82,
    melt: Some(317.30),
    boil: Some(550.0),
    electronegativity: Some(2.19),
    number_of_neutrons: 16,
    mass: 30.973761998,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PolyatomicNonMetal,
};

pub const atom_s: AtomInfo = AtomInfo {
    name: "S",
    atomic_number: 16,
    group: 16,
    period: 3,
    density: 2.067,
    melt: Some(388.36),
    boil: Some(717.87),
    electronegativity: Some(2.58),
    number_of_neutrons: 16,
    mass: 32.06,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PolyatomicNonMetal,
};

pub const atom_cl: AtomInfo = AtomInfo {
    name: "Cl",
    atomic_number: 17,
    group: 17,
    period: 3,
    density: 0.003214,
    melt: Some(171.6),
    boil: Some(239.11),
    electronegativity: Some(3.16),
    number_of_neutrons: 18,
    mass: 35.45,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_ar: AtomInfo = AtomInfo {
    name: "Ar",
    atomic_number: 18,
    group: 18,
    period: 3,
    density: 0.0017837,
    melt: Some(83.80),
    boil: Some(87.30),
    electronegativity: None,
    number_of_neutrons: 22,
    mass: 39.948,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};

pub const atom_k: AtomInfo = AtomInfo {
    name: "K",
    atomic_number: 19,
    group: 1,
    period: 4,
    density: 0.862,
    melt: Some(336.53),
    boil: Some(1032.0),
    electronegativity: Some(0.82),
    number_of_neutrons: 20,
    mass: 39.0983,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_ca: AtomInfo = AtomInfo {
    name: "Ca",
    atomic_number: 20,
    group: 2,
    period: 4,
    density: 1.54,
    melt: Some(1115.0),
    boil: Some(1757.0),
    electronegativity: Some(1.0),
    number_of_neutrons: 20,
    mass: 40.078,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkalineEarthMetal,
};

pub const atom_sc: AtomInfo = AtomInfo {
    name: "Sc",
    atomic_number: 21,
    group: 3,
    period: 4,
    density: 2.989,
    melt: Some(1814.0),
    boil: Some(3109.0),
    electronegativity: Some(1.36),
    number_of_neutrons: 24,
    mass: 44.955908,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ti: AtomInfo = AtomInfo {
    name: "Ti",
    atomic_number: 22,
    group: 4,
    period: 4,
    density: 4.54,
    melt: Some(1941.0),
    boil: Some(3560.0),
    electronegativity: Some(1.54),
    number_of_neutrons: 26,
    mass: 47.867,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_v: AtomInfo = AtomInfo {
    name: "V",
    atomic_number: 23,
    group: 5,
    period: 4,
    density: 6.11,
    melt: Some(2183.0),
    boil: Some(3680.0),
    electronegativity: Some(1.63),
    number_of_neutrons: 28,
    mass: 50.9415,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_cr: AtomInfo = AtomInfo {
    name: "Cr",
    atomic_number: 24,
    group: 6,
    period: 4,
    density: 7.15,
    melt: Some(2180.0),
    boil: Some(2944.0),
    electronegativity: Some(1.66),
    number_of_neutrons: 28,
    mass: 51.9961,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_mn: AtomInfo = AtomInfo {
    name: "Mn",
    atomic_number: 25,
    group: 7,
    period: 4,
    density: 7.44,
    melt: Some(1519.0),
    boil: Some(2334.0),
    electronegativity: Some(1.55),
    number_of_neutrons: 30,
    mass: 54.938044,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_fe: AtomInfo = AtomInfo {
    name: "Fe",
    atomic_number: 26,
    group: 8,
    period: 4,
    density: 7.874,
    melt: Some(1811.0),
    boil: Some(3134.0),
    electronegativity: Some(1.83),
    number_of_neutrons: 30,
    mass: 55.845,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_co: AtomInfo = AtomInfo {
    name: "Co",
    atomic_number: 27,
    group: 9,
    period: 4,
    density: 8.86,
    melt: Some(1768.0),
    boil: Some(3200.0),
    electronegativity: Some(1.88),
    number_of_neutrons: 32,
    mass: 58.933194,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ni: AtomInfo = AtomInfo {
    name: "Ni",
    atomic_number: 28,
    group: 10,
    period: 4,
    density: 8.912,
    melt: Some(1728.0),
    boil: Some(3186.0),
    electronegativity: Some(1.91),
    number_of_neutrons: 30,
    mass: 58.6934,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_cu: AtomInfo = AtomInfo {
    name: "Cu",
    atomic_number: 29,
    group: 11,
    period: 4,
    density: 8.96,
    melt: Some(1357.77),
    boil: Some(2835.0),
    electronegativity: Some(1.9),
    number_of_neutrons: 34,
    mass: 63.546,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_zn: AtomInfo = AtomInfo {
    name: "Zn",
    atomic_number: 30,
    group: 12,
    period: 4,
    density: 7.134,
    melt: Some(692.88),
    boil: Some(1180.0),
    electronegativity: Some(1.65),
    number_of_neutrons: 34,
    mass: 65.38,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ga: AtomInfo = AtomInfo {
    name: "Ga",
    atomic_number: 31,
    group: 13,
    period: 4,
    density: 5.907,
    melt: Some(302.9146),
    boil: Some(2673.0),
    electronegativity: Some(1.81),
    number_of_neutrons: 38,
    mass: 69.723,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_ge: AtomInfo = AtomInfo {
    name: "Ge",
    atomic_number: 32,
    group: 14,
    period: 4,
    density: 5.323,
    melt: Some(1211.40),
    boil: Some(3106.0),
    electronegativity: Some(2.01),
    number_of_neutrons: 42,
    mass: 72.630,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_as: AtomInfo = AtomInfo {
    name: "As",
    atomic_number: 33,
    group: 15,
    period: 4,
    density: 5.776,
    melt: Some(1090.0),
    boil: Some(887.0),
    electronegativity: Some(2.18),
    number_of_neutrons: 42,
    mass: 74.921595,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_se: AtomInfo = AtomInfo {
    name: "Se",
    atomic_number: 34,
    group: 16,
    period: 4,
    density: 4.809,
    melt: Some(453.0),
    boil: Some(958.0),
    electronegativity: Some(2.55),
    number_of_neutrons: 46,
    mass: 78.971,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PolyatomicNonMetal,
};

pub const atom_br: AtomInfo = AtomInfo {
    name: "Br",
    atomic_number: 35,
    group: 17,
    period: 4,
    density: 3.122,
    melt: Some(265.8),
    boil: Some(332.0),
    electronegativity: Some(2.96),
    number_of_neutrons: 44,
    mass: 79.904,
    state_of_matter: StateOfMatter::Liquid,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_kr: AtomInfo = AtomInfo {
    name: "Kr",
    atomic_number: 36,
    group: 18,
    period: 4,
    density: 0.003733,
    melt: Some(115.79),
    boil: Some(119.93),
    electronegativity: Some(3.0),
    number_of_neutrons: 48,
    mass: 83.798,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};

pub const atom_rb: AtomInfo = AtomInfo {
    name: "Rb",
    atomic_number: 37,
    group: 1,
    period: 5,
    density: 1.532,
    melt: Some(312.46),
    boil: Some(961.0),
    electronegativity: Some(0.82),
    number_of_neutrons: 48,
    mass: 85.4678,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_sr: AtomInfo = AtomInfo {
    name: "Sr",
    atomic_number: 38,
    group: 2,
    period: 5,
    density: 2.64,
    melt: Some(1050.0),
    boil: Some(1655.0),
    electronegativity: Some(0.95),
    number_of_neutrons: 50,
    mass: 87.62,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkalineEarthMetal,
};

pub const atom_y: AtomInfo = AtomInfo {
    name: "Y",
    atomic_number: 39,
    group: 3,
    period: 5,
    density: 4.469,
    melt: Some(1799.0),
    boil: Some(3609.0),
    electronegativity: Some(1.22),
    number_of_neutrons: 50,
    mass: 88.90584,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_zr: AtomInfo = AtomInfo {
    name: "Zr",
    atomic_number: 40,
    group: 4,
    period: 5,
    density: 6.506,
    melt: Some(2128.0),
    boil: Some(4682.0),
    electronegativity: Some(1.33),
    number_of_neutrons: 50,
    mass: 91.224,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_nb: AtomInfo = AtomInfo {
    name: "Nb",
    atomic_number: 41,
    group: 5,
    period: 5,
    density: 8.57,
    melt: Some(2750.0),
    boil: Some(5017.0),
    electronegativity: Some(1.6),
    number_of_neutrons: 52,
    mass: 92.90637,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_mo: AtomInfo = AtomInfo {
    name: "Mo",
    atomic_number: 42,
    group: 6,
    period: 5,
    density: 10.22,
    melt: Some(2896.0),
    boil: Some(4912.0),
    electronegativity: Some(2.16),
    number_of_neutrons: 56,
    mass: 95.95,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_tc: AtomInfo = AtomInfo {
    name: "Tc",
    atomic_number: 43,
    group: 7,
    period: 5,
    density: 11.5,
    melt: Some(2430.0),
    boil: Some(4538.0),
    electronegativity: Some(1.9),
    number_of_neutrons: 55,
    mass: 98.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ru: AtomInfo = AtomInfo {
    name: "Ru",
    atomic_number: 44,
    group: 8,
    period: 5,
    density: 12.37,
    melt: Some(2607.0),
    boil: Some(4423.0),
    electronegativity: Some(2.2),
    number_of_neutrons: 58,
    mass: 101.07,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_rh: AtomInfo = AtomInfo {
    name: "Rh",
    atomic_number: 45,
    group: 9,
    period: 5,
    density: 12.41,
    melt: Some(2237.0),
    boil: Some(3968.0),
    electronegativity: Some(2.28),
    number_of_neutrons: 58,
    mass: 102.90550,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_pd: AtomInfo = AtomInfo {
    name: "Pd",
    atomic_number: 46,
    group: 10,
    period: 5,
    density: 12.02,
    melt: Some(1828.05),
    boil: Some(3236.0),
    electronegativity: Some(2.2),
    number_of_neutrons: 62,
    mass: 106.42,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ag: AtomInfo = AtomInfo {
    name: "Ag",
    atomic_number: 47,
    group: 11,
    period: 5,
    density: 10.501,
    melt: Some(1234.93),
    boil: Some(2435.0),
    electronegativity: Some(1.93),
    number_of_neutrons: 60,
    mass: 107.8682,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_cd: AtomInfo = AtomInfo {
    name: "Cd",
    atomic_number: 48,
    group: 12,
    period: 5,
    density: 8.69,
    melt: Some(594.22),
    boil: Some(1040.0),
    electronegativity: Some(1.69),
    number_of_neutrons: 66,
    mass: 112.414,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_in: AtomInfo = AtomInfo {
    name: "In",
    atomic_number: 49,
    group: 13,
    period: 5,
    density: 7.31,
    melt: Some(429.75),
    boil: Some(2345.0),
    electronegativity: Some(1.78),
    number_of_neutrons: 66,
    mass: 114.818,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_sn: AtomInfo = AtomInfo {
    name: "Sn",
    atomic_number: 50,
    group: 14,
    period: 5,
    density: 7.287,
    melt: Some(505.08),
    boil: Some(2875.0),
    electronegativity: Some(1.96),
    number_of_neutrons: 70,
    mass: 118.710,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_sb: AtomInfo = AtomInfo {
    name: "Sb",
    atomic_number: 51,
    group: 15,
    period: 5,
    density: 6.685,
    melt: Some(903.78),
    boil: Some(1860.0),
    electronegativity: Some(2.05),
    number_of_neutrons: 70,
    mass: 121.760,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_te: AtomInfo = AtomInfo {
    name: "Te",
    atomic_number: 52,
    group: 16,
    period: 5,
    density: 6.232,
    melt: Some(722.66),
    boil: Some(1261.0),
    electronegativity: Some(2.1),
    number_of_neutrons: 74,
    mass: 127.60,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_i: AtomInfo = AtomInfo {
    name: "I",
    atomic_number: 53,
    group: 17,
    period: 5,
    density: 4.93,
    melt: Some(386.85),
    boil: Some(457.4),
    electronegativity: Some(2.66),
    number_of_neutrons: 74,
    mass: 126.90447,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::DiatomicNonMetal,
};

pub const atom_xe: AtomInfo = AtomInfo {
    name: "Xe",
    atomic_number: 54,
    group: 18,
    period: 5,
    density: 0.005887,
    melt: Some(161.4),
    boil: Some(165.03),
    electronegativity: Some(2.6),
    number_of_neutrons: 78,
    mass: 131.293,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};

pub const atom_cs: AtomInfo = AtomInfo {
    name: "Cs",
    atomic_number: 55,
    group: 1,
    period: 6,
    density: 1.873,
    melt: Some(301.59),
    boil: Some(944.0),
    electronegativity: Some(0.79),
    number_of_neutrons: 78,
    mass: 132.90545196,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_ba: AtomInfo = AtomInfo {
    name: "Ba",
    atomic_number: 56,
    group: 2,
    period: 6,
    density: 3.594,
    melt: Some(1000.0),
    boil: Some(2170.0),
    electronegativity: Some(0.89),
    number_of_neutrons: 82,
    mass: 137.327,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkalineEarthMetal,
};

pub const atom_la: AtomInfo = AtomInfo {
    name: "La",
    atomic_number: 57,
    group: 3,
    period: 6,
    density: 6.145,
    melt: Some(1193.0),
    boil: Some(3737.0),
    electronegativity: Some(1.1),
    number_of_neutrons: 82,
    mass: 138.90547,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_ce: AtomInfo = AtomInfo {
    name: "Ce",
    atomic_number: 58,
    period: 6,
    group: 0,
    density: 6.77,
    melt: Some(1068.0),
    boil: Some(3716.0),
    electronegativity: Some(1.12),
    number_of_neutrons: 82,
    mass: 140.116,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_pr: AtomInfo = AtomInfo {
    name: "Pr",
    atomic_number: 59,
    period: 6,
    group: 0,
    density: 6.773,
    melt: Some(1208.0),
    boil: Some(3793.0),
    electronegativity: Some(1.13),
    number_of_neutrons: 82,
    mass: 140.90766,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_nd: AtomInfo = AtomInfo {
    name: "Nd",
    atomic_number: 60,
    period: 6,
    group: 0,
    density: 7.007,
    melt: Some(1297.0),
    boil: Some(3347.0),
    electronegativity: Some(1.14),
    number_of_neutrons: 82,
    mass: 144.242,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_pm: AtomInfo = AtomInfo {
    name: "Pm",
    atomic_number: 61,
    period: 6,
    group: 0,
    density: 7.26,
    melt: Some(1315.0),
    boil: Some(3273.0),
    electronegativity: Some(1.13),
    number_of_neutrons: 84,
    mass: 145.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_sm: AtomInfo = AtomInfo {
    name: "Sm",
    atomic_number: 62,
    period: 6,
    group: 0,
    density: 7.52,
    melt: Some(1345.0),
    boil: Some(2067.0),
    electronegativity: Some(1.17),
    number_of_neutrons: 90,
    mass: 150.36,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_eu: AtomInfo = AtomInfo {
    name: "Eu",
    atomic_number: 63,
    period: 6,
    group: 0,
    density: 5.243,
    melt: Some(1099.0),
    boil: Some(1802.0),
    electronegativity: Some(1.2),
    number_of_neutrons: 90,
    mass: 151.964,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_gd: AtomInfo = AtomInfo {
    name: "Gd",
    atomic_number: 64,
    period: 6,
    group: 0,
    density: 7.895,
    melt: Some(1585.0),
    boil: Some(3546.0),
    electronegativity: Some(1.2),
    number_of_neutrons: 94,
    mass: 157.25,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_tb: AtomInfo = AtomInfo {
    name: "Tb",
    atomic_number: 65,
    period: 6,
    group: 0,
    density: 8.229,
    melt: Some(1629.0),
    boil: Some(3503.0),
    electronegativity: Some(1.2),
    number_of_neutrons: 94,
    mass: 158.92535,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_dy: AtomInfo = AtomInfo {
    name: "Dy",
    atomic_number: 66,
    period: 6,
    group: 0,
    density: 8.55,
    melt: Some(1680.0),
    boil: Some(2840.0),
    electronegativity: Some(1.22),
    number_of_neutrons: 98,
    mass: 162.500,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_ho: AtomInfo = AtomInfo {
    name: "Ho",
    atomic_number: 67,
    period: 6,
    group: 0,
    density: 8.795,
    melt: Some(1734.0),
    boil: Some(2993.0),
    electronegativity: Some(1.23),
    number_of_neutrons: 98,
    mass: 164.93033,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_er: AtomInfo = AtomInfo {
    name: "Er",
    atomic_number: 68,
    period: 6,
    group: 0,
    density: 9.066,
    melt: Some(1802.0),
    boil: Some(3141.0),
    electronegativity: Some(1.24),
    number_of_neutrons: 98,
    mass: 167.259,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_tm: AtomInfo = AtomInfo {
    name: "Tm",
    atomic_number: 69,
    period: 6,
    group: 0,
    density: 9.321,
    melt: Some(1818.0),
    boil: Some(2223.0),
    electronegativity: Some(1.25),
    number_of_neutrons: 100,
    mass: 168.93422,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_yb: AtomInfo = AtomInfo {
    name: "Yb",
    atomic_number: 70,
    period: 6,
    group: 0,
    density: 6.965,
    melt: Some(1097.0),
    boil: Some(1469.0),
    electronegativity: Some(1.1),
    number_of_neutrons: 104,
    mass: 173.045,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_lu: AtomInfo = AtomInfo {
    name: "Lu",
    atomic_number: 71,
    period: 6,
    group: 0,
    density: 9.84,
    melt: Some(1925.0),
    boil: Some(3675.0),
    electronegativity: Some(1.27),
    number_of_neutrons: 104,
    mass: 174.9668,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Lanthanide,
};

pub const atom_hf: AtomInfo = AtomInfo {
    name: "Hf",
    atomic_number: 72,
    group: 4,
    period: 6,
    density: 13.31,
    melt: Some(2506.0),
    boil: Some(4876.0),
    electronegativity: Some(1.3),
    number_of_neutrons: 108,
    mass: 178.49,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ta: AtomInfo = AtomInfo {
    name: "Ta",
    atomic_number: 73,
    group: 5,
    period: 6,
    density: 16.654,
    melt: Some(3290.0),
    boil: Some(5731.0),
    electronegativity: Some(1.5),
    number_of_neutrons: 108,
    mass: 180.94788,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_w: AtomInfo = AtomInfo {
    name: "W",
    atomic_number: 74,
    group: 6,
    period: 6,
    density: 19.25,
    melt: Some(3695.0),
    boil: Some(5828.0),
    electronegativity: Some(2.36),
    number_of_neutrons: 110,
    mass: 183.84,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_re: AtomInfo = AtomInfo {
    name: "Re",
    atomic_number: 75,
    group: 7,
    period: 6,
    density: 21.02,
    melt: Some(3459.0),
    boil: Some(5869.0),
    electronegativity: Some(1.9),
    number_of_neutrons: 110,
    mass: 186.207,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_os: AtomInfo = AtomInfo {
    name: "Os",
    atomic_number: 76,
    group: 8,
    period: 6,
    density: 22.61,
    melt: Some(3306.0),
    boil: Some(5285.0),
    electronegativity: Some(2.2),
    number_of_neutrons: 116,
    mass: 190.23,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ir: AtomInfo = AtomInfo {
    name: "Ir",
    atomic_number: 77,
    group: 9,
    period: 6,
    density: 22.56,
    melt: Some(2719.0),
    boil: Some(4701.0),
    electronegativity: Some(2.2),
    number_of_neutrons: 116,
    mass: 192.217,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_pt: AtomInfo = AtomInfo {
    name: "Pt",
    atomic_number: 78,
    group: 10,
    period: 6,
    density: 21.46,
    melt: Some(2041.4),
    boil: Some(4098.0),
    electronegativity: Some(2.28),
    number_of_neutrons: 117,
    mass: 195.084,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_au: AtomInfo = AtomInfo {
    name: "Au",
    atomic_number: 79,
    group: 11,
    period: 6,
    density: 19.282,
    melt: Some(1337.33),
    boil: Some(3129.0),
    electronegativity: Some(2.54),
    number_of_neutrons: 118,
    mass: 196.966569,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_hg: AtomInfo = AtomInfo {
    name: "Hg",
    atomic_number: 80,
    group: 12,
    period: 6,
    density: 13.5336,
    melt: Some(234.43),
    boil: Some(629.88),
    electronegativity: Some(2.0),
    number_of_neutrons: 122,
    mass: 200.592,
    state_of_matter: StateOfMatter::Liquid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_Tl: AtomInfo = AtomInfo {
    name: "Tl",
    atomic_number: 81,
    group: 13,
    period: 6,
    density: 11.85,
    melt: Some(577.0),
    boil: Some(1746.0),
    electronegativity: Some(1.62),
    number_of_neutrons: 124,
    mass: 204.38,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_pb: AtomInfo = AtomInfo {
    name: "Pb",
    atomic_number: 82,
    group: 14,
    period: 6,
    density: 11.342,
    melt: Some(600.61),
    boil: Some(2022.0),
    electronegativity: Some(1.87),
    number_of_neutrons: 126,
    mass: 207.2,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_bi: AtomInfo = AtomInfo {
    name: "Bi",
    atomic_number: 83,
    group: 15,
    period: 6,
    density: 9.807,
    melt: Some(544.7),
    boil: Some(1837.0),
    electronegativity: Some(2.02),
    number_of_neutrons: 126,
    mass: 208.98040,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_po: AtomInfo = AtomInfo {
    name: "Po",
    atomic_number: 84,
    group: 16,
    period: 6,
    density: 9.32,
    melt: Some(527.0),
    boil: Some(1235.0),
    electronegativity: Some(2.0),
    number_of_neutrons: 125,
    mass: 209.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_at: AtomInfo = AtomInfo {
    name: "At",
    atomic_number: 85,
    group: 17,
    period: 6,
    density: 7.0,
    melt: Some(575.0),
    boil: Some(610.0),
    electronegativity: Some(2.2),
    number_of_neutrons: 125,
    mass: 210.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Metalloid,
};

pub const atom_rn: AtomInfo = AtomInfo {
    name: "Rn",
    atomic_number: 86,
    group: 18,
    period: 6,
    density: 0.00973,
    melt: Some(202.0),
    boil: Some(211.3),
    electronegativity: Some(2.2),
    number_of_neutrons: 136,
    mass: 222.0,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::NobleGas,
};

pub const atom_fr: AtomInfo = AtomInfo {
    name: "Fr",
    atomic_number: 87,
    group: 1,
    period: 7,
    density: 1.87,
    melt: Some(300.0),
    boil: Some(950.0),
    electronegativity: Some(0.7),
    number_of_neutrons: 136,
    mass: 223.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_ra: AtomInfo = AtomInfo {
    name: "Ra",
    atomic_number: 88,
    period: 7,
    density: 5.5,
    melt: Some(973.0),
    boil: Some(2010.0),
    electronegativity: Some(0.9),
    group: 2,
    number_of_neutrons: 138,
    mass: 226.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::AlkaliMetal,
};

pub const atom_ac: AtomInfo = AtomInfo {
    name: "Ac",
    atomic_number: 89,
    period: 7,
    density: 10.07,
    melt: Some(1323.0),
    boil: Some(3471.0),
    electronegativity: Some(1.1),
    group: 3,
    number_of_neutrons: 138,
    mass: 227.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_th: AtomInfo = AtomInfo {
    name: "Th",
    atomic_number: 90,
    period: 7,
    group: 0,
    density: 11.72,
    melt: Some(2115.0),
    boil: Some(5061.0),
    electronegativity: Some(1.3),
    number_of_neutrons: 142,
    mass: 232.0377,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_pa: AtomInfo = AtomInfo {
    name: "Pa",
    atomic_number: 91,
    period: 7,
    group: 0,
    density: 15.37,
    melt: Some(1841.0),
    boil: Some(4300.0),
    electronegativity: Some(1.5),
    number_of_neutrons: 140,
    mass: 231.036,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_u: AtomInfo = AtomInfo {
    name: "U",
    atomic_number: 92,
    period: 7,
    group: 0,
    density: 18.95,
    melt: Some(1405.3),
    boil: Some(4404.0),
    electronegativity: Some(1.38),
    number_of_neutrons: 147,
    mass: 238.02891,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_np: AtomInfo = AtomInfo {
    name: "Np",
    atomic_number: 93,
    period: 7,
    group: 0,
    density: 20.45,
    melt: Some(917.0),
    boil: Some(4273.0),
    electronegativity: Some(1.36),
    number_of_neutrons: 144,
    mass: 237.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_pu: AtomInfo = AtomInfo {
    name: "Pu",
    atomic_number: 94,
    period: 7,
    group: 0,
    density: 19.84,
    melt: Some(912.5),
    boil: Some(3501.0),
    electronegativity: Some(1.28),
    number_of_neutrons: 150,
    mass: 244.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_am: AtomInfo = AtomInfo {
    name: "Am",
    atomic_number: 95,
    period: 7,
    group: 0,
    density: 13.69,
    melt: Some(1449.0),
    boil: Some(2880.0),
    electronegativity: Some(1.13),
    number_of_neutrons: 148,
    mass: 243.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_cm: AtomInfo = AtomInfo {
    name: "Cm",
    atomic_number: 96,
    period: 7,
    group: 0,
    density: 13.51,
    melt: Some(1613.0),
    boil: Some(3383.0),
    electronegativity: Some(1.28),
    number_of_neutrons: 151,
    mass: 247.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_bk: AtomInfo = AtomInfo {
    name: "Bk",
    atomic_number: 97,
    period: 7,
    group: 0,
    density: 14.79,
    melt: Some(1259.0),
    boil: Some(2900.0),
    electronegativity: Some(1.3),
    number_of_neutrons: 150,
    mass: 247.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_cf: AtomInfo = AtomInfo {
    name: "Cf",
    atomic_number: 98,
    period: 7,
    group: 0,
    density: 15.1,
    melt: Some(1173.0),
    boil: Some(1743.0),
    electronegativity: Some(1.3),
    number_of_neutrons: 153,
    mass: 251.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_es: AtomInfo = AtomInfo {
    name: "Es",
    atomic_number: 99,
    period: 7,
    group: 0,
    density: 8.84,
    melt: Some(1133.0),
    boil: Some(1269.0),
    electronegativity: Some(1.3),
    number_of_neutrons: 153,
    mass: 252.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_fm: AtomInfo = AtomInfo {
    name: "Fm",
    atomic_number: 100,
    period: 7,
    group: 0,
    density: 9.7,
    melt: Some(1125.0),
    boil: None,
    electronegativity: Some(1.3),
    number_of_neutrons: 157,
    mass: 257.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_md: AtomInfo = AtomInfo {
    name: "Md",
    atomic_number: 101,
    period: 7,
    group: 0,
    density: 10.3,
    melt: Some(1100.0),
    boil: None,
    electronegativity: Some(1.3),
    number_of_neutrons: 157,
    mass: 258.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_no: AtomInfo = AtomInfo {
    name: "No",
    atomic_number: 102,
    period: 7,
    group: 0,
    density: 9.9,
    melt: Some(1100.0),
    boil: None,
    electronegativity: Some(1.3),
    number_of_neutrons: 157,
    mass: 259.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_lr: AtomInfo = AtomInfo {
    name: "Lr",
    atomic_number: 103,
    period: 7,
    group: 0,
    density: 15.6,
    melt: Some(1900.0),
    boil: None,
    electronegativity: Some(1.3),
    number_of_neutrons: 159,
    mass: 262.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Actinide,
};

pub const atom_rf: AtomInfo = AtomInfo {
    name: "Rf",
    atomic_number: 104,
    period: 7,
    density: 23.2,
    melt: Some(2400.0),
    boil: Some(5800.0),
    electronegativity: None,
    group: 4,
    number_of_neutrons: 157,
    mass: 261.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_db: AtomInfo = AtomInfo {
    name: "Db",
    atomic_number: 105,
    period: 7,
    density: 29.3,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 5,
    number_of_neutrons: 157,
    mass: 262.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_sg: AtomInfo = AtomInfo {
    name: "Sg",
    atomic_number: 106,
    period: 7,
    density: 35.0,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 6,
    number_of_neutrons: 157,
    mass: 263.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_bh: AtomInfo = AtomInfo {
    name: "Bh",
    atomic_number: 107,
    period: 7,
    density: 37.1,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 7,
    number_of_neutrons: 155,
    mass: 155.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_hs: AtomInfo = AtomInfo {
    name: "Hs",
    atomic_number: 108,
    period: 7,
    density: 40.7,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 8,
    number_of_neutrons: 157,
    mass: 265.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_mt: AtomInfo = AtomInfo {
    name: "Mt",
    atomic_number: 109,
    period: 7,
    density: 37.4,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 9,
    number_of_neutrons: 157,
    mass: 266.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_ds: AtomInfo = AtomInfo {
    name: "Ds",
    atomic_number: 110,
    period: 7,
    density: 34.8,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 10,
    number_of_neutrons: 151,
    mass: 281.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_rg: AtomInfo = AtomInfo {
    name: "Rg",
    atomic_number: 111,
    period: 7,
    density: 28.7,
    melt: None,
    boil: None,
    electronegativity: None,
    group: 11,
    number_of_neutrons: 161,
    mass: 280.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_cn: AtomInfo = AtomInfo {
    name: "Cn",
    atomic_number: 112,
    period: 7,
    density: 23.7,
    melt: None,
    boil: Some(357.0),
    electronegativity: None,
    group: 12,
    number_of_neutrons: 173,
    mass: 285.0,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::TransitionMetal,
};

pub const atom_nh: AtomInfo = AtomInfo {
    name: "Nh",
    atomic_number: 113,
    period: 7,
    density: 16.0,
    melt: Some(700.0),
    boil: Some(1400.0),
    electronegativity: None,
    group: 13,
    number_of_neutrons: 173,
    mass: 113.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_fl: AtomInfo = AtomInfo {
    name: "Fl",
    atomic_number: 114,
    period: 7,
    density: 14.0,
    melt: Some(340.0),
    boil: Some(420.0),
    electronegativity: None,
    group: 14,
    number_of_neutrons: 175,
    mass: 289.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::PostTransitionMetal,
};

pub const atom_mc: AtomInfo = AtomInfo {
    name: "Mc",
    atomic_number: 115,
    period: 7,
    density: 13.5,
    melt: Some(700.0),
    boil: Some(1400.0),
    electronegativity: None,
    group: 15,
    number_of_neutrons: 174,
    mass: 290.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_lv: AtomInfo = AtomInfo {
    name: "Lv",
    atomic_number: 116,
    period: 7,
    density: 12.9,
    melt: Some(709.0),
    boil: Some(1085.0),
    electronegativity: None,
    group: 16,
    number_of_neutrons: 177,
    mass: 293.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_ts: AtomInfo = AtomInfo {
    name: "Ts",
    atomic_number: 117,
    period: 7,
    density: 7.2,
    melt: Some(723.0),
    boil: Some(883.0),
    electronegativity: None,
    group: 17,
    number_of_neutrons: 177,
    mass: 294.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

pub const atom_og: AtomInfo = AtomInfo {
    name: "Og",
    atomic_number: 118,
    period: 7,
    density: 5.0,
    melt: Some(258.0),
    boil: Some(263.0),
    electronegativity: None,
    group: 18,
    number_of_neutrons: 176,
    mass: 294.0,
    state_of_matter: StateOfMatter::Solid,
    sub_category: SubCategory::Unknown,
};

