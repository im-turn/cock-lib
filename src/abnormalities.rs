use crate::{FromString, GetVariants, Score};

/// The [Abnormalities] enum represents the different types of abnormalities of the cock.
#[derive(Debug, PartialEq, Clone)]
pub enum Abnormalities {
    None,
    Minor(String),
    Major(String),
}

/// The [Score] trait implementation for [Abnormalities] provides a score value based on the type of abnormality.
impl Score for Abnormalities {
    fn score(&self) -> u32 {
        match self {
            Abnormalities::None => 5,
            Abnormalities::Minor(_) => 3,
            Abnormalities::Major(_) => 1,
        }
    }
}

/// The [GetVariants] trait implementation for [Abnormalities] returns a vector of the possible variants of [Abnormalities].
impl GetVariants for Abnormalities {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("None"),
            String::from("Minor"),
            String::from("Major"),
        ]
    }
}

/// Additional methods for the [Abnormalities] struct.
impl Abnormalities {
    /// Returns a string representation of the abnormality.
    pub fn get_abnormality(&self) -> &str {
        match self {
            Abnormalities::None => "None",
            Abnormalities::Minor(minor) => minor,
            Abnormalities::Major(major) => major,
        }
    }
}

/// The [FromString] trait implementation for [Abnormalities] returns an [Abnormalities] variant based on the string provided.
impl FromString for Abnormalities {
    fn from_string(abnormality: &str) -> Abnormalities {
        match abnormality {
            "None" => Abnormalities::None,
            "Minor" => Abnormalities::Minor("".to_string()),
            "Major" => Abnormalities::Major("".to_string()),
            _ => panic!("Invalid abnormality"),
        }
    }
}

/// The [std::fmt::Display] trait implementation for [Abnormalities] returns a string representation of the abnormality.
impl std::fmt::Display for Abnormalities {
    /// Returns a string representation of the [Abnormalities] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Abnormalities::None => write!(f, "None"),
            Abnormalities::Minor(minor) => write!(f, "Minor: {}", minor),
            Abnormalities::Major(major) => write!(f, "Major: {}", major),
        }
    }
}

/// Unit tests for the [Abnormalities] struct.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abnormalities_test() {
        let major_abnormality = Abnormalities::Major(String::from("major"));
        let minor_abnormality = Abnormalities::Minor(String::from("minor"));
        let no_abnormality = Abnormalities::None;

        assert_eq!(major_abnormality.get_abnormality(), "major");
        assert_eq!(minor_abnormality.get_abnormality(), "minor");
        assert_eq!(no_abnormality.get_abnormality(), "None");

        assert_eq!(major_abnormality.score(), 1);
        assert_eq!(minor_abnormality.score(), 3);
        assert_eq!(no_abnormality.score(), 5);
    }
}
