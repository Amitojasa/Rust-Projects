use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph{
    name:String
}


#[derive(Serialize, Deserialize)]
struct Article{
    article:String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn read_json_typed(raw_json:&str) ->Article{

    let parsed : Article = serde_json::from_str(raw_json).unwrap();
    return parsed

}

fn main() {
    let json =  r#"
        {
            "article":"How to work with json",
            "author":"Amitoj",
            "paragraph":[
                {
                    "name":"First Para"
                },{
                    "name":"Second Para"
                },{
                    "name":"Last para"
                }
            ]
        }
    "#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n name of first para is {}", parsed.paragraph[0].name)
}
