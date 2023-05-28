use crate::{FromString, GetVariants, Score};

/// [Balls] is an enum that represents the size of the balls.
#[derive(Debug, PartialEq, Clone)]
pub enum Balls {
    NonExistant,
    Tiny,
    Normal,
    BigSwingers,
    PossibleCancer,
}

/// The [Score] trait implementation for [Balls] provides a score value based on the balls.
impl Score for Balls {
    fn score(&self) -> u32 {
        match self {
            Balls::NonExistant => 2,
            Balls::Tiny => 3,
            Balls::Normal => 4,
            Balls::BigSwingers => 5,
            Balls::PossibleCancer => 1,
        }
    }
}

/// The [GetVariants] trait implementation for [Balls] returns a vector of the possible variants of [Balls].
impl GetVariants for Balls {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("NonExistant"),
            String::from("Tiny"),
            String::from("Normal"),
            String::from("BigSwingers"),
            String::from("PossibleCancer"),
        ]
    }
}

/// The [FromString] trait implementation for [Balls] returns an [Balls] variant based on the string provided.
impl FromString for Balls {
    fn from_string(balls: &str) -> Balls {
        match balls {
            "NonExistant" => Balls::NonExistant,
            "Tiny" => Balls::Tiny,
            "Normal" => Balls::Normal,
            "BigSwingers" => Balls::BigSwingers,
            "PossibleCancer" => Balls::PossibleCancer,
            _ => panic!("Invalid balls"),
        }
    }
}

/// The [std::fmt::Display] trait implementation for [Balls] returns a string representation of the balls.
impl std::fmt::Display for Balls {
    /// Returns a string representation of the [Balls] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Balls::NonExistant => write!(f, "NonExistant"),
            Balls::Tiny => write!(f, "Tiny"),
            Balls::Normal => write!(f, "Normal"),
            Balls::BigSwingers => write!(f, "BigSwingers"),
            Balls::PossibleCancer => write!(f, "PossibleCancer"),
        }
    }
}

/// Unit tests for the [Balls] enum.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balls_score() {
        assert_eq!(Balls::NonExistant.score(), 2);
        assert_eq!(Balls::Tiny.score(), 3);
        assert_eq!(Balls::Normal.score(), 4);
        assert_eq!(Balls::BigSwingers.score(), 5);
        assert_eq!(Balls::PossibleCancer.score(), 1);
    }
}
