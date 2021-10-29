fn main() {
    let damage = dps_calculator::calculate(
        100,
        dps_calculator::Sharpness::White,
        100,
        dps_calculator::ElementalResistance::ThreeStar
    );
    println!("{}", damage);
}
