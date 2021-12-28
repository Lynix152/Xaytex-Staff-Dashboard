#[warn(dead_code)]

use sqlite;
//extern crate firebase;

use firebase::Firebase;

pub fn db()
{
    let connection = sqlite::open(":memory:").unwrap();

    connection
        .execute(
            "
        CREATE TABLE Errors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            error TEXT NOT NULL,
            timestamp DATETIME NOT NULL
        )
        ",
        )
        .unwrap();

    connection
        .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}
