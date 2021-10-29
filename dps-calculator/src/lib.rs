use std::fmt;
use std::fmt::Display;

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
    pub fn get_raw_multiplicity(&self) -> f32 {
        match self {
            Self::Red => 0.50,
            Self::Orange => 0.75,
            Self::Yellow => 1.00,
            Self::Green => 1.05,
            Self::Blue => 1.20,
            Self::White => 1.32,
            Self::Purple => 1.39
        }
    }
    pub fn get_elemental_multiplicity(&self) -> f32 {
        match self {
            Self::Red => 0.25,
            Self::Orange => 0.50,
            Self::Yellow => 0.75,
            Self::Green => 1.00,
            Self::Blue => 1.0625,
            Self::White => 1.15,
            Self::Purple => 1.25
        }
    }
}

pub enum Resistance {
    OneStar,
    TwoStar,
    ThreeStar,
    Custom(f32),
}

pub struct RawResistance {
    resistance: Resistance
}

impl RawResistance {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.resistance {
            Resistance::OneStar => 0.275,
            Resistance::TwoStar => 0.6,
            Resistance::ThreeStar => 0.7,
            Resistance::Custom(v) => *v,
        }
    }
}

pub struct ElementalResistance  {
    resistance: Resistance
}

impl ElementalResistance {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.resistance {
            Resistance::OneStar => 0.125,
            Resistance::TwoStar => 0.175,
            Resistance::ThreeStar => 0.275,
            Resistance::Custom(v) => *v,
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct CalculatedDamage {
    pub raw: u16,
    pub elemental: u16
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
    weapon_damage: u16, sharpness: Sharpness, raw_resist: RawResistance, elemental_damage: u16,
    elemental_resist: ElementalResistance
) -> CalculatedDamage {
    CalculatedDamage {
        raw: (f32::from(weapon_damage) * sharpness.get_raw_multiplicity()
            * raw_resist.get_multiplicity()) as u16,
        elemental: (f32::from(elemental_damage / 10) * sharpness.get_elemental_multiplicity()
            * elemental_resist.get_multiplicity()) as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate, CalculatedDamage, ElementalResistance, RawResistance, Resistance, Sharpness};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(
            100,
            Sharpness::White,
            RawResistance{resistance: Resistance::OneStar},
            100,
            ElementalResistance{resistance: Resistance::OneStar}
        ), CalculatedDamage{raw: 132, elemental: 3});
    }
}
