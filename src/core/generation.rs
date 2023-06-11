use std::borrow::Cow;

use crate::core::{code_snippet, daily_challenge, write_file};

// TODO: Add logging to all functions

pub(crate) fn do_generate(args: &crate::cli::GenerateArgs) -> anyhow::Result<()> {
    assert!(
        args.daily_challenge ^ args.problem.is_some(),
        "Invalid state. Must either be daily challenge or specific problem but not both or none"
    );

    let title_slug: Cow<String> = if let Some(specific_problem) = &args.problem {
        // Problem specified
        // TODO: Parse url if given instead of slug
        Cow::Borrowed(specific_problem)
    } else {
        // Daily problem
        debug_assert!(args.daily_challenge);
        Cow::Owned(daily_challenge::get_daily_challenge_slug())
    };

    let code_snippet = code_snippet::generate_code_snippet(&title_slug);
    write_file::write_file(&title_slug, code_snippet)?;
    Ok(())
}
