use crate::tool::cli::NewArgs;
use crate::tool::config_file::ConfigFile;
use anyhow::Context;
use log::info;
use std::io::{Write, stdin, stdout};

pub(crate) fn do_new(args: &NewArgs) -> anyhow::Result<()> {
    let project_dir = cargo_generate::generate(cargo_generate::GenerateArgs {
        template_path: cargo_generate::TemplatePath {
            auto_path: None,
            subfolder: None,
            test: false,
            git: Some("https://github.com/rust-practice/cargo-leet-template.git".to_owned()),
            branch: None,
            tag: None,
            revision: None,
            path: None,
            favorite: None,
        },
        list_favorites: false,
        name: args.name.clone(),
        force: false,
        verbose: true,
        template_values_file: None,
        silent: false,
        config: None,
        vcs: None,
        lib: true,
        bin: false,
        ssh_identity: None,
        define: vec![],
        init: false,
        destination: None,
        force_git_init: true,
        allow_commands: false,
        overwrite: false,
        skip_submodules: true,
        other_args: None,
        quiet: false,
        continue_on_error: true,
        gitconfig: None,
    })
    .context("failed to generate cargo project")?;

    // interactively set config
    /* todo set config in cargo_generate instead (using a template variable?).
        I think by setting `GenerateArgs.define` or `GenerateArgs.other_args`.
    */
    std::env::set_current_dir(project_dir)?;

    print!("would you like to include the number of each problem in it's name? [y/N]: ");
    stdout().flush()?;
    let mut response = String::new();
    stdin().read_line(&mut response)?;

    let parsed_response = match response.trim().to_ascii_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => todo!(),
    };

    let mut config = ConfigFile::default();

    config.number_in_name = parsed_response;
    info!("input treated as {}. Saving...", config.number_in_name);
    config
        .save()
        .context("failed to save user preference in config file")?;

    Ok(())
}
