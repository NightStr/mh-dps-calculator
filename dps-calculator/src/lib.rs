use std::fmt;
use std::fmt::Display;
use std::convert::TryFrom;
use std::slice::Iter;

#[derive(Debug, )]
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

pub enum Vulnerability {
    OneStar,
    TwoStar,
    ThreeStar,
    Custom(f32),
}

impl TryFrom<u16> for Vulnerability {
    type Error = & 'static str;
    fn try_from(v: u16) -> Result< Self, Self::Error> {
        match v {
            1 => Ok(Vulnerability::OneStar),
            2 => Ok(Vulnerability::TwoStar),
            3 => Ok(Vulnerability::ThreeStar),
            v => {
                match format ! ("0.{}", v).parse::< f32 > () {
                    Ok(x) => Ok(Vulnerability::Custom(x)),
                    Err(_) => {
                        Err("Некорректное значение. Попробуйте еще раз")
                    }
                }
            }
        }
    }
}

pub struct RawVulnerability {
    pub vulnerability: Vulnerability
}

impl RawVulnerability {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.vulnerability {
            Vulnerability::OneStar => 0.275,
            Vulnerability::TwoStar => 0.6,
            Vulnerability::ThreeStar => 0.7,
            Vulnerability::Custom(v) => *v,
        }
    }
}

impl From<Vulnerability> for RawVulnerability {
    fn from(resistance: Vulnerability) -> Self {
        Self{ vulnerability: resistance }
    }
}

impl From<f32> for RawVulnerability {
    fn from(v: f32) -> Self {
        Self{ vulnerability: Vulnerability::Custom(v)}
    }
}

impl TryFrom<u16> for RawVulnerability {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match Vulnerability::try_from(v) {
            Ok(resistance) => Ok(Self{ vulnerability: resistance }),
            Err(e) => Err(e)
        }
    }
}

pub struct ElementalVulnerability {
    pub resistance: Vulnerability
}

impl ElementalVulnerability {
    pub fn get_multiplicity(&self) -> f32 {
        match &self.resistance {
            Vulnerability::OneStar => 0.125,
            Vulnerability::TwoStar => 0.175,
            Vulnerability::ThreeStar => 0.275,
            Vulnerability::Custom(v) => *v,
        }
    }
}


impl From<Vulnerability> for ElementalVulnerability {
    fn from(resistance: Vulnerability) -> Self {
        Self{resistance}
    }
}

impl From<f32> for ElementalVulnerability {
    fn from(v: f32) -> Self {
        Self{resistance: Vulnerability::Custom(v)}
    }
}

impl TryFrom<u16> for ElementalVulnerability {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match Vulnerability::try_from(v) {
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
    use crate::{calculate, CalculatedDamage, ElementalVulnerability, RawVulnerability, Vulnerability, Sharpness};

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(
            100,
            Sharpness::White,
            0,
            25,
            Vulnerability::ThreeStar.into(),
            100,
            Vulnerability::ThreeStar.into()
        ), CalculatedDamage{raw: 92, elemental: 3});
    }
}
