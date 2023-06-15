use std::fmt::Display;

use anyhow::{bail, Context};
use log::info;
use regex::Regex;

pub struct ProblemCode {
    code: String,
    pub type_: ProblemType,
}

pub enum ProblemType {
    NonDesign(FunctionInfo),
    Design,
}

impl ProblemType {
    /// Returns `true` if the problem type is [`NonDesign`].
    ///
    /// [`NonDesign`]: ProblemType::NonDesign
    #[must_use]
    pub fn is_non_design(&self) -> bool {
        matches!(self, Self::NonDesign(..))
    }
}

impl TryFrom<String> for ProblemCode {
    type Error = anyhow::Error;

    fn try_from(code: String) -> Result<Self, Self::Error> {
        let type_ = if Self::is_design(&code) {
            info!("Problem Type is Design");
            ProblemType::Design
        } else {
            info!("Problem Type is NonDesign");
            ProblemType::NonDesign(Self::get_fn_info(&code).context("Failed to get function info")?)
        };
        Ok(Self { code, type_ })
    }
}

impl AsRef<str> for ProblemCode {
    fn as_ref(&self) -> &str {
        &self.code
    }
}

impl ProblemCode {
    fn is_design(code: &str) -> bool {
        !code.contains("impl Solution {")
    }

    fn get_fn_info(code: &str) -> anyhow::Result<FunctionInfo> {
        let re = Regex::new(r#"\n\s*pub fn ([a-z_0-9]*)\((.*)\)(?: ?-> ?(.*))? \{"#)?;
        let caps = if let Some(caps) = re.captures(code) {
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
        })
        .context("Failed to parse function arguments")?;

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
            fn_args: args,
            return_type,
        })
    }

    pub fn has_tree(&self) -> bool {
        if let ProblemType::NonDesign(fn_info) = &self.type_ {
            fn_info.has_tree()
        } else {
            false
        }
    }

    pub fn has_list(&self) -> bool {
        if let ProblemType::NonDesign(fn_info) = &self.type_ {
            fn_info.has_list()
        } else {
            false
        }
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub fn_args: FunctionArgs,
    pub return_type: Option<FunctionArgType>,
}

impl FunctionInfo {
    pub fn get_args_with_case(&self) -> String {
        let mut result = String::from("#[case] ");
        for c in self.fn_args.raw_str.chars() {
            match c {
                ',' => result.push_str(", #[case] "),
                _ => result.push(c),
            }
        }

        if let Some(return_type) = self.return_type.as_ref() {
            result.push_str(&format!(", #[case] expected: {return_type}"))
        }
        result
    }

    pub fn get_args_names(&self) -> String {
        let names: Vec<_> = self
            .fn_args
            .args
            .iter()
            .map(|arg| arg.identifier.clone())
            .collect();
        names.join(", ")
    }

    pub fn get_test_case(&self, example_test_case_raw: &str) -> anyhow::Result<String> {
        let mut result = String::new();
        let n = self.fn_args.len();
        let lines: Vec<_> = example_test_case_raw.lines().collect();

        if lines.len() != self.fn_args.len() {
            bail!(
                "Expected number of augments ({}) to match the number of lines download ({})",
                self.fn_args.len(),
                lines.len()
            )
        }

        for (i, (&line, arg_type)) in lines
            .iter()
            .zip(self.fn_args.args.iter().map(|arg| &arg.arg_type))
            .enumerate()
        {
            result.push_str(
                &arg_type
                    .apply(line)
                    .context("Failed to apply type information to the example from leetcode")?,
            );

            if i < n - 1 {
                result.push_str(", ");
            }
        }

        // Include return type
        if self.return_type.is_some() {
            result.push_str(", todo!(\"Expected Result\")");
        }

        Ok(result)
    }

    pub fn has_tree(&self) -> bool {
        self.fn_args.args.iter().any(|arg| arg.arg_type.is_tree())
    }

    pub fn has_list(&self) -> bool {
        self.fn_args.args.iter().any(|arg| arg.arg_type.is_list())
    }
}

#[derive(Debug)]
pub struct FunctionArg {
    pub identifier: String,
    pub arg_type: FunctionArgType,
}

#[derive(Debug)]
pub struct FunctionArgs {
    raw_str: String,
    pub args: Vec<FunctionArg>,
}

impl FunctionArgs {
    pub fn new(raw_str: String) -> anyhow::Result<Self> {
        let re = Regex::new(r#"([a-z_0-9]*?)\s*:\s*([A-Za-z0-9<>]*)"#)?;
        let caps: Vec<_> = re.captures_iter(&raw_str).collect();
        let mut args: Vec<FunctionArg> = vec![];
        for cap in caps {
            let identifier = cap.get(1).expect("Required to match").as_str().to_string();
            let arg_type = cap
                .get(2)
                .expect("Required to match")
                .as_str()
                .try_into()
                .context("Failed to get argument type")?;

            args.push(FunctionArg {
                identifier,
                arg_type,
            })
        }

        Ok(Self { raw_str, args })
    }

    fn len(&self) -> usize {
        self.args.len()
    }
}

/// Function Arg Type (FAT)
#[derive(Debug)]
pub enum FunctionArgType {
    FATi32,
    FATVeci32,
    FATVecVeci32,
    FATString,
    FATList,
    FATTree,
}

impl FunctionArgType {
    /// Applies any special changes needed to the value based on the type
    fn apply(&self, line: &str) -> anyhow::Result<String> {
        Ok(match self {
            FunctionArgType::FATi32 => {
                let _: i32 = line.parse()?;
                line.to_string()
            }
            FunctionArgType::FATVeci32 => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("vec!{line}")
            }
            FunctionArgType::FATVecVeci32 => {
                Self::does_pass_basic_vec_tests(line)?;
                let mut result = String::new();
                for c in line.chars() {
                    match c {
                        '[' => result.push_str("vec!["),
                        _ => result.push(c),
                    }
                }
                result
            }
            FunctionArgType::FATString => line.to_string(),
            FunctionArgType::FATList => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("ListHead::from(\"{line}\").into()")
            }
            FunctionArgType::FATTree => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("TreeRoot::from(\"{line}\").into()")
            }
        })
    }

    fn does_pass_basic_vec_tests(s: &str) -> anyhow::Result<()> {
        if !s.starts_with('[') || !s.ends_with(']') {
            bail!("Expecting something that can be represented as a vec but got '{s}'");
        }
        Ok(())
    }

    fn is_tree(&self) -> bool {
        matches!(self, FunctionArgType::FATTree)
    }

    fn is_list(&self) -> bool {
        matches!(self, FunctionArgType::FATList)
    }
}

impl Display for FunctionArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FunctionArgType::FATi32 => "i32",
            FunctionArgType::FATVeci32 => "Vec<i32>",
            FunctionArgType::FATVecVeci32 => "Vec<Vec<i32>>",
            FunctionArgType::FATString => "String",
            FunctionArgType::FATList => "Option<Box<ListNode>>",
            FunctionArgType::FATTree => "Option<Rc<RefCell<TreeNode>>>",
        };

        write!(f, "{s}")
    }
}

impl TryFrom<&str> for FunctionArgType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use FunctionArgType::*;
        Ok(match value.trim() {
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
