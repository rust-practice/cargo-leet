use convert_case::{Case, Casing};
use std::{
    fs::{remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

// TODO: Add logging to all functions

fn update_lib(slug_snake: &str) -> anyhow::Result<()> {
    let lib_path = PathBuf::from(format!("{}/../src/lib.rs", env!("CARGO_MANIFEST_DIR")));
    let mut lib = OpenOptions::new().append(true).open(lib_path)?;
    let _ = lib.write(format!("pub mod {slug_snake};").as_bytes())?;
    Ok(())
}

pub fn write_file(title_slug: &str, code_snippet: String) -> anyhow::Result<()> {
    let slug_snake = title_slug.to_case(Case::Snake);
    // TODO: Find way to specify desired new file name from a config
    let path = PathBuf::from(format!("src/{slug_snake}.rs"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.clone())?;
    file.write_all(code_snippet.as_bytes())?;
    let output = update_lib(&slug_snake);
    if output.is_err() {
        // clean up
        remove_file(path)?;
        output?;
    }
    // TODO: Check if there is a simpler way to do this
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .spawn()?
        .wait()?;
    Ok(())
}
