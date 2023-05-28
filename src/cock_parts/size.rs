use crate::{FromString, GetVariants, Score};

/// [SizeType] is an enum that represents the metric of measurement for the cocks [Size]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SizeType {
    Centimeters,
    Inches,
}

/// The [Score] trait implementation for [SizeType] provides a score value based on the size type.
impl GetVariants for SizeType {
    fn get_variants() -> Vec<String> {
        vec![String::from("Centimeters"), String::from("Inches")]
    }
}

/// The [FromString] trait implementation for [SizeType] returns a [SizeType] variant based on the string provided.
impl FromString for SizeType {
    fn from_string(size_type: &str) -> SizeType {
        match size_type {
            "Centimeters" => SizeType::Centimeters,
            "Inches" => SizeType::Inches,
            _ => panic!("Invalid size type"),
        }
    }
}

/// [Size] is a struct that represents the size of a cock.
#[derive(Debug, Clone)]
pub struct Size {
    pub length: f32,
    pub girth: f32,
    pub size_type: SizeType,
}

/// Implementation of [Score] trait for [Size]. The score is calculated based on the size type.
impl Size {
    /// Function to create a Size instance from given length and girth in centimeters.
    pub fn from_cm(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Centimeters,
        }
    }

    /// Function to create a [Size] instance from given length and girth in inches.
    pub fn from_in(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Inches,
        }
    }
}

impl Score for Size {
    /// Function to calculate the score of a [Size] instance.
    fn score(&self) -> u32 {
        match self {
            Size {
                size_type: SizeType::Inches,
                ..
            } => {
                let length_score = match (self.length * 10.0) as u32 {
                    0..=35 => 1,
                    36..=50 => 2,
                    51..=57 => 3,
                    58..=70 => 4,
                    71..=80 => 5,
                    _ => 2,
                };

                let girth_score = match (self.girth * 10.0) as u32 {
                    0..=33 => 1,
                    34..=40 => 2,
                    41..=46 => 3,
                    47..=54 => 4,
                    55..=63 => 5,
                    _ => 2,
                };
                length_score + girth_score
            }
            Size {
                size_type: SizeType::Centimeters,
                ..
            } => {
                let length_score = match (self.length * 10.0) as u32 {
                    0..=90 => 1,
                    91..=125 => 2,
                    126..=145 => 3,
                    146..=175 => 4,
                    176..=200 => 5,
                    _ => 1,
                };

                let girth_score = match (self.girth * 10.0) as u32 {
                    0..=85 => 1,
                    86..=100 => 2,
                    101..=115 => 3,
                    116..=135 => 4,
                    136..=155 => 5,
                    _ => 1,
                };

                length_score + girth_score
            }
        }
    }
}

/// Implementation of [std::fmt::Display] trait for [Size].
impl std::fmt::Display for Size {
    /// Function to format the [Size] instance.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.size_type {
            SizeType::Centimeters => write!(f, "{}cm x {}cm", self.length, self.girth),
            SizeType::Inches => write!(f, "{}in x {}in", self.length, self.girth),
        }
    }
}
