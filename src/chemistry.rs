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
    number_of_neutrons: u32,
    mass: f64, // https://en.wikipedia.org/wiki/Relative_atomic_mass
    state_of_matter: StateOfMatter,
    sub_category: SubCategory,
    // A lot of more stuff to be added...
}

pub const atom_h: AtomInfo = AtomInfo {
    name: "H",
    atomic_number: 1,
    number_of_neutrons: 0,
    mass: 1.008,
    state_of_matter: StateOfMatter::Gas,
    sub_category: SubCategory::DiatomicNonMetal,
};
