use phr_backend::Api;
use std::process::Command;
use std::{env, fs};

const QUERIES: &[&str] = &[];

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let schema_json = Api::without_database()
        .introspect()
        .expect("unable to introspect schema");

    let schema_path = format!("{}/schema.json", out_dir);
    fs::write(&schema_path, &schema_json).expect("unable to write schema.json");

    let query_source_dir = "src/queries";
    println!("cargo:rerun-if-changed={}", query_source_dir);

    let query_out_dir = format!("{}/queries", out_dir);
    fs::create_dir_all(&query_out_dir).expect("cannot create query output directory");

    for &query in QUERIES {
        let query_source = format!("{}/{}.graphql", query_source_dir, query);
        let success = Command::new("graphql-client")
            .arg("generate")
            .arg(&query_source)
            .arg("--schema-path")
            .arg(&schema_path)
            .arg("-o")
            .arg(&query_out_dir)
            .status()
            .expect("unable to run graphql-client cli")
            .success();
        assert!(success, "unable to generate graphql query");
        println!("cargo:rerun-if-changed={}", query_source);
    }
}
