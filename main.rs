fn main() {
    println!("{}", dps_calculator::calculate(
        100, dps_calculator::Sharpness::White, 100,
        dps_calculator::ElementalResistance::OneStar
    ));
}
