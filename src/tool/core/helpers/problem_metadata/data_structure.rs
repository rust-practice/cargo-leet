use super::super::problem_code::FunctionInfo;

use crate::tool::core::helpers::problem_code::ProblemType;

use anyhow::Context;
use log::info;

use super::super::problem_code::ProblemCode;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ProblemMetaDataResponse {
    data: Data,
}
impl ProblemMetaDataResponse {
    pub(crate) fn into_problem_metadata(self) -> anyhow::Result<ProblemMetadata> {
        self.data.question.try_into()
    }
}

#[derive(serde::Deserialize, Debug)]
struct Data {
    question: Question,
}

#[derive(serde::Deserialize, Debug)]
struct Question {
    #[serde(rename = "questionFrontendId")]
    id: String,
    #[serde(rename = "questionTitle")]
    title: String,
    #[serde(rename = "exampleTestcaseList")]
    example_test_case_list: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct ProblemMetadata {
    pub(crate) id: u16,
    title: String,
    example_test_case_list: Vec<String>,
}

impl TryFrom<Question> for ProblemMetadata {
    type Error = anyhow::Error;

    fn try_from(value: Question) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.parse().context("failed to parse id")?,
            title: value.title,
            example_test_case_list: value.example_test_case_list,
        })
    }
}

impl ProblemMetadata {
    pub(crate) fn get_num_and_title(&self) -> String {
        format!("{}. {}", self.id, self.title)
    }

    pub(crate) fn get_test_cases(&self, problem_code: &ProblemCode) -> String {
        info!("Going to get tests");

        let mut imports = String::new();

        let tests = match &problem_code.type_ {
            ProblemType::NonDesign(fn_info) => {
                // Add imports
                if problem_code.has_tree() {
                    imports.push_str("use cargo_leet::TreeRoot;\n");
                }
                if problem_code.has_list() {
                    imports.push_str("use cargo_leet::ListHead;\n");
                }

                // Add actual test cases
                self.get_test_cases_is_not_design(fn_info)
            }
            ProblemType::Design => self.get_test_cases_is_design(),
        };

        format!(
            "
#[cfg(test)]
mod tests {{
    use super::*;
    {imports}

    {tests}
}}
"
        )
    }

    fn get_test_cases_is_not_design(&self, fn_info: &FunctionInfo) -> String {
        let mut result = "use rstest::rstest;

    #[rstest]
"
        .to_string();

        // Add test cases
        for example_test_case_raw in &self.example_test_case_list {
            let test_case = fn_info.get_test_case(example_test_case_raw);
            result.push_str(&format!("    #[case({test_case})]\n"));
        }

        // Add test case function body
        let test_fn = format!(
            "    fn case({}) {{
        let actual = Solution::{}({});
        {}
    }}",
            fn_info.get_args_with_case(),
            fn_info.name,
            fn_info.get_args_names(),
            fn_info.get_solution_comparison_code(),
        );
        result.push_str(&test_fn);

        result
    }

    #[allow(clippy::unused_self)] // TODO Onè: implement question type from leetcode
                                  // see: https://leetcode.com/tag/design/
    fn get_test_cases_is_design(&self) -> String {
        String::new()
    }
}
