use super::super::problem_code::FunctionInfo;

use crate::tool::core::helpers::{
    problem_code::ProblemType, problem_description::data_structure::ProblemDescription,
};

use anyhow::Context;
use log::{error, info};

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

    pub(crate) fn get_test_cases(
        &self,
        problem_code: &ProblemCode,
        description: &ProblemDescription,
    ) -> String {
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
                let solutions = description.get_solutions();
                self.get_test_cases_is_not_design(fn_info, solutions)
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

    fn get_test_cases_is_not_design(
        &self,
        fn_info: &FunctionInfo,
        mut solutions: Vec<String>,
    ) -> String {
        let mut result = "use rstest::rstest;

    #[rstest]
"
        .to_string();

        if solutions.len() != self.example_test_case_list.len() {
            error!(
                "Number of solutions ({}) does not match the number of test cases ({}). Falling back to no solutions. Solutions were: {solutions:?}", 
                solutions.len(),
                self.example_test_case_list.len()
            );
            solutions = self
                .example_test_case_list
                .iter()
                .map(|_| "todo!(\"Failed to get solutions\"".to_string())
                .collect();
        }
        assert_eq!(solutions.len(), self.example_test_case_list.len());

        // Add test cases
        for (example_test_case_raw, solution) in self.example_test_case_list.iter().zip(solutions) {
            let test_case = fn_info.get_test_case(example_test_case_raw, &solution);
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

    #[allow(clippy::unused_self, clippy::missing_const_for_fn)] // TODO Onè: implement question type from leetcode
                                                                // see: https://leetcode.com/tag/design/
    fn get_test_cases_is_design(&self) -> String {
        String::new()
    }
}
