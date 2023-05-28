use crate::{FromString, GetVariants};

/// The [Tier] enum represents the grade given to a cock.
#[derive(Debug, PartialEq, Clone)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

/// The [GetVariants] trait implementation for [Tier] returns a vector of the possible variants of [Tier].
impl GetVariants for Tier {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("S"),
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
            String::from("E"),
            String::from("F"),
        ]
    }
}

/// The [FromString] trait implementation for [Tier] returns a variant of [Tier].
impl FromString for Tier {
    fn from_string(tier: &str) -> Tier {
        match tier {
            "S" => Tier::S,
            "A" => Tier::A,
            "B" => Tier::B,
            "C" => Tier::C,
            "D" => Tier::D,
            "E" => Tier::E,
            "F" => Tier::F,
            _ => panic!("Invalid tier"),
        }
    }
}

/// The [std::fmt::Display] trait implementation for [Tier] returns a string representation of the [Tier] variant.
impl std::fmt::Display for Tier {
    /// Returns a string representation of the [Tier] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::S => write!(f, "S"),
            Tier::A => write!(f, "A"),
            Tier::B => write!(f, "B"),
            Tier::C => write!(f, "C"),
            Tier::D => write!(f, "D"),
            Tier::E => write!(f, "E"),
            Tier::F => write!(f, "F"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier() {
        let s = Tier::S;
        let a = Tier::A;
        let b = Tier::B;
        let c = Tier::C;
        let d = Tier::D;
        let e = Tier::E;
        let f = Tier::F;

        assert_eq!(s, Tier::S);
        assert_eq!(a, Tier::A);
        assert_eq!(b, Tier::B);
        assert_eq!(c, Tier::C);
        assert_eq!(d, Tier::D);
        assert_eq!(e, Tier::E);
        assert_eq!(f, Tier::F);
    }
}
