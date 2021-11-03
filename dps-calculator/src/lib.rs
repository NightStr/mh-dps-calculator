use std::fmt;
use std::fmt::Display;
use std::convert::TryFrom;

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


impl TryFrom<u16> for Sharpness {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Red),
            2 => Ok(Self::Orange),
            3 => Ok(Self::Yellow),
            4 => Ok(Self::Green),
            5 => Ok(Self::Blue),
            6 => Ok(Self::White),
            7 => Ok(Self::Purple),
            _ => Err("Неверное значение для заточки")
        }
    }
}

pub enum ulnerability {
    OneStar,
    TwoStar,
    ThreeStar,
    Custom(f32),
}

impl TryFrom<u16> for ulnerability {
    type Error = & 'static str;
    fn try_from(v: u16) -> Result< Self, Self::Error> {
        match v {
            1 => Ok(ulnerability::OneStar),
            2 => Ok(ulnerability::TwoStar),
            3 => Ok(ulnerability::ThreeStar),
            v => {
                match format ! ("0.{}", v).parse::< f32 > () {
                    Ok(x) => Ok(ulnerability::Custom(x)),
                    Err(_) => {
                        Err("Некорректное значение. Попробуйте еще раз")
                    }
                }
            }
        }
    }
}

pub struct RawVulnerability {
    pub vulnerability: ulnerability
}

impl RawVulnerability {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.vulnerability {
            ulnerability::OneStar => 0.275,
            ulnerability::TwoStar => 0.6,
            ulnerability::ThreeStar => 0.7,
            ulnerability::Custom(v) => *v,
        }
    }
}

impl From<ulnerability> for RawVulnerability {
    fn from(resistance: ulnerability) -> Self {
        Self{ vulnerability: resistance }
    }
}

impl From<f32> for RawVulnerability {
    fn from(v: f32) -> Self {
        Self{ vulnerability: ulnerability::Custom(v)}
    }
}

impl TryFrom<u16> for RawVulnerability {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match ulnerability::try_from(v) {
            Ok(resistance) => Ok(Self{ vulnerability: resistance }),
            Err(e) => Err(e)
        }
    }
}

pub struct ElementalVulnerability {
    pub resistance: ulnerability
}

impl ElementalVulnerability {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.resistance {
            ulnerability::OneStar => 0.125,
            ulnerability::TwoStar => 0.175,
            ulnerability::ThreeStar => 0.275,
            ulnerability::Custom(v) => *v,
        }
    }
}


impl From<ulnerability> for ElementalVulnerability {
    fn from(resistance: ulnerability) -> Self {
        Self{resistance}
    }
}

impl From<f32> for ElementalVulnerability {
    fn from(v: f32) -> Self {
        Self{resistance: ulnerability::Custom(v)}
    }
}

impl TryFrom<u16> for ElementalVulnerability {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match ulnerability::try_from(v) {
            Ok(resistance) => Ok(Self{resistance}),
            Err(e) => Err(e)
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
    weapon_damage: u16, sharpness: Sharpness, affinity: u16, affinity_size: u16,
    raw_vulnerability: RawVulnerability, elemental_damage: u16,
    elemental_vulnerability: ElementalVulnerability
) -> CalculatedDamage {
    let raw: u16 = (f32::from(weapon_damage) * sharpness.get_raw_multiplicity()
            * raw_vulnerability.get_multiplicity()) as u16;
    CalculatedDamage {
        raw: raw + (raw /100*affinity_size)/100*affinity,
        elemental: (f32::from(elemental_damage / 10) * sharpness.get_elemental_multiplicity()
            * elemental_vulnerability.get_multiplicity()) as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate, CalculatedDamage, ElementalVulnerability, RawVulnerability, ulnerability, Sharpness};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(
            100,
            Sharpness::White,
            0,
            25,
            ulnerability::ThreeStar.into(),
            100,
            ulnerability::ThreeStar.into()
        ), CalculatedDamage{raw: 92, elemental: 3});
    }
}
