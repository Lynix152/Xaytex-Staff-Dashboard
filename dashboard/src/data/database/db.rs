#[warn(dead_code)]

use sqlite;
extern crate firebase;

use self::firebase::Firebase;

pub fn database()
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


pub fn connect() -> Firebase
{
    let firebase = match Firebase::new("") {
        OK(connection) => { connection }
        Err(_) => { panic!("Could not get a connection to firebase. Please contact Lynix152") }
    };
}