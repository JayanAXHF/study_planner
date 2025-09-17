pub mod cli;
pub mod errors;
pub mod structs;
use color_eyre::eyre::Result;
use std::{
    fs::{self},
    time::Duration,
};
use structs::*;
use tracing::info;

impl NcertBooks {
    /// Retrieves a slice of books for a given subject and grade.
    pub fn get_books(&self, subject: Subject, grade: Grade) -> Result<&[Book], BookError> {
        let grade_map = match grade {
            9 => &self.ninth,
            10 => &self.tenth,
            _ => panic!("Invalid grade"),
        };
        grade_map
            .get(&subject)
            .map(|sb| sb.books.as_slice())
            .ok_or(BookError::BookNotFound(subject, grade))
    }

    /// Downloads a specific chapter of a book.
    pub async fn download_book<P>(&self, book: &Book, chapter: u8, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let file_name = format!(
            "{}-{:0>2}.pdf",
            book.title.to_lowercase().replace(" ", "_"),
            chapter
        );
        let book_path = path.as_ref().join(file_name);
        let chapter_code = format!("{}{:0>2}", book.pdf_code, chapter);
        let url = format!("https://ncert.nic.in/textbook/pdf/{chapter_code}.pdf");
        info!(url);
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                     AppleWebKit/537.36 (KHTML, like Gecko) \
                     Chrome/139.0.0.0 Safari/537.36",
            )
            .http1_only()
            .timeout(Duration::from_secs(30))
            .build()?;

        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        fs::write(&book_path, &bytes)?;
        Ok(())
    }
    /// Load NCERT books from the embedded TOML file.
    pub fn load_books() -> Result<Self> {
        let data = include_str!("./bookcodes.toml");
        let parsed: NcertBooks = toml::from_str(data)?;
        Ok(parsed)
    }
}
