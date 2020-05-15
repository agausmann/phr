use anyhow::Context;
use dotenv::dotenv;
use juniper_warp::graphiql_filter;
use phr_backend::Api;
use std::env;
use warp::Filter;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    //TODO serve frontend files
    let filter = warp::path("api").and(
        (warp::path("graphql").and(Api::new(&database_url)?.to_filter()))
            .or(warp::path("graphiql").and(graphiql_filter("/api/graphql"))),
    );
    warp::serve(filter).run(([127, 0, 0, 1], 8000));

    Ok(())
}
