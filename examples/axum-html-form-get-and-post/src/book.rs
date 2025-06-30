// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;

// Demo book structure with some example fields for title and author.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
}
