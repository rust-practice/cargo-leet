use std::borrow::Cow;

use anyhow::{bail, Context};
use log::info;

use crate::{
    config::Config,
    core::helpers::{code_snippet, daily_challenge, write_file},
};

pub(crate) fn do_generate(args: &crate::cli::GenerateArgs) -> anyhow::Result<()> {
    let title_slug: Cow<String> = if let Some(specific_problem) = &args.problem {
        // Problem specified
        if is_url(specific_problem) {
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
        let slug = daily_challenge::get_daily_challenge_slug()?;
        info!("Slug for daily problem is: '{slug}'");
        Cow::Owned(slug)
    };

    let code_snippet = code_snippet::generate_code_snippet(&title_slug)
        .context("Failed to generate code snippet")?;
    write_file::write_file(&title_slug, code_snippet).context("Failed to write files")?;
    Ok(())
}

/// Quick and dirty test to see if this is a url
fn is_url(value: &str) -> bool {
    value.contains('/')
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
