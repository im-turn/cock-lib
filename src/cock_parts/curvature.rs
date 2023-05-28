use crate::{FromString, GetVariants};

/// Enum representing the direction of [Curvature] for a cock.
/// This includes directions [Curvature::Straight], [Curvature::Left], [Curvature::Right], [Curvature::Upwards], [Curvature::Downwards], and [Curvature::Other].
/// There's also an [Curvature::Other] variant that can store a custom description as a string.
#[derive(Debug, PartialEq, Clone)]
pub enum Curvature {
    Straight,
    Left,
    Right,
    Upwards,
    Downwards,
    Other(String),
}

/// Implementation of the [GetVariants] trait for [Curvature]. This enables the creation
/// of a vector containing all possible variants as string values.
impl GetVariants for Curvature {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Straight"),
            String::from("Left"),
            String::from("Right"),
            String::from("Upwards"),
            String::from("Downwards"),
            String::from("Other"),
        ]
    }
}

/// Implementation of the [FromString] trait for [Curvature]. This allows a [Curvature] instance
/// to be created from a string value. The [Curvature::Other] variant involves a user prompt for a
/// custom description.
impl FromString for Curvature {
    fn from_string(curvature: &str) -> Curvature {
        match curvature {
            "Straight" => Curvature::Straight,
            "Left" => Curvature::Left,
            "Right" => Curvature::Right,
            "Upwards" => Curvature::Upwards,
            "Downwards" => Curvature::Downwards,
            "Other" => Curvature::Other("".to_string()),
            _ => panic!("Invalid curvature"),
        }
    }
}

/// Implementation of the [std::fmt::Display] trait for [Curvature]. This allows a [Curvature] instance to be
/// converted to a string for display purposes. For the [Curvature::Other] variant, the custom description
/// is displayed.
impl std::fmt::Display for Curvature {
    /// Returns the string description of a [Curvature] instance.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Curvature::Straight => write!(f, "Straight"),
            Curvature::Left => write!(f, "Left"),
            Curvature::Right => write!(f, "Right"),
            Curvature::Upwards => write!(f, "Upwards"),
            Curvature::Downwards => write!(f, "Downwards"),
            Curvature::Other(other) => write!(f, "{}", other),
        }
    }
}
/// Tests for the [Curvature] enum
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curvature() {
        let straight = Curvature::Straight;
        let left = Curvature::Left;
        let right = Curvature::Right;
        let upwards = Curvature::Upwards;
        let downwards = Curvature::Downwards;
        let other = Curvature::Other(String::from("test"));

        assert_eq!(straight, Curvature::Straight);
        assert_eq!(left, Curvature::Left);
        assert_eq!(right, Curvature::Right);
        assert_eq!(upwards, Curvature::Upwards);
        assert_eq!(downwards, Curvature::Downwards);
        assert_eq!(other, Curvature::Other(String::from("test")));
    }
}
