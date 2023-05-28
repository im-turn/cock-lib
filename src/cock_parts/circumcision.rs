use crate::{FromString, GetVariants};

/// [Circumcision] is an enum that represents the circumcision status of a cock.
#[derive(Debug, PartialEq, Clone)]
pub enum Circumcision {
    Circumcised,
    Uncircumcised,
}

/// The [GetVariants] trait implementation for [Circumcision] returns a vector of the possible variants of [Circumcision].
impl GetVariants for Circumcision {
    fn get_variants() -> Vec<String> {
        vec![String::from("Circumcised"), String::from("Uncircumcised")]
    }
}

/// The [FromString] traits implementation for [Circumcision] returns a [Circumcision] variant.
impl FromString for Circumcision {
    fn from_string(circumcision: &str) -> Circumcision {
        match circumcision {
            "Circumcised" => Circumcision::Circumcised,
            "Uncircumcised" => Circumcision::Uncircumcised,
            _ => panic!("Invalid circumcision"),
        }
    }
}

/// The [std::fmt::Display] traits implementation for [Circumcision] returns a [std::fmt::Result].
impl std::fmt::Display for Circumcision {
    /// Returns a string representation of the [Circumcision] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Circumcision::Circumcised => write!(f, "Circumcised"),
            Circumcision::Uncircumcised => write!(f, "Uncircumcised"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circumcision_score() {
        let circumsized = Circumcision::Circumcised;
        let uncircumsized = Circumcision::Uncircumcised;

        assert_eq!(circumsized, Circumcision::Circumcised);
        assert_eq!(uncircumsized, Circumcision::Uncircumcised);
    }
}
