#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

table! {
    persons (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

#[derive(Insertable)]
#[table_name = "persons"]
struct NewPerson<'a> {
    name: &'a str,
    age: i32,
}

#[derive(Queryable)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url).unwrap();

    // Insert
    let new_person = NewPerson { name: "Alice", age: 30 };
    diesel::insert_into(persons::table)
        .values(&new_person)
        .execute(&conn)
        .unwrap();

    // Query
    let results: Vec<Person> = persons::table
        .limit(5)
        .load::<Person>(&conn)
        .unwrap();

    for person in results {
        println!("{}: {} ({})", person.id, person.name, person.age);
    }
}
