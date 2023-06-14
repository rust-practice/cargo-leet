pub struct ProblemCode {
    code: String,
}

impl From<String> for ProblemCode {
    fn from(value: String) -> Self {
        Self { code: value }
    }
}

impl From<ProblemCode> for String {
    fn from(value: ProblemCode) -> Self {
        value.code
    }
}

impl AsRef<str> for ProblemCode {
    fn as_ref(&self) -> &str {
        &self.code
    }
}

impl ProblemCode {
    pub fn is_design(&self) -> bool {
        !self.code.starts_with("impl Solution {")
    }

    pub fn get_fn_info(&self) -> anyhow::Result<FunctionInfo> {
        todo!()
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub args: FunctionArgs,
    pub return_type: Option<FunctionArgType>,
}

impl FunctionInfo {
    pub fn get_args_with_case(&self) -> anyhow::Result<String> {
        todo!()
    }

    pub fn get_args_names(&self) -> anyhow::Result<String> {
        todo!()
    }

    pub(crate) fn get_test_case(&self, raw_str: &str) -> String {
        todo!()
    }
}

pub struct FunctionArgs {
    raw_str: String,
    pub args: Vec<FunctionArgType>,
}

impl FunctionArgs {
    pub fn new(raw_string: String) -> Self {
        todo!()
    }
}

/// Function Arg Type (FAT)
pub enum FunctionArgType {
    FATi32,
    FATVeci32,
    FATVecVeci32,
    FATString,
    FATList,
    FATTree,
}
