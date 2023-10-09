extern crate diesel;

use diesel::prelude::*;
use diesel::prelude::*;

use crate::schema::*;

#[derive(Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub name: &'a str,
    pub age: i32,
}

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
