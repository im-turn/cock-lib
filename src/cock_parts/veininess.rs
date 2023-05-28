use crate::{FromString, GetVariants, Score};

/// Represents the level of [Veininess] of a cock.
/// This is an enum with five possible values: [Veininess::Invisible], [Veininess::SlightPumper], [Veininess::Normal], [Veininess::HealthyPumper], and [Veininess::Juicer].
#[derive(Debug, PartialEq, Clone)]
pub enum Veininess {
    Invisible,
    SlightPumper,
    Normal,
    HealthyPumper,
    Juicer,
}

/// Implementing [Score] trait for [Veininess] enum.
/// Each [Veininess] level corresponds to a specific score.
impl Score for Veininess {
    fn score(&self) -> u32 {
        match self {
            Veininess::Invisible => 1,
            Veininess::SlightPumper => 2,
            Veininess::Normal => 3,
            Veininess::HealthyPumper => 4,
            Veininess::Juicer => 5,
        }
    }
}

/// Implementing [GetVariants] for [Veininess] enum to provide the different variant options for [Veininess].
impl GetVariants for Veininess {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Invisible"),
            String::from("SlightPumper"),
            String::from("Normal"),
            String::from("HealthyPumper"),
            String::from("Juicer"),
        ]
    }
}

/// Implementing [FromString] for [Veininess] enum to create a [Veininess] instance from a string.
impl FromString for Veininess {
    fn from_string(veininess: &str) -> Veininess {
        match veininess {
            "Invisible" => Veininess::Invisible,
            "SlightPumper" => Veininess::SlightPumper,
            "Normal" => Veininess::Normal,
            "HealthyPumper" => Veininess::HealthyPumper,
            "Juicer" => Veininess::Juicer,
            _ => panic!("Invalid veininess"),
        }
    }
}

/// Implementing display trait for [Veininess] for formatted print.
impl std::fmt::Display for Veininess {
    /// Returns a string representation of the [Veininess] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Veininess::Invisible => write!(f, "Invisible"),
            Veininess::SlightPumper => write!(f, "SlightPumper"),
            Veininess::Normal => write!(f, "Normal"),
            Veininess::HealthyPumper => write!(f, "HealthyPumper"),
            Veininess::Juicer => write!(f, "Juicer"),
        }
    }
}

/// Tests for the [Veininess] enum
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veininess_score() {
        assert_eq!(Veininess::Invisible.score(), 1);
        assert_eq!(Veininess::SlightPumper.score(), 2);
        assert_eq!(Veininess::Normal.score(), 3);
        assert_eq!(Veininess::HealthyPumper.score(), 4);
        assert_eq!(Veininess::Juicer.score(), 5);
    }
}
