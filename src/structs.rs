use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use strum::{Display, EnumIter};

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

pub type Grade = u8;

#[derive(Debug, Clone, Deserialize, Default, derive_more::Display)]
#[display("{}", title)]
pub struct Book {
    pub title: String,
    pub pdf_code: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SubjectBooks {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NcertBooks {
    pub ninth: HashMap<Subject, SubjectBooks>,
    pub tenth: HashMap<Subject, SubjectBooks>,
}

#[derive(thiserror::Error, Debug)]
pub enum BookError {
    #[error("No book found for subject {0:?} and grade {1}")]
    BookNotFound(Subject, Grade),
    #[error("No grade found for subject {0:?}")]
    GradeNotFound(Subject),
}
