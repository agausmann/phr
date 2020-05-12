use anyhow::Context;
use chrono::naive::NaiveDate;
use dotenv::dotenv;
use phc_stats::Database;
use std::{env, fs};

const USAGE: &str = "usage: add_race <file> <id> <YYYY-MM-dd>";

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let file = args.nth(1).context(USAGE)?;
    let id: i32 = args.next().context(USAGE)?.parse().context(USAGE)?;
    let date: NaiveDate = args.next().context(USAGE)?.parse().context(USAGE)?;
    let results = fs::read_to_string(&file)?;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = Database::connect(&database_url)?;

    database.add_race(id, date, &results)?;

    Ok(())
}
