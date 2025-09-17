use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use strum::{Display, EnumIter};

/// Represents the subjects available for NCERT books.
#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    EnumIter,
    Copy,
    Clone,
    Hash,
    Deserialize,
    Default,
    clap::ValueEnum,
    strum::EnumString,
)]
pub enum Subject {
    #[default]
    #[serde(rename = "Mathematics")]
    Math,
    #[serde(rename = "Science")]
    Science,
    #[serde(rename = "English")]
    English,
    #[serde(rename = "History")]
    History,
    #[serde(rename = "Geography")]
    Geography,
    #[serde(rename = "Politics")]
    Politics,
    #[serde(rename = "Hindi")]
    Hindi,
    #[serde(rename = "Sanskrit")]
    Sanskrit,
    #[serde(rename = "SocialScience")]
    SocialScience,
    #[serde(rename = "EnvironmentalEducation")]
    EnvironmentalEducation,
    #[serde(rename = "HealthAndPhysicalEducation")]
    HealthAndPhysicalEducation,
}

/// Represents the grade level, stored as a `u8`.
pub type Grade = u8;

/// Represents a single NCERT book with its title and PDF code.
#[derive(Debug, Clone, Deserialize, Default, derive_more::Display)]
#[display("{}", title)]
pub struct Book {
    pub title: String,
    pub pdf_code: String,
}

/// A collection of books for a specific subject.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SubjectBooks {
    pub books: Vec<Book>,
}

/// Represents the entire collection of NCERT books, organized by grade and subject.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct NcertBooks {
    pub ninth: HashMap<Subject, SubjectBooks>,
    pub tenth: HashMap<Subject, SubjectBooks>,
}

/// Custom error types for book-related operations.
#[derive(thiserror::Error, Debug)]
pub enum BookError {
    /// Error indicating that no book was found for a given subject and grade.
    #[error("No book found for subject {0:?} and grade {1}")]
    BookNotFound(Subject, Grade),
    /// Error indicating that no grade was found for a given subject.
    #[error("No grade found for subject {0:?}")]
    GradeNotFound(Subject),
}
