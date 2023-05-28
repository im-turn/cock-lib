//! A library for comprehensive grading of cocks.
//!
//! `cock-tier` provides a modular, extensible, and detailed system for
//! grading and classifying cocks based on various metrics.
//!
//! This library allows users to:
//! - Assign scores based on multiple factors such as size, shape, and aesthetics.
//! - Easily extend the functionality by adding new grading methods.
//! - Obtain a comprehensive summary of the cocks score, grade, stats, etc.
//!
//! # Example
//! ```
//! use lib::{CockStruct, Size, Aesthetic, Balls, Shape, Curvature, Circumcision, Veininess, Abnormalities, Inches};
//!
//! let cock = CockStruct::new(
//!     Size {
//!         length: 5.5,
//!         girth: 4.5,
//!         size_type: Inches,
//!     },
//!     Aesthetic::Normal,
//!     Balls::Normal,
//!     Shape::Cylindrical,
//!     Curvature::Straight,
//!     Circumcision::Uncircumcised,
//!     Veininess::Normal,
//!     Abnormalities::None,
//! );
//!
//! // Perform your operations on `cock`
//! ```
//!
//! # TODO
//! add more examples

mod abnormalities;
mod aesthetic;
mod balls;
pub mod bin_modules;
mod circumcision;
mod cock_handler;
mod cock_struct;
mod curvature;
mod shape;
mod size;
mod tier;
mod traits;
mod user;
mod veininess;

pub use abnormalities::Abnormalities;
pub use aesthetic::Aesthetic;
pub use balls::Balls;
pub use circumcision::Circumcision;
pub use cock_handler::{CockHandler, CockResult};
pub use cock_struct::CockStruct;
pub use curvature::Curvature;
pub use shape::Shape;
pub use size::{
    Size,
    SizeType::{self, Centimeters, Inches},
};
pub use tier::Tier;
pub use traits::{FromString, GetVariants, Score};
pub use user::{User as InnerUser, ID};
pub use veininess::Veininess;
