use std::error::Error;

use cargo_leet::{code_snippet, daily_challenge, write_file};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let title_slug = if args.len() == 1 {
        daily_challenge::get_daily_challenge_slug()
    } else if args.len() != 2 {
        return Err("Usage: binary SLUG".into());
    } else {
        args.nth(1).unwrap()
    };
    let code_snippet = code_snippet::generate_code_snippet(&title_slug);
    write_file::write_file(&title_slug, code_snippet)?;
    Ok(())
}
