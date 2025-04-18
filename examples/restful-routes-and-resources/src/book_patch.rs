// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;

// Demo book patch structure with some example fields for id, title, author.
// A production app could prefer an id to be type u32, UUID, etc.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct BookPatch {
    pub id: u32,
    pub title: Option<String>,
    pub author: Option<String>,
}

// Display the book using the format "{title} by {author}".
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for BookPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            "{:?} by {:?}", 
            self.title, 
            self.author,
        )
    }
}
