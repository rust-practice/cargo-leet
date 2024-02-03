use anyhow::{bail, Context};
use convert_case::{Case, Casing};
use log::info;
use std::borrow::Cow;

use crate::tool::{
    cli,
    config::Config,
    core::helpers::{
        code_snippet::get_code_snippet_for_problem, daily_challenge,
        problem_metadata::get_problem_metadata, write_to_disk,
    },
};

pub(crate) fn do_generate(args: &cli::GenerateArgs) -> anyhow::Result<()> {
    let title_slug: Cow<String> = if let Some(specific_problem) = &args.problem {
        get_slug_from_args(specific_problem)
            .with_context(|| format!("expected URL or slug but got {specific_problem}"))?
    } else {
        // Daily problem
        let slug = daily_challenge::get_daily_challenge_slug()?;
        info!("Slug for daily problem is: '{slug}'");
        Cow::Owned(slug)
    };

    let (module_name, module_code) = create_module_code(&title_slug, args).with_context(|| {
        format!("failed to generate the name and module code for {title_slug:?}")
    })?;
    write_to_disk::write_file(&module_name, &module_code).context("failed to write to disk")?;
    println!("Generated module: {module_name}");
    Ok(())
}

fn get_slug_from_args(specific_problem: &String) -> anyhow::Result<Cow<'_, String>> {
    Ok(if is_url(specific_problem) {
        // Working with a url
        info!("Using '{specific_problem}' as a url");
        let slug = url_to_slug(specific_problem)?;
        info!("Extracted slug '{slug}' from url");
        Cow::Owned(slug)
    } else {
        // This is expected to be a valid slug
        info!("Using '{specific_problem}' as a slug");
        Cow::Borrowed(specific_problem)
    })
}

/// Gets the code and other data from leetcode and generates the suitable code
/// for the module and the name of the module Returns the module name and the
/// module code
///
/// NB: Did not return `Cow` because `module_name` is always a modified version
/// of the input
fn create_module_code(
    title_slug: &str,
    args: &cli::GenerateArgs,
) -> anyhow::Result<(String, String)> {
    info!("Building module contents for {title_slug}");

    let meta_data =
        get_problem_metadata(title_slug).context("failed to retrieve problem meta data")?;

    // Add problem URL
    let mut code_snippet = format!(
        "//! Solution for {}{title_slug}\n",
        Config::LEETCODE_PROBLEM_URL
    );

    // Add problem number and title
    code_snippet.push_str(&format!("//! {}\n", meta_data.get_num_and_title()));

    // Add blank line between docstring and code
    code_snippet.push('\n');

    // Get code snippet
    let problem_code = get_code_snippet_for_problem(title_slug)?;
    code_snippet.push_str(problem_code.as_ref());

    code_snippet.push_str(
        "\n\n// << ---------------- Code below here is only for local use ---------------- >>\n",
    );

    // Add struct for non design questions
    if problem_code.type_.is_non_design() {
        code_snippet.push_str("\npub struct Solution;\n");
    }

    // Add leet code types
    if problem_code.has_tree() {
        code_snippet.push_str("use cargo_leet::TreeNode;\n");
    }
    if problem_code.has_list() {
        code_snippet.push_str("use cargo_leet::ListNode;\n");
    }

    // Add tests
    let tests = meta_data.get_test_cases(&problem_code);
    code_snippet.push_str(&tests);

    // Set module name
    let module_name = if args.should_include_problem_number {
        info!("Including problem number in module name");
        format!("_{}_{}", meta_data.id, title_slug.to_case(Case::Snake))
    } else {
        info!("Using snake case slug for module name");
        title_slug.to_case(Case::Snake)
    };

    Ok((module_name, code_snippet))
}

/// Quick and dirty test to see if this is a url
/// Uses a character that is not allowed in slugs but must be in a url to decide
/// between the two
fn is_url(value: &str) -> bool {
    value.contains('/')
}

fn url_to_slug(url: &str) -> anyhow::Result<String> {
    debug_assert!(Config::LEETCODE_PROBLEM_URL.ends_with('/'));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_in_slug_out() {
        let slug = "two-sum".to_string();
        let actual = get_slug_from_args(&slug).expect("Expect value to be valid");
        assert_eq!(actual.to_string(), slug);
    }

    #[test]
    fn url_in_slug_out() {
        let url = "https://leetcode.com/problems/two-sum/".to_string();
        let expected = "two-sum";
        let actual = get_slug_from_args(&url).expect("Expect value to be valid");
        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    fn invalid_url() {
        // Missing "s" in https
        let url = "http://leetcode.com/problems/two-sum/".to_string();
        let actual = get_slug_from_args(&url);
        assert!(actual.is_err());
    }
}
