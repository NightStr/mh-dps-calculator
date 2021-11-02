fn main() {
    let damage = dps_calculator::calculate(
        100,
        dps_calculator::Sharpness::White,
        0,
        25,
        dps_calculator::RawVulnerability {resistance: dps_calculator::Resistance::ThreeStar},
        100,
        dps_calculator::ElementalVulnerability {resistance: dps_calculator::Resistance::ThreeStar},
    );
    println!("{}", damage);
}
