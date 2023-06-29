#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(unreachable_pub)]

// For use in external code
#[cfg(feature = "leet_env")]
mod leetcode_env;
#[cfg(feature = "leet_env")]
pub use leetcode_env::{
    list::{ListHead, ListNode},
    tree::{TreeNode, TreeRoot},
};

#[cfg(feature = "tool")]
mod tool;
#[cfg(feature = "tool")]
pub use crate::tool::{cli::CargoCli, core::run, log::init_logging};
