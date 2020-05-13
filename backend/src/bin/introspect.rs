use dotenv::dotenv;
use phr_backend::Api;
use std::env;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let api = Api::new(&database_url)?;
    println!("{}", api.introspect()?);

    Ok(())
}
