#![allow(clippy::transmute_ptr_to_ptr)] // clippy::transmute_ptr_to_ptr occurs in the macro #[flat_path] which we can't
                                        // control

use super::problem_code::ProblemCode;
use crate::tool::config::Config;
use anyhow::{bail, Context};
use log::info;
use regex::Regex;
use serde_flat_path::flat_path;

#[flat_path]
#[derive(serde::Deserialize)]
struct CodeSnippetResponse {
    #[flat_path("data.question.codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
}

#[derive(serde::Deserialize)]
struct CodeSnippet {
    lang: String,
    code: String,
}

pub(crate) fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<ProblemCode> {
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

    let Some(mut result) = code_snippets_res
        .code_snippets
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then_some(cs.code))
    else {
        bail!("Rust not supported for this problem")
    };

    // Add todo!() placeholders in function bodies
    let re = Regex::new(r#"\{\s*\}"#)?;
    let re = Regex::new(r"\{\s*\}")?;
    result = re
        .replace_all(&result, "{ todo!(\"Fill in body\") }")
        .to_string();

    result.try_into()
}
