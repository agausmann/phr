use anyhow::ensure;
use phr_backend::Api;
use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::Write;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let schema_json = Api::without_database().introspect()?;

    let schema_path = format!("{}/schema.json", out_dir);
    fs::write(&schema_path, &schema_json)?;

    let query_source_dir = "src/queries";
    println!("cargo:rerun-if-changed={}", query_source_dir);

    let query_out_dir = format!("{}/queries", out_dir);
    fs::remove_dir_all(&query_out_dir).or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            Ok(())
        } else {
            Err(err)
        }
    })?;
    fs::create_dir_all(&query_out_dir)?;

    for entry in fs::read_dir(&query_source_dir)? {
        let entry = entry?;
        let query_source = entry.path();
        let success = Command::new("graphql-client")
            .arg("generate")
            .arg(&query_source)
            .arg("--schema-path")
            .arg(&schema_path)
            .arg("-o")
            .arg(&query_out_dir)
            .status()?
            .success();
        ensure!(success, "unable to generate graphql query");
        println!("cargo:rerun-if-changed={}", query_source.display());
    }

    let mut queries_out = File::create(format!("{}/queries.rs", out_dir))?;
    for entry in fs::read_dir(&query_out_dir)? {
        let entry = entry?;
        writeln!(queries_out, "include!({:?});", entry.path())?;
    }

    Ok(())
}
