use crate::{FromString, GetVariants};

/// Enum representing the [Shape] for a cock.
/// The shapes include [Shape::Cylindrical], [Shape::Tapered] and an [Shape::Other] variant that can store a custom [Shape] description as a string.
#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Cylindrical,
    Tapered,
    Other(String),
}

impl Shape {
    /// Returns the string description of a [Shape] instance.
    pub fn get_shape(&self) -> &str {
        match self {
            Shape::Cylindrical => "Cylindrical",
            Shape::Tapered => "Tapered",
            Shape::Other(other) => other,
        }
    }
}

/// Implementation of the [FromString] trait for [Shape]. This allows a [Shape] instance to be created from a string value.
/// The [Shape::Other] variant involves a user prompt for a custom shape description.
impl FromString for Shape {
    fn from_string(shape: &str) -> Shape {
        match shape {
            "Cylindrical" => Shape::Cylindrical,
            "Tapered" => Shape::Tapered,
            "Other" => Shape::Other("".to_string()),
            _ => panic!("Invalid shape"),
        }
    }
}

/// Implementation of the [GetVariants] trait for [Shape]. This enables the creation of a vector containing all possible variants as string values.
impl GetVariants for Shape {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Cylindrical"),
            String::from("Tapered"),
            String::from("Other"),
        ]
    }
}

/// Implementation of the [std::fmt::Display] trait for [Shape]. This allows a [Shape] instance to be converted to a string for display purposes.
/// For the [Shape::Other] variant, the custom shape description is displayed.
impl std::fmt::Display for Shape {
    /// Returns the string description of a [Shape] instance.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Cylindrical => write!(f, "Cylindrical"),
            Shape::Tapered => write!(f, "Tapered"),
            Shape::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let cylindrical = Shape::Cylindrical;
        let tapered = Shape::Tapered;
        let other = Shape::Other(String::from("test"));

        assert_eq!(cylindrical, Shape::Cylindrical);
        assert_eq!(tapered, Shape::Tapered);
        assert_eq!(other, Shape::Other(String::from("test")));
    }
}
