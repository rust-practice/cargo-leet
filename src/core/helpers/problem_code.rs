use anyhow::{bail, Context};
use regex::Regex;

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
        let re = Regex::new(r#"pub fn ([a-z_0-9]*)\((.*)\)(?: ?-> ?(.*))? \{"#)?;
        let caps = if let Some(caps) = re.captures(&self.code) {
            caps
        } else {
            bail!("Regex failed to match");
        };

        let name = if let Some(name) = caps.get(1) {
            name.as_str().to_string()
        } else {
            bail!("Function name not found in code")
        };

        let args = FunctionArgs::new(if let Some(args) = caps.get(2) {
            args.as_str().to_string()
        } else {
            bail!("Function arguments not matched")
        });

        let return_type: Option<FunctionArgType> = match caps.get(3) {
            Some(s) => Some(
                s.as_str()
                    .try_into()
                    .context("Failed to convert return type")?,
            ),
            None => None,
        };

        Ok(FunctionInfo {
            name,
            args,
            return_type,
        })
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

impl TryFrom<&str> for FunctionArgType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use FunctionArgType::*;
        Ok(match value {
            "i32" => FATi32,
            "Vec<i32>" => FATVeci32,
            "Vec<Vec<i32>>" => FATVecVeci32,
            "String" => FATString,
            "Option<Box<ListNode>>" => FATList,
            "Option<Rc<RefCell<TreeNode>>>" => FATTree,
            _ => bail!("Unknown type: '{value}'"),
        })
    }
}
