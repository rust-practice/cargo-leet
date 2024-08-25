use anyhow::{bail, Context};
use log::{error, info};
use std::{
    env,
    fs::{remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

use crate::tool::does_user_confirm;

fn update_lib(module_name: &str) -> anyhow::Result<()> {
    info!("Adding {module_name} to libs.rs");
    let lib_path = PathBuf::from("src/lib.rs");
    let mut lib = OpenOptions::new()
        .append(true)
        .open(&lib_path)
        .with_context(|| {
            format!(
                "Failed to open {:?}",
                env::current_dir()
                    .expect("Unable to resolve current directory")
                    .join(lib_path)
            )
        })?;
    // TODO Onè: Not duplicate module add if it already exists
    let _ = lib.write(format!("pub mod {module_name};").as_bytes())?;
    Ok(())
}

pub(crate) fn write_file(module_name: &str, module_code: &str) -> anyhow::Result<()> {
    info!("Writing code to disk for module {module_name}");
    let path = PathBuf::from(format!("src/{module_name}.rs"));
    // This creates a TOCTOU but the window is small
    if path.exists() && !(does_user_confirm(format!("{path:?} already exists. Overwrite?"))?) {
        bail!("aborted at user request");
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .with_context(|| format!("Failed to create '{}'", path.display()))?;
    file.write_all(module_code.as_bytes())
        .with_context(|| format!("Failed writing to '{}'", path.display()))?;
    let lib_update_status = update_lib(module_name);
    if lib_update_status.is_err() {
        error!("Failed to update lib.rs: Performing cleanup of partially completed command");
        // clean up
        remove_file(&path).with_context(|| {
            format!(
                "Failed to remove '{}' during cleanup after failing to update lib.rs",
                path.display()
            )
        })?;
        lib_update_status.context(
            "Failed to update lib.rs. Does the file exists? Is it able to be written to?",
        )?;
    }

    info!("Going to run rustfmt on files");
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .output()
        .context("Error running rustfmt")?;
    Ok(())
}
