use super::super::problem_code::FunctionInfo;

use crate::tool::core::helpers::problem_code::ProblemType;

use log::info;

use super::super::problem_code::ProblemCode;

use anyhow::Context;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ProblemMetaDataResponse {
    data: Data,
}
impl ProblemMetaDataResponse {
    pub(crate) fn into_problem_metadata(self) -> anyhow::Result<ProblemMetadata> {
        let result = self.data.question;
        result.validate()?;
        Ok(result)
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Data {
    question: ProblemMetadata,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ProblemMetadata {
    #[serde(rename = "questionFrontendId")]
    id: String, // TODO Onè: Can't remember why this is a string, we seem to fail if it's not a u16 anyway
    #[serde(rename = "questionTitle")]
    title: String,
    #[serde(rename = "exampleTestcaseList")]
    example_test_case_list: Vec<String>,
}

impl ProblemMetadata {
    /// Checks if the data is valid
    pub(crate) fn validate(&self) -> anyhow::Result<()> {
        let _: u16 = self.get_id()?;
        Ok(())
    }

    pub(crate) fn get_id(&self) -> anyhow::Result<u16> {
        let result = self
            .id
            .parse()
            .with_context(|| format!("ID is not a valid u16. Got: {}", self.id))?;
        Ok(result)
    }

    pub(crate) fn get_num_and_title(&self) -> anyhow::Result<String> {
        Ok(format!("{}. {}", self.get_id()?, self.title))
    }

    pub(crate) fn get_test_cases(&self, problem_code: &ProblemCode) -> anyhow::Result<String> {
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

    pub(crate) fn get_test_cases_is_not_design(
        &self,
        fn_info: &FunctionInfo,
    ) -> anyhow::Result<String> {
        let mut result = "use rstest::rstest;

    #[rstest]
"
        .to_string();

        // Add test cases
        // explicit_iter_loop
        for example_test_case_raw in &self.example_test_case_list {
            let test_case = fn_info
                .get_test_case(example_test_case_raw)
                .context("Failed to convert downloaded test case into macro of input")?;
            // uninlined_format_args
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

        Ok(result)
    }

    #[allow(
        clippy::manual_string_new,
        clippy::unnecessary_wraps,
        clippy::unused_self
    )] // TODO Onè: implement question type from leetcode
       // see: https://leetcode.com/tag/design/
    pub(crate) fn get_test_cases_is_design(&self) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}
