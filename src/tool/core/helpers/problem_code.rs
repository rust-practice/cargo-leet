use anyhow::{bail, Context};
use log::{debug, info, warn};
use regex::Regex;
use strum::IntoEnumIterator as _;

#[derive(Debug)]
pub(crate) struct ProblemCode {
    code: String,
    pub(crate) type_: ProblemType,
}

#[derive(Debug)]
pub(crate) enum ProblemType {
    NonDesign(FunctionInfo),
    Design,
}

impl ProblemType {
    /// Returns `true` if the problem type is [`NonDesign`].
    ///
    /// [`NonDesign`]: ProblemType::NonDesign
    #[must_use]
    pub(crate) const fn is_non_design(&self) -> bool {
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
        let result = Self { code, type_ };
        debug!("ProblemCode built: {result:#?}");
        Ok(result)
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
        let re = Regex::new(r"(?s)\n\s*pub fn ([a-z_0-9]*)\s*\((.*)\)(?: ?-> ?(.*))? \{")?;
        let Some(caps) = re.captures(code) else {
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

        let return_type: Option<FunctionArgType> = caps.get(3).map(|s| s.as_str().into());

        Ok(FunctionInfo {
            name,
            fn_args: args,
            return_type,
        })
    }

    pub(crate) fn has_tree(&self) -> bool {
        if let ProblemType::NonDesign(fn_info) = &self.type_ {
            fn_info.has_tree()
        } else {
            false
        }
    }

    pub(crate) fn has_list(&self) -> bool {
        if let ProblemType::NonDesign(fn_info) = &self.type_ {
            fn_info.has_list()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub(crate) struct FunctionInfo {
    pub(crate) name: String,
    fn_args: FunctionArgs,
    return_type: Option<FunctionArgType>,
}

impl FunctionInfo {
    pub(crate) fn get_args_with_case(&self) -> String {
        let mut result = String::from("#[case] ");
        result.push_str(&self.fn_args.raw_str.replace(',', ", #[case] "));

        if let Some(return_type) = self.return_type.as_ref() {
            result.push_str(&format!(", #[case] expected: {}", return_type.as_str()));
        }
        result
    }

    pub(crate) fn get_args_names(&self) -> String {
        let names: Vec<_> = self
            .fn_args
            .args
            .iter()
            .map(|arg| arg.identifier.clone())
            .collect();
        names.join(", ")
    }

    pub(crate) fn get_solution_comparison_code(&self) -> String {
        if matches!(&self.return_type, Some(FunctionArgType::F64)) {
            "assert!((actual - expected).abs() < 1e-5, \"Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.\");"
        } else {
            "assert_eq!(actual, expected);"
        }
            .to_string()
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
            result.push_str(&arg_type.apply(line));

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

    fn has_tree(&self) -> bool {
        self.fn_args.args.iter().any(|arg| arg.arg_type.is_tree())
    }

    fn has_list(&self) -> bool {
        self.fn_args.args.iter().any(|arg| arg.arg_type.is_list())
    }
}

#[derive(Debug)]
pub(crate) struct FunctionArg {
    identifier: String,
    arg_type: FunctionArgType,
}

#[derive(Debug)]
struct FunctionArgs {
    raw_str: String,
    args: Vec<FunctionArg>,
}

impl FunctionArgs {
    fn new(raw_str: String) -> anyhow::Result<Self> {
        let re = Regex::new(r"([A-Za-z_0-9]+?)\s*:\s*([A-Za-z0-9<>]*)")?;
        let caps: Vec<_> = re.captures_iter(&raw_str).collect();
        let mut args: Vec<FunctionArg> = vec![];
        for cap in caps {
            let identifier = cap.get(1).expect("Required to match").as_str().to_string();
            let arg_type = cap.get(2).expect("Required to match").as_str().into();

            args.push(FunctionArg {
                identifier,
                arg_type,
            });
        }

        Ok(Self { raw_str, args })
    }

    fn len(&self) -> usize {
        self.args.len()
    }
}

/// Function Arg Type (FAT)
#[derive(Debug, Eq, Hash, PartialEq, strum::EnumIter)]
enum FunctionArgType {
    // Search Key: SK_ADD_TYPE
    // Add type name to enum
    I32,
    I64,
    F64,
    Bool,
    String_,
    VecI32,
    VecF64,
    VecBool,
    VecString,
    VecVecI32,
    VecVecString,
    VecVecChar,
    List,
    Tree,
    Other { raw: String },
}

impl FunctionArgType {
    /// Applies any special changes needed to the value based on the type
    fn apply(&self, line: &str) -> String {
        debug!("Going to apply changes to argument input for {self:#?} to {line:?}");
        let result = match self {
            // Search Key: SK_ADD_TYPE
            // Add how string of type should be modified for code saved for user
            Self::String_ | Self::Bool => Ok(line.to_string()),
            Self::I32 => match line.parse::<i32>() {
                Ok(_) => Ok(line.to_string()),
                Err(e) => Err(format!(
                    "In testing the test input {line:?} the parsing to i32 failed with error: {e}"
                )),
            },
            Self::I64 => match line.parse::<i64>() {
                Ok(_) => Ok(line.to_string()),
                Err(e) => Err(format!(
                    "In testing the test input {line:?} the parsing to i64 failed with error: {e}"
                )),
            },
            Self::F64 => match line.parse::<f64>() {
                Ok(_) => Ok(line.to_string()),
                Err(e) => Err(format!(
                    "In testing the test input {line:?} the parsing to f64 failed with error: {e}"
                )),
            },
            Self::VecI32
            | Self::VecBool
            | Self::VecF64
            | Self::VecVecI32
            | Self::VecString
            | Self::VecVecString
            | Self::VecVecChar => {
                match Self::does_pass_basic_vec_tests(line) {
                    Ok(()) => {
                        let mut result = line.to_string();
                        if [Self::VecString, Self::VecVecString].contains(self) {
                            result = result.replace("\",", "\".into(),"); // Replace ones before end
                            result = result.replace("\"]", "\".into()]"); // Replace end
                        } else if self == &Self::VecVecChar {
                            result = result.replace('"', "'");
                        }
                        Ok(result.replace('[', "vec!["))
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Self::List => match Self::does_pass_basic_vec_tests(line) {
                Ok(()) => Ok(format!("ListHead::from(vec!{line}).into()")),
                Err(e) => Err(e.to_string()),
            },
            Self::Tree => match Self::does_pass_basic_vec_tests(line) {
                Ok(()) => Ok(format!("TreeRoot::from(\"{line}\").into()")),
                Err(e) => Err(e.to_string()),
            },
            Self::Other { raw: _ } => Ok(format!("todo!(\"{line}\")")),
        };
        result.unwrap_or_else(|e| {
            warn!("Type Mismatch? Type detected as '{self:?}' but got argument value of {line:?}. Error: {e}");
            format!("todo!({line:?})")
        })
    }

    fn does_pass_basic_vec_tests(s: &str) -> anyhow::Result<()> {
        if !s.starts_with('[') || !s.ends_with(']') {
            bail!("Expecting something that can be represented as a vec but got {s:?}");
        }
        Ok(())
    }

    const fn is_tree(&self) -> bool {
        matches!(self, Self::Tree)
    }

    const fn is_list(&self) -> bool {
        matches!(self, Self::List)
    }

    /// Returns `true` if the function arg type is [`Other`].
    ///
    /// [`Other`]: FunctionArgType::Other
    #[must_use]
    const fn is_other(&self) -> bool {
        matches!(self, Self::Other { .. })
    }

    fn as_str(&self) -> &str {
        match self {
            // Search Key: SK_ADD_TYPE
            // Add string that corresponds to each variant
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F64 => "f64",
            Self::Bool => "bool",
            Self::String_ => "String",
            Self::VecI32 => "Vec<i32>",
            Self::VecF64 => "Vec<f64>",
            Self::VecBool => "Vec<bool>",
            Self::VecString => "Vec<String>",
            Self::VecVecI32 => "Vec<Vec<i32>>",
            Self::VecVecString => "Vec<Vec<String>>",
            Self::VecVecChar => "Vec<Vec<char>>",
            Self::List => "Option<Box<ListNode>>",
            Self::Tree => "Option<Rc<RefCell<TreeNode>>>",
            Self::Other { raw } => raw,
        }
    }
}

impl From<&str> for FunctionArgType {
    fn from(value: &str) -> Self {
        let value = value.trim();
        // Loop over all variants and see if one matches
        for fat in Self::iter() {
            if !fat.is_other() && fat.as_str() == value {
                return fat;
            }
        }
        warn!("Unknown type {value:?} found please report this in an issue https://github.com/rust-practice/cargo-leet/issues/new?&labels=bug&template=missing_type.md");
        Self::Other {
            raw: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::IntoEnumIterator;

    use super::*;

    const fn get_100_same_tree() -> &'static str {
        "// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

    }
}
"
    }

    const fn get_97_interleaving_string() -> &'static str {
        "impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {

    }
}
"
    }

    const fn get_706_design_hashmap() -> &'static str {
        "struct MyHashMap {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {

    }

    fn put(&self, key: i32, value: i32) {

    }

    fn get(&self, key: i32) -> i32 {

    }

    fn remove(&self, key: i32) {

    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
"
    }

    const fn get_2_add_two_numbers() -> &'static str {
        "
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    }
}
"
    }

    #[test]
    fn has_tree_with_tree() {
        // Arrange / Act
        let problem_code: ProblemCode = get_100_same_tree()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(problem_code.has_tree());
    }

    #[test]
    fn has_tree_without_tree() {
        // Arrange / Act
        let problem_code: ProblemCode = get_97_interleaving_string()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(!problem_code.has_tree());
    }

    #[test]
    fn has_tree_design_question() {
        // Arrange / Act
        let problem_code: ProblemCode = get_706_design_hashmap()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(!problem_code.has_tree());
    }

    #[test]
    fn has_list_with_list() {
        // Arrange / Act
        let problem_code: ProblemCode = get_2_add_two_numbers()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(problem_code.has_list());
    }

    #[test]
    fn has_list_without_list() {
        // Arrange / Act
        let problem_code: ProblemCode = get_97_interleaving_string()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(!problem_code.has_list());
    }

    #[test]
    fn has_list_design_question() {
        // Arrange / Act
        let problem_code: ProblemCode = get_706_design_hashmap()
            .to_string()
            .try_into()
            .expect("Should be valid code");

        // Assert
        assert!(!problem_code.has_list());
    }

    #[test]
    fn get_args_with_case() {
        // Arrange / Act
        let fn_info = extract_function_info(get_97_interleaving_string());

        // Assert
        assert_eq!(
            fn_info.get_args_with_case(),
            "#[case] s1: String, #[case]  s2: String, #[case]  s3: String, #[case] expected: bool"
        );
    }

    #[test]
    fn get_args_names() {
        // Arrange / Act
        let fn_info = extract_function_info(get_97_interleaving_string());

        // Assert
        assert_eq!(fn_info.get_args_names(), "s1, s2, s3");
    }

    fn get_fn_info_min_sub_array_len() -> FunctionInfo {
        FunctionInfo {
            name: "min_sub_array_len".into(),
            fn_args: FunctionArgs {
                raw_str: "target: i32, nums: Vec<i32>".into(),
                args: vec![
                    FunctionArg {
                        identifier: "target".into(),
                        arg_type: FunctionArgType::I32,
                    },
                    FunctionArg {
                        identifier: "nums".into(),
                        arg_type: FunctionArgType::VecI32,
                    },
                ],
            },
            return_type: Some(FunctionArgType::I32),
        }
    }

    #[test]
    fn get_test_case_ok() {
        // Arrange
        let expected = "7, vec![2,3,1,2,4,3], todo!(\"Expected Result\")";
        let fn_info = get_fn_info_min_sub_array_len();
        let input = "7\n[2,3,1,2,4,3]";

        // Act
        let actual = fn_info.get_test_case(input).expect("Expected Ok");

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_test_case_invalid_num_args() {
        // Arrange
        let fn_info = get_fn_info_min_sub_array_len();
        let input = "[2,3,1,2,4,3]";

        // Act
        let actual = fn_info.get_test_case(input);

        // Assert
        assert!(actual.is_err());
    }

    const fn create_code_stub_all_arg_types_non_design() -> &'static str {
        // Search Key: SK_ADD_TYPE
        // Add a unique argument to the function to test retrieval of the unique
        // argument name (should match the lookup area)
        "
impl Solution {
    pub fn func_name(
        L2AC6p: i32,
        q7kv5k: i64,
        pP7GhC: f64,
        HFGzdD: bool,
        kjACSr: String,
        ePfFj3: Vec<i32>,
        kRubF2: Vec<f64>,
        ykyF5X: Vec<bool>,
        NkCeR6: Vec<String>,
        bBtcWe: Vec<Vec<i32>>,
        ndi4ny: Vec<Vec<String>>,
        ndi9ny: Vec<Vec<char>>,
        bJy3HH: Option<Box<ListNode>>,
        ndQLTu: Option<Rc<RefCell<TreeNode>>>,
        PRnJhw: UnknownType,
    ) {
    }
}
"
    }

    const fn fn_type_to_id(fat: &FunctionArgType) -> &'static str {
        match fat {
            // Search Key: SK_ADD_TYPE
            // Add the unique string value as an id (needs to match area where it is set)
            FunctionArgType::I32 => "L2AC6p",
            FunctionArgType::I64 => "q7kv5k",
            FunctionArgType::F64 => "pP7GhC",
            FunctionArgType::Bool => "HFGzdD",
            FunctionArgType::String_ => "kjACSr",
            FunctionArgType::VecI32 => "ePfFj3",
            FunctionArgType::VecF64 => "kRubF2",
            FunctionArgType::VecBool => "ykyF5X",
            FunctionArgType::VecString => "NkCeR6",
            FunctionArgType::VecVecI32 => "bBtcWe",
            FunctionArgType::VecVecString => "ndi4ny",
            FunctionArgType::VecVecChar => "ndi9ny",
            FunctionArgType::List => "bJy3HH",
            FunctionArgType::Tree => "ndQLTu",
            FunctionArgType::Other { .. } => "PRnJhw",
        }
    }

    fn extract_function_info(code: &str) -> FunctionInfo {
        let problem_code: ProblemCode = code.to_string().try_into().expect("Should be valid code");

        if let ProblemType::NonDesign(info) = problem_code.type_ {
            info
        } else {
            panic!("Expected Non Design Problem")
        }
    }

    #[test]
    fn function_parsing() {
        // Arrange
        let code = create_code_stub_all_arg_types_non_design();

        // Create hashset and fill with the possible argument types
        let mut left_to_see = HashSet::new();
        FunctionArgType::iter().for_each(|x| {
            left_to_see.insert(x);
        });

        // Add special handling for Other variant
        left_to_see.remove(&FunctionArgType::Other { raw: String::new() });
        left_to_see.insert(FunctionArgType::Other {
            raw: "UnknownType".to_string(),
        });

        // Act
        let fn_info = extract_function_info(code);

        // Assert
        assert_eq!(fn_info.name, "func_name");
        assert!(fn_info.return_type.is_none());
        for arg in &fn_info.fn_args.args {
            assert!(
                left_to_see.contains(&arg.arg_type),
                "Duplicate type seen. Each type should show up EXACTLY ONCE. Duplicate type: {}",
                arg.arg_type.as_str()
            );
            left_to_see.remove(&arg.arg_type);
            assert_eq!(
                arg.identifier,
                fn_type_to_id(&arg.arg_type),
                "ArgType: {}",
                arg.arg_type.as_str()
            );
        }
        assert!(
            left_to_see.is_empty(),
            "Expected all argument types to be seen but haven't seen {left_to_see:?}",
        );
    }

    #[test]
    fn function_arg_type_apply() {
        // Using an array instead of rstest because we need to ensure all inputs are
        // covered
        use FunctionArgType as FAT;
        let inputs = [
            // Search Key: SK_ADD_TYPE
            // Create a pair for the new type with a sample input from leetcode
            (FAT::I32, "1"),
            (FAT::I64, "2"),
            (FAT::F64, "2.00000"),
            (FAT::Bool, "true"),
            (FAT::String_, "\"leetcode\""),
            (FAT::VecI32, "[1,2,3,4]"),
            (FAT::VecF64, "[6.00000,0.50000,-1.00000,1.00000,-1.00000]"),
            (FAT::VecBool, "[true,false,false,false,false]"),
            (FAT::VecString, "[\"@..aA\",\"..B#.\",\"....b\"]"),
            (FAT::VecVecI32, "[[2,2,3],[7]]"),
            (
                FAT::VecVecString,
                "[[\"java\"],[\"nodejs\"],[\"nodejs\",\"reactjs\"]]",
            ),
            (
                FAT::VecVecChar,
                "[[\"X\",\".\",\".\",\"X\"],[\".\",\".\",\".\",\"X\"],[\".\",\".\",\".\",\"X\"]]",
            ),
            (FAT::List, "[1,2,4]"),
            (FAT::Tree, "[1,null,2,3]"),
            (FAT::Other { raw: String::new() }, "1"),
        ];

        // Create hashset and fill with the possible argument types
        let mut left_to_see = HashSet::new();
        FunctionArgType::iter().for_each(|x| {
            left_to_see.insert(x);
        });

        // Ensure each is there exactly once
        for (fat, _) in &inputs {
            assert!(
                left_to_see.contains(fat),
                "Duplicate type seen. Each type should show up EXACTLY ONCE. Duplicate type: {}",
                fat.as_str()
            );
            left_to_see.remove(fat);
        }
        assert!(
            left_to_see.is_empty(),
            "Expected all argument types to be seen but haven't seen {left_to_see:?}",
        );

        for (fat, input) in inputs {
            let expected = match fat {
                // Search Key: SK_ADD_TYPE
                // Add the expected output of the input set
                FAT::I32 => "1",
                FAT::I64 => "2",
                FAT::F64 => "2.00000",
                FAT::Bool => "true",
                FAT::String_ => "\"leetcode\"",
                FAT::VecI32 => "vec![1,2,3,4]",
                FAT::VecF64 => "vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]",
                FAT::VecBool => "vec![true,false,false,false,false]",
                FAT::VecString => "vec![\"@..aA\".into(),\"..B#.\".into(),\"....b\".into()]",
                FAT::VecVecI32 => "vec![vec![2,2,3],vec![7]]",
                FAT::VecVecString => {
                    "vec![vec![\"java\".into()],vec![\"nodejs\".into()],vec![\"nodejs\".into(),\"reactjs\".into()]]"
                }
                FAT::VecVecChar => { "vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','X']]" }
                FAT::List => "ListHead::from(vec![1,2,4]).into()",
                FAT::Tree => "TreeRoot::from(\"[1,null,2,3]\").into()",
                FAT::Other { raw: _ } => "todo!(\"1\")",
            };
            let actual = fat.apply(input);
            assert_eq!(actual, expected);
        }
    }
}
