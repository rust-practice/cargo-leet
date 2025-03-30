use super::{get_response, local_store::path_local_store_code_snippet, problem_code::ProblemCode};
use crate::tool::config::Config;
use anyhow::{Context, bail};
use log::info;
use regex::Regex;

type CodeSnippets = Vec<CodeSnippet>;

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
struct Data {
    question: Question,
}

#[derive(serde::Deserialize, Debug)]
struct Question {
    #[serde(rename = "codeSnippets")]
    code_snippets: CodeSnippets,
}

#[derive(serde::Deserialize, Debug)]
struct CodeSnippet {
    lang: String,
    code: String,
}

pub(crate) fn get_code_snippet_for_problem(title_slug: &str) -> anyhow::Result<ProblemCode> {
    info!("Attempting to get code snippet for {title_slug:?}");
    get_code_snippets_response(title_slug)?.into_rust_problem_code()
}

fn get_code_snippets_response(title_slug: &str) -> anyhow::Result<CodeSnippetResponse> {
    get_response(
        title_slug,
        local_store_request_code_snippet,
        external_request_code_snippet,
    )
}

fn local_store_request_code_snippet(title_slug: &str) -> anyhow::Result<String> {
    let path = path_local_store_code_snippet(title_slug);
    std::fs::read_to_string(&path).with_context(|| format!("failed to read string from {path:?}"))
}

fn external_request_code_snippet(title_slug: &str) -> anyhow::Result<String> {
    info!("[External] Going to send request for code for problem with title: {title_slug}");
    ureq::post(Config::LEETCODE_GRAPH_QL)
        .send_json(serde_json::json!({
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
        .body_mut()
        .read_to_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use anyhow::Context;
    use rstest::rstest;

    use crate::tool::core::helpers::{
        code_snippet::{external_request_code_snippet, get_code_snippets_response},
        local_store::tests::{SlugList, get_rnd_request_delay, insta_settings, title_slugs},
    };

    #[rstest]
    #[ignore = "Only use for downloading responses"]
    fn download_response_from_leetcode(title_slugs: SlugList) {
        for title_slug in title_slugs {
            let sleep_delay = std::time::Duration::from_millis(get_rnd_request_delay());
            println!(
                "Going to sleep for {} milliseconds before requesting and trying to save {title_slug}",
                sleep_delay.as_millis()
            );
            std::thread::sleep(sleep_delay); // Sleep to not go too hard on leetcode API
            let response_string = external_request_code_snippet(title_slug).unwrap();
            let path = super::path_local_store_code_snippet(title_slug);
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
                .with_context(|| format!("failed to save response to {path:?}"))
                .unwrap();
            file.write_all(response_string.as_bytes()).unwrap();
            println!("Save of {title_slug:?} completed\n");
        }
        println!("Successfully saved all responses");
    }

    #[rstest]
    fn conversion_from_leetcode_response(title_slugs: SlugList, insta_settings: insta::Settings) {
        for title_slug in title_slugs {
            insta_settings.bind(|| {
                insta::assert_debug_snapshot!(
                    format!("response {title_slug}"),
                    get_code_snippets_response(title_slug).unwrap()
                );
            });
        }
    }
}
