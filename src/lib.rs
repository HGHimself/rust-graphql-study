#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate juniper;

pub mod models;
pub mod schema;
pub mod utils;

use diesel::prelude::*;
use dotenv::dotenv;
use juniper::FieldResult;
use std::env;

pub struct Context;
impl juniper::Context for Context {}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_test() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct QueryRoot;

#[graphql_object(context = "Context")]
impl QueryRoot {
    fn categories(context: &Context) -> FieldResult<Vec<models::categories::Category>> {
        let connection = establish_connection();
        Ok(models::categories::read(&connection))
    }
}
