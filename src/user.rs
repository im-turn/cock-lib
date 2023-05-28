use crate::{FromString, GetVariants};

/// Represents a [User] of the cock analyzer.
/// `name` field is the name of the user.
/// `discord_name` field is the discord name of the user.
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub name: String,
    pub discord_name: String,
}

/// Represents a unique identifier for a user, their [ID].
/// Variants include an anonymous identifier or a specific user.
#[derive(Debug, PartialEq, Clone)]
pub enum ID {
    Anonymous,
    User(User),
}

/// Implementing [GetVariants] for [ID] enum to provide the different variant options for [ID].
impl GetVariants for ID {
    fn get_variants() -> Vec<String> {
        vec![String::from("Anonymous"), String::from("User")]
    }
}

/// Implementing [FromString] for [ID] enum to create an [ID] instance from a string.
impl FromString for ID {
    fn from_string(id: &str) -> ID {
        match id {
            "Anonymous" => ID::Anonymous,
            "User" => ID::User(User {
                name: String::from(""),
                discord_name: String::from(""),
            }),
            _ => panic!("Invalid ID"),
        }
    }
}

/// Implementing [std::fmt::Display] trait for [ID] for formatted print.
impl std::fmt::Display for ID {
    /// Returnis a string representation of the [ID] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::Anonymous => write!(f, "Anonymous User"),
            ID::User(user) => write!(
                f,
                "Username: {}\nDiscord name: {}",
                user.name, user.discord_name
            ),
        }
    }
}

/// Tests for the [User] and [ID] structs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test"),
        };

        assert_eq!(user.name, "S");
        assert_eq!(user.discord_name, String::from("test"));
    }

    #[test]
    fn test_id() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test"),
        };

        let id = ID::User(user);

        match id {
            ID::User(user) => {
                assert_eq!(user.name, "S");
                assert_eq!(user.discord_name, String::from("test"));
            }
            _ => panic!("Expected ID::User"),
        }
    }
}
