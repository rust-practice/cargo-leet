use std::fmt::Display;

use anyhow::{bail, Context};
use log::{error, info, warn};
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

    pub fn get_solution_comparison_code(&self) -> String {
        if let Some(FunctionArgType::F64) = &self.return_type {
            "assert!((actual - expected).abs() < 1e-5, \"Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.\");"
        } else {
            "assert_eq!(actual, expected);"
        }
        .to_string()
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
    I32,
    I64,
    F64,
    Bool,
    VecI32,
    VecF64,
    VecBool,
    VecVecI32,
    String_,
    List,
    Tree,
    Other { raw: String },
}

impl FunctionArgType {
    /// Applies any special changes needed to the value based on the type
    fn apply(&self, line: &str) -> anyhow::Result<String> {
        use FunctionArgType::*;
        Ok(match self {
            I32 => {
                if let Err(e) = line.parse::<i32>() {
                    warn!("In testing the test input \"{line}\" the parsing to i32 failed with error: {e}")
                };
                line.to_string()
            }
            I64 => {
                if let Err(e) = line.parse::<i64>() {
                    warn!("In testing the test input \"{line}\" the parsing to i64 failed with error: {e}")
                };
                line.to_string()
            }
            F64 => {
                if let Err(e) = line.parse::<f64>() {
                    warn!("In testing the test input \"{line}\" the parsing to f64 failed with error: {e}")
                };
                line.to_string()
            }
            VecI32 | VecBool | VecF64 => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("vec!{line}")
            }
            VecVecI32 => {
                Self::does_pass_basic_vec_tests(line)?;
                line.replace('[', "vec![")
            }
            String_ | Bool => line.to_string(),
            List => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("ListHead::from(vec!{line}).into()")
            }
            Tree => {
                Self::does_pass_basic_vec_tests(line)?;
                format!("TreeRoot::from(\"{line}\").into()")
            }
            Other { raw: _ } => line.to_string(), // Assume input is fine and pass on verbatim,
        })
    }

    fn does_pass_basic_vec_tests(s: &str) -> anyhow::Result<()> {
        if !s.starts_with('[') || !s.ends_with(']') {
            bail!("Expecting something that can be represented as a vec but got '{s}'");
        }
        Ok(())
    }

    fn is_tree(&self) -> bool {
        matches!(self, FunctionArgType::Tree)
    }

    fn is_list(&self) -> bool {
        matches!(self, FunctionArgType::List)
    }
}

impl Display for FunctionArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FunctionArgType::*;
        let s = match self {
            I32 => "i32",
            I64 => "i64",
            F64 => "f64",
            Bool => "bool",
            VecI32 => "Vec<i32>",
            VecF64 => "Vec<f64>",
            VecBool => "Vec<bool>",
            VecVecI32 => "Vec<Vec<i32>>",
            String_ => "String",
            List => "Option<Box<ListNode>>",
            Tree => "Option<Rc<RefCell<TreeNode>>>",
            Other { raw } => raw,
        };

        write!(f, "{s}")
    }
}

impl TryFrom<&str> for FunctionArgType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use FunctionArgType::*;
        Ok(match value.trim() {
            "i32" => I32,
            "i64" => I64,
            "f64" => F64,
            "bool" => Bool,
            "Vec<i32>" => VecI32,
            "Vec<f64>" => VecF64,
            "Vec<bool>" => VecBool,
            "Vec<Vec<i32>>" => VecVecI32,
            "String" => String_,
            "Option<Box<ListNode>>" => List,
            "Option<Rc<RefCell<TreeNode>>>" => Tree,
            trimmed_value => {
                error!("Unknown type \"{trimmed_value}\" found please report this in an issue https://github.com/rust-practice/cargo-leet/issues/new");
                Other {
                    raw: trimmed_value.to_string(),
                }
            }
        })
    }
}
