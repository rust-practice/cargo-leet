use crate::{config::Config, core::helpers::problem_code::ProblemType};
use anyhow::Context;
use log::info;
use serde::Deserialize;
use serde_flat_path::flat_path;

use super::problem_code::{FunctionInfo, ProblemCode};

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

        let mut imports = String::new();

        let tests = match &problem_code.type_ {
            ProblemType::NonDesign(fn_info) => {
                if problem_code.has_tree() {
                    imports.push_str("use cargo_leet::TreeRoot;\n");
                }
                if problem_code.has_list() {
                    imports.push_str("use cargo_leet::ListHead;\n");
                }
                self.get_test_cases_is_not_design(fn_info)
                    .context("Failed to get test cases for non-design problem")?
            }
            ProblemType::Design => self
                .get_test_cases_is_design()
                .context("Failed to get test cases for design problem")?,
        };

        Ok(format!(
            r#"
#[cfg(test)]
mod tests {{
    use super::*;
    {imports}

    {tests}
}}
"#
        ))
    }

    fn get_test_cases_is_not_design(&self, fn_info: &FunctionInfo) -> anyhow::Result<String> {
        let mut result = "use rstest::rstest;

    #[rstest]
"
        .to_string();

        // Add test cases
        for example_test_case_raw in self.example_test_case_list.iter() {
            let test_case = fn_info
                .get_test_case(example_test_case_raw)
                .context("Failed to convert downloaded test case into macro of input")?;
            result.push_str(&format!("    #[case({})]\n", test_case))
        }

        // Add test case function body
        let test_fn = format!(
            "    fn case({}) {{
        let actual = Solution::{}({});
        assert_eq!(actual, expected);
    }}",
            fn_info.get_args_with_case(),
            fn_info.name,
            fn_info.get_args_names()
        );
        result.push_str(&test_fn);

        Ok(result)
    }

    fn get_test_cases_is_design(&self) -> anyhow::Result<String> {
        // TODO Create the test cases for design problems
        Ok("".to_string())
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
