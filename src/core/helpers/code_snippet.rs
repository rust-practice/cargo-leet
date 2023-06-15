use super::problem_code::ProblemCode;
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

pub fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<ProblemCode> {
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
        .context("Failed to convert response from json to codes_snippet")?;

    match code_snippets_res
        .code_snippets
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then_some(cs.code))
    {
        Some(result) => Ok(result.try_into()?),
        None => bail!("Rust not supported for this problem"),
    }
}
