use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize,Deserialize)]
struct Article{
    name: String,
    author:String,
    paragraph: Vec<Paragraph>
}


fn main() {
    let article: Article= Article{
        name:String::from("Hello"),
        author: String::from("amitoj"),
        paragraph: vec![
            Paragraph{
                name: String::from("test")
            },Paragraph{
                name: String::from("test2")
            },
        ]
    };

    let json= serde_json::to_string(&article).unwrap();
    println!("json is: {}", json);
}
