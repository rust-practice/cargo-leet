use anyhow::{Context, bail};
use log::{error, info};
use std::{
    env,
    fs::{self, OpenOptions, remove_file},
    io::Write,
    path::PathBuf,
    process::Command,
};

use crate::tool::does_user_confirm;

/// Updates lib.rs by adding a module declaration for `module_name` only check
/// for possible duplication if `is_likely_already_exists` is true because it
/// opens the file twice in that case to avoid reading from and writing to the
/// file when the common case is expected to be append only
fn update_lib(module_name: &str, is_likely_already_exist: bool) -> anyhow::Result<()> {
    info!("Adding {module_name} to libs.rs");
    let lib_path = PathBuf::from("src/lib.rs");

    // Check to avoid duplicating module declaration
    if is_likely_already_exist {
        // Note this does not handle multi-line comments with /* */
        let contents = fs::read_to_string(&lib_path)
            .context("failed to read from lib.rs to check for existing mod declaration")?;
        for line in contents.lines() {
            if !line.trim().starts_with('/') && line.contains(module_name) {
                info!("lib.rs already contains {module_name} skipping update");
                return Ok(());
            }
        }
    }

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

    lib.write_all(format!("pub mod {module_name};").as_bytes())
        .context("write to lib.rs failed")?;
    Ok(())
}

pub(crate) fn write_file(module_name: &str, module_code: &str) -> anyhow::Result<()> {
    info!("Writing code to disk for module {module_name}");
    let path = PathBuf::from(format!("src/{module_name}.rs"));
    // This creates a TOCTOU but the window is small
    let did_file_already_exist = path.exists();
    if did_file_already_exist
        && !(does_user_confirm(format!("{path:?} already exists. Overwrite?"))?)
    {
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
    let lib_update_status = update_lib(module_name, did_file_already_exist);
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
