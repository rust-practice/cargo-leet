use crate::cli::Cli;

mod code_snippet;
mod daily_challenge;
mod write_file;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    // TODO: Check for Cargo.toml to ensure we are in a valid folder
    // let mut args = std::env::args();
    // let title_slug = if args.len() == 1 {
    //     daily_challenge::get_daily_challenge_slug()
    // } else if args.len() != 2 {
    //     return Err("Usage: binary SLUG".into());
    // } else {
    //     args.nth(1).unwrap()
    // };
    // let code_snippet = code_snippet::generate_code_snippet(&title_slug);
    // write_file::write_file(&title_slug, code_snippet)?;
    Ok(())
}
