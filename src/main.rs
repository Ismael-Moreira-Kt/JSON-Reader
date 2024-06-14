use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Paragraph {
    name: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}