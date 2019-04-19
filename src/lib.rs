pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Discovery, NewDiscovery};

pub fn create_discovery<'a>(conn: &PgConnection, discovery: NewDiscovery) -> Discovery {
    use schema::discoveries;

    diesel::insert_into(discoveries::table)
        .values(&discovery)
        .get_result(conn)
        .expect("Error saving new post")
}
