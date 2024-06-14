use serde::{Deserialize, Serialize};
use std::{env, fs, process};



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



fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(0);
    }

    let file_name = &env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_name).expect("The file could not be read");
    
    let parsed: Article = serde_json::from_str(&content).expect("Error when parsing JSON");
    println!("{:#?}", parsed);
}