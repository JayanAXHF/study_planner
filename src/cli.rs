use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

use crate::{Grade, Subject};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, global = true, default_value_t = false)]
    pub silent: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download NCERT books
    Download {
        /// Subject
        #[arg(value_name = "SUBJECT")]
        subject: Option<Subject>,
        /// Grade
        #[arg(value_name = "GRADE")]
        grade: Option<u8>,
        /// Book title
        #[arg(value_name = "TITLE")]
        title: Option<String>,
        /// Path to save the books
        #[arg(short, long, value_name = "PATH", default_value_os_t = PathBuf::from("."))]
        path: PathBuf,
        #[arg(short, long, value_name = "Chapter")]
        chapter: Option<u8>,
    },
}

pub trait OptionExt<T> {
    fn unwrap_or_user_input(self, prompt: &str) -> Result<T>;
}

impl<T: FromStr + Default> OptionExt<T> for Option<T> {
    fn unwrap_or_user_input(self, prompt: &str) -> Result<T> {
        match self {
            Some(value) => Ok(value),
            None => {
                let value = inquire::Text::new(prompt).prompt()?;
                let value = value.trim();
                let t_value = T::from_str(value).unwrap_or_default();
                Ok(t_value)
            }
        }
    }
}
