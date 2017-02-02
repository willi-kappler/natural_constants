//! Geosciences related constants

// taken from http://hpiers.obspm.fr/eop-pc/models/constants.html

// Earth's rotation constants
pub const mean_angular_velocity_earth: f64 = 7.29211501; // [10^(-5)rad/s]
pub const nominal_angular_velocity_earth: f64 = 7.2921151467064	; // [10^(-5)rad/s]
pub const conventional_duration_mean_solar_day: f64 = 86400.0; // [s]
pub const ratio_mean_solar_day_sidereal_day: f64 = 1.002737909350795; // []
pub const conventional_duration_sidereal_day: f64 = 86164.09053083288; // [s]
pub const ratio_mean_solar_day_stellar_day: f64 = 1.00273781191135448; // []
pub const stellar_day: f64 = 86164.098903691; // [s]
pub const general_precession_in_longitude: f64 = 5028.7922; // [" per century]
pub const chandler_frequency_terrestrial_frame: f64 = 0.843330; // [cycle per tropical year]
pub const chandler_period_terrestrial_frame: f64 = 433.1; // [mean solar day D]
pub const quality_factor_chandler_peak: f64 = 170; // []
pub const free_core_nutation_celestial_frame: f64 = 430.23; // [mean solar day D]
pub const quality_factor_free_core_nutation: f64 = 20000; // []
pub const sidereal_year: f64 = 365.256363004; // [mean solar day D=86400s]
pub const tropical_year: f64 = 365.242190402; // [mean solar day D=86400s]
pub const mean_motion_moon: f64 = 0.2299708345219431; // [rad/mean solar day D=86400s]

// Geodetic constants
pub const earth_equatorial_radius: f64 = 6378136.61; // [m]
pub const first_equatorial_unit_inertia: f64 = 8.01012; // [10^(37) kg m²]
pub const second_equatorial_unit_inertia: f64 = 8.01032; // [10^(37 kg m²]
pub const mean_equatorial_unit_inertia: f64 = 8.010171184; // []
pub const axial_moment_inertia: f64 = 8.03652; // [10^(37 kg m²]
pub const Longitude_principal_inertia_axis_a: f64 = -14.929110; // [°]
pub const Colatitude_principal_inertia_axis_a: f64 = 0.0000378848; // [°]
pub const first_equatorial_moment_inertia_core: f64 = 9.1152379; // [10^(36 kg m²]
pub const second_equatorial_moment_inertia_core: f64 = 9.1153997; // [10^(36 kg m²]
pub const axial_moment_inertia_core: f64 = 9.1393530; // [10^(36 kg m²]
pub const first_equatorial_moment_inertia_mantle: f64 = 7.0165; // [10^(37 kg m²]
pub const second_equatorial_moment_inertia_core: f64 = 7.09872; // []
pub const dynamical_ellipticity: f64 = 3.28454791e-3; // []
pub const dynamical_ellipticity_core: f64 = 2.6462e-3; // []
pub const second_degree_term_earth_gravity_potential: f64 = 1.08263591e-3; // []
pub const love_number: f64 = 0.3; // []
pub const secular_love_number: f64 = 0.9383; // []

// Gravitational constants
pub const mean_equatorial_gravity: f64 = 9.780327810; // [m s^(-2)]
pub const gravitational_constant: f64 = 6.6738480; // [10^(-11) m³ kg^(-1) s^(-2)]
pub const geocentric_constant_gravitation: f64 = 3.9860044188; // [10^(14) m³ s^(-2)]
pub const heliocentric_constant_gravitation: f64 = 1.3271244207650; // [10^(14) m³ s^(-2)]

// Cosmologial constants
pub const hubble_constant: f64 = 733; // [km s^(-1) Mpc^(-1)]
pub const hubble_length: f64 = 1.275e26; // [m]
pub const age_universe: f64 = 13.7315; // [Giga year]
