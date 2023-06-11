use std::borrow::Cow;

use anyhow::bail;
use log::info;

use crate::{
    config::Config,
    core::{code_snippet, daily_challenge, write_file},
};

pub(crate) fn do_generate(args: &crate::cli::GenerateArgs) -> anyhow::Result<()> {
    assert!(
        args.daily_challenge ^ args.problem.is_some(),
        // This shouldn't happen, should be enforced by clap
        "Invalid state. Must either be daily challenge or specific problem but not both or none"
    );

    let title_slug: Cow<String> = if let Some(specific_problem) = &args.problem {
        // Problem specified
        // TODO: Parse url if given instead of slug
        if specific_problem.contains('/') {
            // Working with a url
            info!("Using '{specific_problem}' as a url");
            let slug = url_to_slug(specific_problem)?;
            info!("Extracted slug '{slug}' from url");
            Cow::Owned(slug)
        } else {
            // This is expected to be a valid slug
            info!("Using '{specific_problem}' as a slug");
            Cow::Borrowed(specific_problem)
        }
    } else {
        // Daily problem
        debug_assert!(args.daily_challenge);
        let slug = daily_challenge::get_daily_challenge_slug();
        info!("Slug for daily problem is: '{slug}'");
        Cow::Owned(slug)
    };

    let code_snippet = code_snippet::generate_code_snippet(&title_slug);
    write_file::write_file(&title_slug, code_snippet)?;
    Ok(())
}

fn url_to_slug(url: &str) -> anyhow::Result<String> {
    assert!(Config::LEETCODE_PROBLEM_URL.ends_with('/'));
    if !url.starts_with(Config::LEETCODE_PROBLEM_URL) {
        bail!(
            "Expected a leetcode url that starts with '{}' but got '{url}'",
            Config::LEETCODE_PROBLEM_URL
        )
    }
    let split_url: Vec<_> = url.split('/').collect();
    let split_prefix: Vec<_> = Config::LEETCODE_PROBLEM_URL.split('/').collect();
    debug_assert!(split_prefix.len() < split_url.len());
    Ok(split_url[split_prefix.len() - 1].to_string())
}
