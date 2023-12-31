// src/main.rs
#[macro_use]
extern crate diesel;

use std::env;

use diesel::prelude::*;
use diesel::prelude::*;
use dotenv::dotenv;

use models::{NewPerson, Person};
use schema::persons;
use utils::gen_ulid_str;

mod models;
mod schema;

mod config;

// use actix::*;
// use actix_cors::Cors;
// use actix_files::Files;
// use actix_web::{App, http, HttpServer, web};

// mod db;
// mod models;
// mod routes;
// mod schema;
// mod server;
// mod session;

// table! {
//     persons (id) {
//         id -> Integer,
//         name -> Text,
//         age -> Integer,
//     }
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ulid = gen_ulid_str();

    // Convert ULID to string
    // let ulid_string = u.to_string();
    println!("Generated ULID: {}", ulid);

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url).unwrap();

    // Insert
    let new_person = NewPerson { name: "Alice", age: 30 };
    diesel::insert_into(persons::table)
        .values(&new_person)
        .execute(&mut conn)
        .unwrap();

    // Query
    let results: Vec<Person> = persons::table
        .limit(5)
        .load::<Person>(&mut conn)
        .unwrap();

    for person in results {
        println!("{}: {} ({})", person.id, person.name, person.age);
    }

    // Parse a ULID from string
    // let parsed_ulid = ulid::Ulid::from_string(&ulid_string).unwrap();

    // println!("Parsed ULID: {}", parsed_ulid);

    // sleep(10).await();

    // let server = server::ChatServer::new().start();
    // let conn_spec = "chat.db";
    // let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    // let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    // let server_addr = "127.0.0.1";
    // let server_port = 8080;
    // let app = HttpServer::new(move || {
    //     let cors = Cors::default()
    //         .allowed_origin("http://localhost:3000")
    //         .allowed_origin("http://localhost:8080")
    //         .allowed_methods(vec!["GET", "POST"])
    //         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    //         .allowed_header(http::header::CONTENT_TYPE)
    //         .max_age(3600);
    //     App::new()
    //         .app_data(web::Data::new(server.clone()))
    //         // .app_data(web::Data::new(pool.clone()))
    //         // .wrap(cors)
    //         // .service(web::resource("/").to(routes::index))
    //         // .route("/ws", web::get().to(routes::chat_server))
    //         // .service(routes::create_user)
    //         // .service(routes::get_user_by_id)
    //         // .service(routes::get_user_by_phone)
    //         // .service(routes::get_conversation_by_id)
    //         // .service(routes::get_rooms)
    //         .service(Files::new("/", "./static"))
    // })
    //     .workers(2)
    //     .bind((server_addr, server_port))?
    //     .run();
    // println!("Server running at http://{server_addr}:{server_port}/");
    // app.await
    Ok(())
}