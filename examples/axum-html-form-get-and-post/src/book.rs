// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::{Serialize, Deserialize};

// Demo book structure with some example fields for title and author.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
}
