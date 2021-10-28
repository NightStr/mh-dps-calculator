use rust_decimal;

pub enum Sharpness {
    Red,
    Yellow,
    Green,
    Blue,
    White,
    Purple,
}

impl Sharpness {
    pub fn get_multiplicity(&self) -> rust_decimal::Decimal {
        match self {
            _ => rust_decimal::Decimal::new(1, 0)
        }
    }
    pub fn get_elemental_multiplicity(&self) -> rust_decimal::Decimal {
        match self {
            _ => rust_decimal::Decimal::new(1, 0)
        }
    }
}

pub enum ElementalResistance {
    OneStar,
    TwoStar,
    ThreeStar,
}

impl ElementalResistance {
    pub fn get_multiplicity(&self) -> rust_decimal::Decimal {
        match self {
            _ => rust_decimal::Decimal::new(1, 0)
        }
    }
}

pub fn calculate(
    weapon_damage: i32, sharpness: Sharpness, elemental_damage: i32,
    elemental_resist: ElementalResistance
) -> rust_decimal::Decimal {
    (
        rust_decimal::Decimal::new(weapon_damage.into(), 0) * sharpness.get_multiplicity()
    ) + (
        rust_decimal::Decimal::new(elemental_damage.into(), 0) * sharpness.get_elemental_multiplicity() * elemental_resist.get_multiplicity()
    )
}

#[cfg(test)]
mod tests {
    use crate::{calculate, ElementalResistance, Sharpness};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(
            100, Sharpness::White, 100,
            ElementalResistance::OneStar
        ).to_string(), "200");
    }
}
