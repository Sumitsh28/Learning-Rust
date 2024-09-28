use serde::{Deserialize, Serialize}; 

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)] 
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    println!("Hello, world!");

    let json = r#"
    {
      "article": "how to work with json in Rust",
      "author": "Sumit",
      "paragraph": [
        {
          "name": "starting sentences"
        },
        {
          "name": "body of the paragraph"
        },
        {
          "name": "end of the paragraph"
        }
      ]
    }
    "#;

    let parsed: Article = read_json_typed(json);

    println!("\n\nThe name of the author is: {}", parsed.author);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
