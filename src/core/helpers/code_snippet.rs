use crate::config::Config;
use anyhow::{bail, Context};
use log::info;
use serde::Deserialize;
use serde_flat_path::flat_path;

#[flat_path]
#[derive(Deserialize)]
pub struct CodeSnippetResponse {
    #[flat_path("data.question.codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
}
#[derive(Deserialize)]
pub struct CodeSnippet {
    lang: String,
    code: String,
}

pub fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<String> {
    info!("Going to get code for {title_slug}");
    let code_snippets_res = ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
            "query": r#"query questionEditorData($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        codeSnippets {
                            lang
                            code
                        }
                    }
                }"#,
            "variables":{"titleSlug": title_slug},
            "operationName":"questionEditorData"
        }))
        .context("Get request for code_snippet failed")?
        .into_json::<CodeSnippetResponse>()
        .context("Failed to convert codes_snippet response from json")?;

    match code_snippets_res
        .code_snippets
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then_some(cs.code))
    {
        Some(result) => Ok(result),
        None => bail!("Rust not supported for this problem"),
    }
}

pub fn get_test_cases(title_slug: &str, is_design: bool) -> anyhow::Result<String> {
    info!("Going to get tests for {title_slug}");
    let tests = if is_design {
        r#"
            use rstest::rstest;
        "#
        .to_string()
    } else {
        // TODO: Get test cases for design problems
        "".to_string()
    };
    Ok(format!(
        r#"
        #[cfg(test)]
        mod tests {{
            use super::*;
            {tests}
        }}
    "#
    ))
}
