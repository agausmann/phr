use anyhow::ensure;
use std::process::Command;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let result = Command::new("../frontend/x.sh").status()?;
    ensure!(result.success(), "failed to build frontend crate");

    for entry_result in WalkDir::new("../frontend")
        .into_iter()
        .filter_entry(|entry| entry.path().as_os_str() != "../frontend/static")
    {
        let entry = entry_result?;
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    Ok(())
}
