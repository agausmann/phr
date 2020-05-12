// this is still needed because diesel devs can't be bothered to implement it
// in a backwards compatible way. ugh.
#[macro_use]
extern crate diesel;

mod api;
mod model;
mod parser;
mod schema;

use self::api::{Context, Mutation, Query, Schema};
use self::parser::parse_race;
use chrono::naive::NaiveDate;
use diesel::prelude::*;

pub struct Database {
    conn: MysqlConnection,
}

impl Database {
    pub fn connect(db_url: &str) -> anyhow::Result<Database> {
        Ok(Database {
            conn: Connection::establish(db_url)?,
        })
    }

    pub fn add_race(&self, id: i32, date: NaiveDate, results: &str) -> anyhow::Result<()> {
        let race = parse_race(results)?;
        race.insert_into(&self.conn, id, date)
    }
}

pub struct Api {
    context: Context,
    schema: Schema,
}
impl Api {
    pub fn new(db_url: &str) -> anyhow::Result<Api> {
        Ok(Api {
            context: Context {
                db: Connection::establish(db_url)?,
            },
            schema: Schema::new(Query, Mutation),
        })
    }
}
