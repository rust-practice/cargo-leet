#![forbid(unsafe_code)]

#[cfg(feature = "leet_env")]
mod leetcode_env;
#[cfg(feature = "tool")]
mod tool;

// For use in external code
#[cfg(feature = "leet_env")]
pub use leetcode_env::list::ListHead;
#[cfg(feature = "leet_env")]
pub use leetcode_env::list::ListNode;
#[cfg(feature = "leet_env")]
pub use leetcode_env::tree::TreeNode;
#[cfg(feature = "leet_env")]
pub use leetcode_env::tree::TreeRoot;

// For use in main.rs
#[cfg(feature = "tool")]
pub use crate::tool::cli::CargoCli;
#[cfg(feature = "tool")]
pub use crate::tool::core::run;
#[cfg(feature = "tool")]
pub use crate::tool::log::init_logging;
