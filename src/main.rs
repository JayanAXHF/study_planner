use clap::Parser;
use color_eyre::eyre::{self, Result};
use colored::Colorize;
use itertools::Itertools;
use std::{
    io::{BufWriter, Write},
    process::Stdio,
    sync::LazyLock,
};
use study_planner::{
    cli::{self, OptionExt},
    structs::NcertBooks,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub static PAGER_COMMAND: LazyLock<String> =
    LazyLock::new(|| std::env::var("STUDY_PAGER").unwrap_or_else(|_| "less -R".to_string()));

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let filter = if cli.debug {
        LevelFilter::DEBUG
    } else if cli.silent {
        LevelFilter::ERROR
    } else {
        LevelFilter::INFO
    };
    let env_filter = EnvFilter::builder()
        .with_default_directive(filter.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .compact()
        .init();

    study_planner::errors::init()?;
    match cli.command {
        cli::Commands::Download {
            subject,
            grade,
            title,
            path,
            chapter,
        } => {
            let subject = subject.unwrap_or_user_input("Enter subject")?;
            let grade = grade.unwrap_or_user_input("Enter grade")?;
            let ncert = NcertBooks::load_books()?;
            let books = ncert.get_books(subject, grade)?;
            let chapter = chapter.unwrap_or_user_input("Enter chapter number")?;
            let book = if let Some(title) = title {
                books
                    .iter()
                    .find(|book| {
                        book.title.to_lowercase().replace([' ', '_', '-'], "")
                            == title.to_lowercase().replace([' ', '_', '-'], "")
                    })
                    .ok_or_else(|| eyre::eyre!("No book found with title {title}"))?
                    .clone()
            } else {
                inquire::Select::new("Select a book", books.to_vec()).prompt()?
            };
            ncert.download_book(&book, chapter, &path).await?;
        }

        cli::Commands::List { subject, grade } => {
            let command = PAGER_COMMAND.clone();
            let mut iter = command.split_whitespace();
            let cname = iter.next().unwrap();
            let rest = if iter.next().is_some() {
                command.split_whitespace().to_owned().collect_vec()[1..].to_owned()
            } else {
                [].to_vec()
            };
            let mut command = std::process::Command::new(cname)
                .args(rest)
                .stdin(Stdio::piped())
                .spawn()?;
            let mut stdin = BufWriter::new(command.stdin.take().unwrap());
            let ncert = NcertBooks::load_books()?;
            if subject.is_none() && grade.is_none() {
                writeln!(stdin, "{}", "Ninth".bold())?;
                for (subject, subject_books) in ncert.ninth {
                    writeln!(stdin, "\t{}", subject.to_string().cyan())?;
                    for book in subject_books.books {
                        writeln!(stdin, "\t\t{}", book.title)?;
                    }
                }

                writeln!(stdin, "{}", "Tenth".bold())?;
                for (subject, subject_books) in ncert.tenth {
                    writeln!(stdin, "\t{}", subject.to_string().cyan())?;
                    for book in subject_books.books {
                        writeln!(stdin, "\t\t{}", book.title)?;
                    }
                }
            } else {
                todo!()
            }
            stdin.flush()?;
            command.wait()?;
        }
        #[expect(unreachable_patterns)]
        _ => {}
    }
    Ok(())
}
