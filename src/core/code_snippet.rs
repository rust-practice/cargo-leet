use crate::config::Config;
use serde::Deserialize;
use serde_flat_path::flat_path;

// TODO: Add logging to all functions

#[flat_path]
#[derive(Deserialize)]
struct CodeSnippetResponse {
    #[flat_path("data.question.codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
}
#[derive(Deserialize)]
struct CodeSnippet {
    lang: String,
    code: String,
}

fn get_code_snippet_question(title_slug: &str) -> String {
    // TODO: Change return type to be a Result
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
        .unwrap()
        .into_json::<CodeSnippetResponse>()
        .unwrap();
    code_snippets_res
        .code_snippets
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then_some(cs.code))
        .unwrap()
}

fn get_test_cases(_title_slug: &str, is_design: bool) -> String {
    let tests = if is_design {
        r#"
            use rstest::rstest;
        "#
        .to_string()
    } else {
        // TODO: Get test cases for design problems
        "".to_string()
    };
    format!(
        r#"
        #[cfg(test)]
        mod tests {{
            use super::*;
            {tests}
        }}
    "#
    )
}

pub fn generate_code_snippet(title_slug: &str) -> String {
    // add URL
    let mut code_snippet = format!(
        "//! Solution for {}{title_slug}\n",
        Config::LEETCODE_PROBLEM_URL
    );

    // get code snippet
    let code = get_code_snippet_question(title_slug);
    code_snippet.push_str(&code);

    // handle non design snippets
    let is_design = code.starts_with("impl Solution {");
    if is_design {
        code_snippet.push_str("\npub struct Solution;\n")
    }

    // add tests
    let test = get_test_cases(title_slug, is_design);
    code_snippet.push_str(&test);
    code_snippet
}
