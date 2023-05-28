use crate::{FromString, GetVariants, Score};

/// /// The [Aesthetic] enum represents different aesthetic values of a cock.
#[derive(Debug, PartialEq, Clone)]
pub enum Aesthetic {
    JustChopItOff,
    TooBigToBeFunctional,
    Gross,
    UglyButUsable,
    KindaMediocre,
    Normal,
    ABitBetterThanNormal,
    NiceOne,
    Cockalicious,
    Perfect,
}

/// The [Score] trait implementation for [Aesthetic] provides a score value based on the aesthetic.
impl Score for Aesthetic {
    fn score(&self) -> u32 {
        match self {
            Aesthetic::JustChopItOff => 1,
            Aesthetic::TooBigToBeFunctional => 2,
            Aesthetic::Gross => 3,
            Aesthetic::UglyButUsable => 4,
            Aesthetic::KindaMediocre => 5,
            Aesthetic::Normal => 6,
            Aesthetic::ABitBetterThanNormal => 7,
            Aesthetic::NiceOne => 8,
            Aesthetic::Cockalicious => 9,
            Aesthetic::Perfect => 10,
        }
    }
}

/// The [GetVariants] trait implementation for [Aesthetic] returns a vector of the possible variants of [Aesthetic].
impl GetVariants for Aesthetic {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("JustChopItOff"),
            String::from("TooBigToBeFunctional"),
            String::from("Gross"),
            String::from("UglyButUsable"),
            String::from("KindaMediocre"),
            String::from("Normal"),
            String::from("ABitBetterThanNormal"),
            String::from("NiceOne"),
            String::from("Cockalicious"),
            String::from("Perfect"),
        ]
    }
}

/// The [FromString] trait implementation for [Aesthetic] returns an [Aesthetic] variant based on the string provided.
impl FromString for Aesthetic {
    fn from_string(aesthetic: &str) -> Aesthetic {
        match aesthetic {
            "JustChopItOff" => Aesthetic::JustChopItOff,
            "TooBigToBeFunctional" => Aesthetic::TooBigToBeFunctional,
            "Gross" => Aesthetic::Gross,
            "UglyButUsable" => Aesthetic::UglyButUsable,
            "KindaMediocre" => Aesthetic::KindaMediocre,
            "Normal" => Aesthetic::Normal,
            "ABitBetterThanNormal" => Aesthetic::ABitBetterThanNormal,
            "NiceOne" => Aesthetic::NiceOne,
            "Cockalicious" => Aesthetic::Cockalicious,
            "Perfect" => Aesthetic::Perfect,
            _ => panic!("Invalid aesthetic"),
        }
    }
}

/// [std::fmt::Display] implementation for [Aesthetic].
impl std::fmt::Display for Aesthetic {
    /// Returns a string representation of the [Aesthetic] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aesthetic::JustChopItOff => write!(f, "Just chop it off"),
            Aesthetic::TooBigToBeFunctional => write!(f, "Too big to be functional"),
            Aesthetic::Gross => write!(f, "Gross"),
            Aesthetic::UglyButUsable => write!(f, "Ugly but usable"),
            Aesthetic::KindaMediocre => write!(f, "Kinda mediocre"),
            Aesthetic::Normal => write!(f, "Normal"),
            Aesthetic::ABitBetterThanNormal => write!(f, "A bit better than normal"),
            Aesthetic::NiceOne => write!(f, "Nice one"),
            Aesthetic::Cockalicious => write!(f, "Cockalicious"),
            Aesthetic::Perfect => write!(f, "Perfect"),
        }
    }
}

/// Unit tests for [Aesthetic].
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aesthetic_score() {
        assert_eq!(Aesthetic::JustChopItOff.score(), 1);
        assert_eq!(Aesthetic::TooBigToBeFunctional.score(), 2);
        assert_eq!(Aesthetic::Gross.score(), 3);
        assert_eq!(Aesthetic::UglyButUsable.score(), 4);
        assert_eq!(Aesthetic::KindaMediocre.score(), 5);
        assert_eq!(Aesthetic::Normal.score(), 6);
        assert_eq!(Aesthetic::ABitBetterThanNormal.score(), 7);
        assert_eq!(Aesthetic::NiceOne.score(), 8);
        assert_eq!(Aesthetic::Cockalicious.score(), 9);
        assert_eq!(Aesthetic::Perfect.score(), 10);
    }
}
