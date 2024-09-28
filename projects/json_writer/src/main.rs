use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize,Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}


fn main() {
    println!("Hello, world!");

    let  article = Article {
        article: String::from("New article"),
        author:String::from("Sumit"),
        paragraphs: vec![
            Paragraph {
                name: String::from("First paragraph"),
            },
            Paragraph {
                name: String::from("Mid paragraph"),
            },
            Paragraph {
                name: String::from("Last paragraph"),
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("The Json is {}",json);
}
