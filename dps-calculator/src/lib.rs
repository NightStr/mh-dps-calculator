use std::fmt;
use std::fmt::Display;
use rust_decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

#[derive(Debug)]
pub enum Sharpness {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    White,
    Purple,
}

impl Sharpness {
    pub fn get_raw_multiplicity(&self) -> rust_decimal::Decimal {
        match self {
            Self::Red => dec!(0.50),
            Self::Orange => dec!(0.75),
            Self::Yellow => dec!(1.00),
            Self::Green => dec!(1.05),
            Self::Blue => dec!(1.20),
            Self::White => dec!(1.32),
            Self::Purple => dec!(1.39)
        }
    }
    pub fn get_elemental_multiplicity(&self) -> rust_decimal::Decimal {
        match self {
            Self::Red => dec!(0.25),
            Self::Orange => dec!(0.50),
            Self::Yellow => dec!(0.75),
            Self::Green => dec!(1.00),
            Self::Blue => dec!(1.0625),
            Self::White => dec!(1.15),
            Self::Purple => dec!(1.25)
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
            Self::OneStar => dec!(0.3),
            Self::TwoStar => dec!(0.9),
            Self::ThreeStar => dec!(1),
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct CalculatedDamage {
    pub raw: i32,
    pub elemental: i32
}

impl Display for CalculatedDamage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raw: {}, elemental: {}, overall: {}", self.raw, self.elemental,
            self.raw + self.elemental
        )
    }
}

pub fn calculate(
    weapon_damage: i32, sharpness: Sharpness, elemental_damage: i32,
    elemental_resist: ElementalResistance
) -> CalculatedDamage {
    CalculatedDamage {
        raw: (
            rust_decimal::Decimal::from(weapon_damage)
                * sharpness.get_raw_multiplicity()
        ).trunc().to_i32().unwrap(),
        elemental: (
            rust_decimal::Decimal::from(elemental_damage / 10)
                * sharpness.get_elemental_multiplicity()
                * elemental_resist.get_multiplicity()
        ).trunc().to_i32().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate, CalculatedDamage, ElementalResistance, Sharpness};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(
            100, Sharpness::White, 100,
            ElementalResistance::OneStar
        ), CalculatedDamage{raw: 132, elemental: 3});
    }
}
