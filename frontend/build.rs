use dotenv::dotenv;
use phr_backend::Api;
use std::process::Command;
use std::{env, fs};

const QUERIES: &[&str] = &[];

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let schema_json = Api::new(&database_url)
        .expect("unable to build api")
        .introspect()
        .expect("unable to introspect schema");

    let out_dir = env::var("OUT_DIR").unwrap();
    let schema_path = format!("{}/schema.json", out_dir);

    fs::write(&schema_path, &schema_json).expect("unable to write schema.json");

    for &query in QUERIES {
        let success = Command::new("graphql-client")
            .arg("generate")
            .arg(format!("src/queries/{}.graphql", query))
            .arg("--schema-path")
            .arg(&schema_path)
            .arg("-o")
            .arg(&out_dir)
            .status()
            .expect("unable to run graphql-client cli")
            .success();
        assert!(success, "unable to generate graphql query")
    }
}
