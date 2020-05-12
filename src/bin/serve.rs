use dotenv::dotenv;
use phc_stats::Api;
use std::env;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let api = Api::new(&database_url)?;
    api.serve(([127, 0, 0, 1], 8080));

    Ok(())
}
