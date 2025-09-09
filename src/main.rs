use clap::Parser;
use color_eyre::eyre::{self, Result};
use study_planner::{
    cli::{self, OptionExt},
    structs::NcertBooks,
};
use tracing::info;
use tracing_subscriber::Layer;

#[tokio::main]
async fn main() -> Result<()> {
    study_planner::errors::init()?;
    tracing_subscriber::fmt::init();
    let cli = cli::Cli::parse();
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
            info!(ninth = ?ncert.ninth, tenth = ?ncert.tenth, "Loaded books");
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
    }
    Ok(())
}
