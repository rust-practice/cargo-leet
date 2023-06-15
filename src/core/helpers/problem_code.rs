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
}

pub struct FunctionInfo {
    pub name: String,
    pub fn_args: FunctionArgs,
    pub return_type: Option<FunctionArgType>,
}

impl FunctionInfo {
    pub fn get_args_with_case(&self) -> anyhow::Result<String> {
        todo!()
    }

    pub fn get_args_names(&self) -> anyhow::Result<String> {
        todo!()
    }

    pub(crate) fn get_test_case(&self, example_test_case_raw: &str) -> anyhow::Result<String> {
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
            result.push_str(", todo!(\"return type\")");
        }

        Ok(result)
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
                // TODO finish converting input into correct type
                format!("\"{line}\"")
            }
            FunctionArgType::FATTree => {
                Self::does_pass_basic_vec_tests(line)?;
                // TODO finish converting input into correct type
                format!("\"{line}\"")
            }
        })
    }

    fn does_pass_basic_vec_tests(s: &str) -> anyhow::Result<()> {
        if !s.starts_with('[') || !s.ends_with(']') {
            bail!("Expecting something that can be represented as a vec but got '{s}'");
        }
        Ok(())
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
