#![forbid(unsafe_code)]

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
