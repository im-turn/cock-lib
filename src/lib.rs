//! A library for comprehensive grading of cocks.
//!
//! `cock-lib` provides a modular, extensible, and detailed system for
//! grading and classifying cocks based on various metrics.
//!
//! This library allows users to:
//! - Assign scores based on multiple factors such as size, shape, and aesthetics.
//! - Easily extend the functionality by adding new grading methods.
//! - Obtain a comprehensive summary of the cocks score, grade, stats, etc.
//!
//! # Simple Example
//! ```
//! use cock_lib::{
//!     CockStruct,
//!     cock_parts::{Size, Aesthetic, Balls, Shape, Curvature, Circumcision, Veininess, Abnormalities, Inches}
//! };
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

pub mod cock_parts;
pub mod cock_handler;
pub mod cock_struct;
pub mod user;
pub mod traits;
pub mod tier;

pub use user::{ID, InnerUser};
pub use traits::{GetVariants, FromString, Score};
pub use tier::Tier;
pub use cock_struct::CockStruct;
pub use cock_handler::{CockHandler, CockResult};

#[cfg(feature = "bin_use")]
pub mod bin_use;
