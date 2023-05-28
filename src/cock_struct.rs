use crate::{
    Abnormalities, Aesthetic, Balls, Circumcision, Curvature, FromString, Inches, Shape, Size,
    Veininess,
};

/// Struct representing detailed information about a [CockStruct]. Each property of a [CockStruct]
/// is represented by a separate field, enabling fine-grained control and accurate descriptions.
#[derive(Debug, Clone)]
pub struct CockStruct {
    pub size: Size,
    pub aesthetic: Aesthetic,
    pub balls: Balls,
    pub shape: Shape,
    pub curvature: Curvature,
    pub circumcision: Circumcision,
    pub veininess: Veininess,
    pub abnormalities: Abnormalities,
}

impl CockStruct {
    /// Constructor for creating a new instance of [CockStruct].
    /// All parameters needed to fully describe a [CockStruct] are passed in as arguments.
    pub fn new(
        size: Size,
        aesthetic: Aesthetic,
        balls: Balls,
        shape: Shape,
        curvature: Curvature,
        circumcision: Circumcision,
        veininess: Veininess,
        abnormalities: Abnormalities,
    ) -> CockStruct {
        CockStruct {
            size,
            aesthetic,
            balls,
            shape,
            curvature,
            circumcision,
            veininess,
            abnormalities,
        }
    }

    /// Constructor for creating a new instance of [CockStruct] with the 'default' values.
    pub fn default() -> CockStruct {
        CockStruct {
            size: Size {
                length: 0.0,
                girth: 0.0,
                size_type: Inches,
            },
            aesthetic: Aesthetic::Normal,
            balls: Balls::Normal,
            shape: Shape::Other(String::from("")),
            curvature: Curvature::Other(String::from("")),
            circumcision: Circumcision::Uncircumcised,
            veininess: Veininess::Normal,
            abnormalities: Abnormalities::None,
        }
    }

    /// Function for editing an existing instance of a [CockStruct] value.
    /// The `part` parameter is used to specify which part of the [CockStruct] to edit.
    /// The `new` parameter is used to specify the new value of the part.
    pub fn from_str_part(&mut self, part: &str, new: &str) {
        match part {
            "Abnormalities" => self.abnormalities = Abnormalities::from_string(new),
            "Aesthetic" => self.aesthetic = Aesthetic::from_string(new),
            "Balls" => self.balls = Balls::from_string(new),
            "Circumcision" => self.circumcision = Circumcision::from_string(new),
            "Curvature" => self.curvature = Curvature::from_string(new),
            "Shape" => self.shape = Shape::from_string(new),
            "Veininess" => self.veininess = Veininess::from_string(new),
            _ => panic!("Invalid part"),
        }
    }

    /// Function for editing an existing instance of a [CockStruct] value.
    /// The `item` parameter is used to verify whether the current [CockStruct] value requires a user submitted value.
    /// [Abnormalities::Major], [Abnormalities::Minor], [Shape::Other], and [Curvature::Other] all require a user submitted value.
    /// The `part` parameter is used to specify which part of the [CockStruct] to edit ("Abnormalities", "Shape", "Curvature").
    /// The `new` parameter is used to specify the new value of the part.
    pub fn get_custom(&mut self, part: &str, item: &str, new: &str) {
        match item {
            "Major" => self.abnormalities = Abnormalities::Major(new.to_string()),
            "Minor" => self.abnormalities = Abnormalities::Minor(new.to_string()),
            "Other" => match part {
                "Shape" => self.shape = Shape::Other(new.to_string()),
                "Curvature" => self.curvature = Curvature::Other(new.to_string()),
                _ => panic!("Invalid part"),
            },
            _ => panic!("Invalid item"),
        }
    }
}

/// This implementation of [std::fmt::Display] allows a [CockStruct] to be converted to a string for easy display.
impl std::fmt::Display for CockStruct {
    /// Returns a string representation of a [CockStruct].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Aesthetic: {}", self.aesthetic)?;
        writeln!(f, "Balls: {}", self.balls)?;
        writeln!(f, "Shape: {}", self.shape)?;
        writeln!(f, "Curvature: {}", self.curvature)?;
        writeln!(f, "Circumcision: {}", self.circumcision)?;
        writeln!(f, "Veininess: {}", self.veininess)?;
        write!(f, "Abnormalities: {}", self.abnormalities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Inches;

    #[test]
    fn cock_struct_test() {
        let cock = CockStruct::new(
            Size {
                length: 5.6,
                girth: 4.1,
                size_type: Inches,
            },
            Aesthetic::UglyButUsable,
            Balls::PossibleCancer,
            Shape::Other(String::from("test")),
            Curvature::Right,
            Circumcision::Circumcised,
            Veininess::HealthyPumper,
            Abnormalities::Major(String::from("Active case of herpes")),
        );

        assert_eq!(cock.size.length, 5.6);
        assert_eq!(cock.size.girth, 4.1);
        assert_eq!(cock.size.size_type, Inches);
        assert_eq!(cock.aesthetic, Aesthetic::UglyButUsable);
        assert_eq!(cock.balls, Balls::PossibleCancer);
        assert_eq!(cock.shape.get_shape(), "test");
        assert_eq!(cock.curvature, Curvature::Right);
        assert_eq!(cock.circumcision, Circumcision::Circumcised);
        assert_eq!(cock.veininess, Veininess::HealthyPumper);
        assert_eq!(
            cock.abnormalities.get_abnormality(),
            "Active case of herpes"
        );
    }
}
