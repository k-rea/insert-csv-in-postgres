extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate diesel;

use std::error::Error;
use std::{env, process};
use std::ffi::OsString;

use dotenv::dotenv;
use diesel::{Insertable, insert_into, RunQueryDsl};
use crate::diesel::Connection;
use diesel::pg::PgConnection;

pub mod schema;
use crate::schema::persons;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name=persons)]
struct NewPerson {
    family_name: String,
    first_name: String,
    age: i32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let conn = &mut establish_connection();
    let file_path = get_first_arg()?;

    let mut rdr = csv::Reader::from_path(&file_path)?;
    let mut vec: Vec<NewPerson> = Vec::new();
    for result in rdr.deserialize::<NewPerson>() {
        vec.push(result?);
    }
    println!("{:?}", vec);
    insert_into(persons::table).values(&vec).execute(conn)?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}