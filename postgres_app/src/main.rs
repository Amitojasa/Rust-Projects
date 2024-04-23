use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author{
    _id:i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://amitojsinghahuja:@localhost:5432/library", NoTls)
        .expect("Failed to connect to database");

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author(
            id  SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book(
            id        SERIAL PRIMARY KEY,
            title     VARCHAR NOT NULL,
            author_id INTEGER NOT NULL REFERENCES author(id)
        )
    ")?;

    let mut authors = HashMap::new();

    authors.insert(String::from("Amitoj"),"Canada");
    authors.insert(String::from("Amitoj2"),"India");
    authors.insert(String::from("Amitoj3"),"India");

    for (key, value) in &authors{
        let author = Author{
            _id : 0,
            name : key.to_string(),
            country: value.to_string()
        };

        client.execute("INSERT INTO author(name,country) VALUES($1,$2)", &[&author.name, &author.country])?;
    }

    for row in client.query("SELECT id,name,country from author", &[])?{
        let author = Author{
            _id:row.get(0),
            name:row.get(1),
            country:row.get(2),
        };

        println!("{} is from {}",author.name, author.country);
    }

    Ok(())
}
