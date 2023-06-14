use crate::config::Config;
use anyhow::Context;
use log::info;
use serde::Deserialize;
use serde_flat_path::flat_path;

use super::problem_code::ProblemCode;

/// This struct is only used because there are two fields that we are interested in that start with the same path and flat_path does not support that yet
#[flat_path]
#[derive(Deserialize, Debug)]
struct QuestionWrapper {
    #[flat_path("data.question")]
    inner: ProblemMetadata,
}

#[flat_path]
#[derive(Deserialize, Debug)]
pub struct ProblemMetadata {
    #[serde(rename = "questionFrontendId")]
    id: String,
    #[serde(rename = "exampleTestcaseList")]
    example_test_case_list: Vec<String>,
}

impl ProblemMetadata {
    /// Checks if the data is valid
    fn validate(&self) -> anyhow::Result<()> {
        let _: u16 = self.get_id()?;
        Ok(())
    }

    pub fn get_id(&self) -> anyhow::Result<u16> {
        let result = self
            .id
            .parse()
            .with_context(|| format!("ID is not a valid u16. Got: {}", self.id))?;
        Ok(result)
    }

    pub fn get_test_cases(&self, problem_code: &ProblemCode) -> anyhow::Result<String> {
        info!("Going to get tests");
        // TODO implement generation of test cases
        let tests = if problem_code.is_design() {
            r#"
use rstest::rstest;
"#
            .to_string()
        } else {
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
}

pub fn get_problem_metadata(title_slug: &str) -> anyhow::Result<ProblemMetadata> {
    info!("Going to get problem metadata");
    let QuestionWrapper { inner: result } = ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
            "query": r#"query consolePanelConfig($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                questionFrontendId
                exampleTestcaseList
            }
        }"#,
            "variables":{"titleSlug": title_slug},
            "operationName":"consolePanelConfig"
        }))
        .context("Get request for problem metadata failed")?
        .into_json()
        .context("Failed to convert response from json to problem metadata")?;

    result
        .validate()
        .context("Failed to validate problem metadata")?;
    Ok(result)
}
