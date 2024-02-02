use super::problem_code::ProblemCode;
use crate::tool::config::Config;
use anyhow::{bail, Context};
use log::info;
use regex::Regex;

type CodeSnippets = Vec<CodeSnippet>;
#[derive(serde::Deserialize)]
struct Question {
    #[serde(rename = "codeSnippet")] // TODO cw: Fix error created (missing an s)
    code_snippets: CodeSnippets,
}

#[derive(serde::Deserialize)]
struct CodeSnippetResponse {
    data: Data,
}
impl CodeSnippetResponse {
    /// Consumes self and returns the snippets
    ///
    /// Was only being called once and couldn't justify allocating a new string
    /// when this one wasn't needed anymore
    fn into_code_snippets(self) -> CodeSnippets {
        self.data.question.code_snippets
    }
}
#[derive(serde::Deserialize)]
struct Data {
    question: Question,
}

#[derive(serde::Deserialize)]
struct CodeSnippet {
    lang: String,
    code: String,
}

pub(crate) fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<ProblemCode> {
    let request_code_snippet = request_code_snippet(title_slug)?;

    // Deserialize from Response to Struct
    let code_snippets_res = request_code_snippet
        .into_json::<CodeSnippetResponse>()
        .context("Failed to convert response from json to codes_snippet")?;

    extract_rust_code(code_snippets_res)
}

fn extract_rust_code(code_snippets_res: CodeSnippetResponse) -> anyhow::Result<ProblemCode> {
    let Some(mut result) = code_snippets_res
        .into_code_snippets()
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then_some(cs.code))
    else {
        bail!("Rust not supported for this problem")
    };

    // Add todo!() placeholders in function bodies
    let re = Regex::new(r"\{\s*\}")?;
    result = re
        .replace_all(&result, r#"{ todo!("Fill in body") }"#)
        .to_string();

    result.try_into()
}

fn request_code_snippet(title_slug: &str) -> Result<ureq::Response, anyhow::Error> {
    info!("Going to send request for code for problem with title: {title_slug}");
    ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
            "query": "query questionEditorData($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        codeSnippets {
                            lang
                            code
                        }
                    }
                }",
            "variables":{"titleSlug": title_slug},
            "operationName":"questionEditorData"
        }))
        .context("Get request for code_snippet failed")
}

#[cfg(test)]
mod tests {
    // use super::*;

    use std::io::Write as _;

    use crate::tool::core::helpers::code_snippet::request_code_snippet;

    #[test]
    fn test_name() {
        let slug = "two-sum";
        let response = request_code_snippet(slug).unwrap();
        let s = response.into_string().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("output.json")
            .unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }
}
