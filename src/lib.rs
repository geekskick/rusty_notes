pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::NewItem;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_item(conn : &SqliteConnection, item: NewItem)->usize{
    use schema::lists;

    diesel::insert_into(lists::table).values(item).execute(conn).expect("Insert failed")
}
