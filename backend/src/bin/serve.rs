use dotenv::dotenv;
use phr_backend::Api;
use std::env;
use warp::Filter;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let api = Api::new(&database_url)?;
    warp::serve(
        (warp::path("graphql").and(api.to_filter()))
            .or(warp::path("graphiql").and(juniper_warp::graphiql_filter("/graphql"))),
    )
    .run(([127, 0, 0, 1], 8080));

    Ok(())
}
