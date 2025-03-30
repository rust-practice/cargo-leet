use anyhow::Context;

use crate::tool::cli::NewArgs;

pub(crate) fn do_new(args: &NewArgs) -> anyhow::Result<()> {
    cargo_generate::generate(cargo_generate::GenerateArgs {
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

    Ok(())
}
