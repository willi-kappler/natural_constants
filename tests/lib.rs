#[test]
fn test_math() {
    use natural_constants::math::*;

    println!("{}", golden_ratio)
}

#[test]
fn test_conversion() {
    use natural_constants::conversion::*;

    let mass_in_g = 50_000.0; // 50 kg
    assert_eq!(to_kilo(mass_in_g), 50.0);
    assert_eq!(from_kilo(50.0), mass_in_g);

    let energy_in_j = 125_000_000.0; // 125 MJoule
    assert_eq!(to_mega(energy_in_j), 125.0);
    assert_eq!(from_mega(125.0), energy_in_j);

    let file_size_in_bytes1 = 73_000_000_000.0; // 74 gigabytes (not gibibytes)
    assert_eq!(to_giga(file_size_in_bytes1), 73.0);
    assert_eq!(from_giga(73.0), file_size_in_bytes1);

    let file_size_in_bytes2 = 312_000_000_000_000.0; // 312 terabytes (not tebibytes)
    assert_eq!(to_tera(file_size_in_bytes2), 312.0);
    assert_eq!(from_tera(312.0), file_size_in_bytes2);

    let force_in_kn = 780.23; // kilo newton
    assert_eq!(kilo_to_mega(force_in_kn), 0.78023);
    assert_eq!(mega_to_kilo(0.78023), force_in_kn);

    let temp_in_kelvin = 1.0;
    assert_eq!(kelvin_to_deg_cel(temp_in_kelvin), -272.15);
    assert_eq!(deg_cel_to_kelvin(-272.15), temp_in_kelvin);
}

#[test]
fn limits() {
    use natural_constants::math::*;
    assert_eq!(golden_ratio, (1.0 + 5.0_f64.sqrt())/2.0);
}
