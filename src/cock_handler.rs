use crate::{CockStruct, Score, Tier, ID};

/// This struct encapsulates the result of a score calculation, [CockResult].
/// It includes the raw score and the percentage score.
#[derive(Debug, PartialEq, Clone)]
pub struct CockResult {
    pub score: f32,
    pub percentage: f32,
}

/// This struct handles all operations related to the [CockStruct] entity.
/// It keeps track of an identifier and the [CockStruct] itself.
#[derive(Debug, Clone)]
pub struct CockHandler {
    pub id: ID,
    pub cock: CockStruct,
}

impl CockHandler {
    /// Constructor for a new [CockHandler]    
    pub fn new(id: ID, cock: CockStruct) -> CockHandler {
        CockHandler { id, cock }
    }

    /// This method calculates the total score for the current [CockStruct].
    /// The calculation takes into account several attributes of the [CockStruct].
    /// It returns a [CockResult] containing the raw score and the percentage score.
    pub fn total_score(&self) -> CockResult {
        let actual_score = (
            self.cock.aesthetic.score()         // 10
            + self.cock.balls.score()           // 5
            + self.cock.veininess.score()       // 5
            + self.cock.abnormalities.score()   // 5
            + self.cock.size.score()) as f32    // 10
            * 2.0;

        let total_possible_score = 70.0;

        let percentage_score = actual_score / total_possible_score * 100.0;

        CockResult {
            score: actual_score,
            percentage: percentage_score,
        }
    }

    /// This method determines the grade for the current [CockStruct].
    /// The grade is based on the percentage score.
    pub fn grade(&self) -> Tier {
        let percentage_score = self.total_score().percentage;

        let score = percentage_score as i32;

        match score {
            91..=100 => Tier::S,
            81..=90 => Tier::A,
            71..=80 => Tier::B,
            61..=70 => Tier::C,
            51..=60 => Tier::D,
            41..=50 => Tier::E,
            _ => Tier::F,
        }
    }
}

/// This implementation of [std::fmt::Display] allows a [CockHandler] to be converted to a string for easy display.
impl std::fmt::Display for CockHandler {
    /// Returns a string representation of the [CockHandler] variant.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_score = self.total_score();
        let grade = self.grade();

        writeln!(f, "--- ID ---\n{}\n", self.id)?;
        writeln!(f, "--- Cock Info ---\n{}\n", self.cock)?;
        writeln!(f, "Score: {}", total_score.score)?;
        writeln!(f, "Percentage: {:.2}%", total_score.percentage)?;
        write!(f, "Grade: {:?}", grade)
    }
}

/// add testing
#[cfg(test)]
mod tests {}
