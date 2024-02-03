use super::problem_code::ProblemCode;
use crate::tool::config::Config;
use anyhow::{bail, Context};
use log::info;
use regex::Regex;

type CodeSnippets = Vec<CodeSnippet>;

#[derive(serde::Deserialize)]
struct CodeSnippetResponse {
    data: Data,
}
impl CodeSnippetResponse {
    fn into_rust_problem_code(self) -> anyhow::Result<ProblemCode> {
        let code_snippet = self.data.question.code_snippets;
        let Some(mut result) = code_snippet
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
}

#[derive(serde::Deserialize)]
struct Data {
    question: Question,
}

#[derive(serde::Deserialize)]
struct Question {
    #[serde(rename = "codeSnippet")] // TODO Onè: Fix error created (missing an s)
    code_snippets: CodeSnippets,
}

#[derive(serde::Deserialize)]
struct CodeSnippet {
    lang: String,
    code: String,
}

pub(crate) fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<ProblemCode> {
    get_code_snippets_response(title_slug)?.into_rust_problem_code()
}

fn get_code_snippets_response(title_slug: &str) -> anyhow::Result<CodeSnippetResponse> {
    let json = if cfg!(test) {
        local_store_request_code_snippet(title_slug)
    } else {
        external_request_code_snippet(title_slug)
    }?;
    let result = serde_json::from_str(&json)
        .context("failed to convert from String to CodeSnippetResponse as json")?;
    Ok(result)
}

fn local_store_request_code_snippet(title_slug: &str) -> anyhow::Result<String> {
    let path = super::local_store::path_local_store_code_snippet().join(title_slug);
    std::fs::read_to_string(&path).with_context(|| format!("failed to read string from {path:?}"))
}

fn external_request_code_snippet(title_slug: &str) -> anyhow::Result<String> {
    info!("[External] Going to send request for code for problem with title: {title_slug}");
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
        .context("failed to get request for code_snippet failed")?
        .into_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::tool::core::helpers::local_store::tests::{title_slugs, SlugList};

    // TODO Onè: Create an ignored test to download the data for testing

    #[rstest]
    fn conversion_from_leetcode_response(title_slugs: SlugList) {
        // TODO Onè: Implement test using locally stored data
        for title in title_slugs {
            dbg!(title);
        }

        todo!("Onè")
    }
}
