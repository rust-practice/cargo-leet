use convert_case::{Case, Casing};
use std::{
    error::Error,
    fs::{remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

fn update_lib(slug_snake: &str) -> Result<(), Box<dyn Error>> {
    let lib_path = PathBuf::from(format!("{}/../src/lib.rs", env!("CARGO_MANIFEST_DIR")));
    let mut lib = OpenOptions::new().append(true).open(lib_path)?;
    let _ = lib.write(format!("pub mod {slug_snake};").as_bytes())?;
    Ok(())
}

pub fn write_file(title_slug: &str, code_snippet: String) -> Result<(), Box<dyn Error>> {
    let slug_snake = title_slug.to_case(Case::Snake);
    let path = PathBuf::from(format!(
        "{}/../src/{slug_snake}.rs",
        env!("CARGO_MANIFEST_DIR")
    ));
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
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir(format!("{}/../", env!("CARGO_MANIFEST_DIR")))
        .spawn()?
        .wait()?;
    Ok(())
}
