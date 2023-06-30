#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(unreachable_pub)]

//! The main aim of **cargo-leet** is to make it easier to develop solutions to
//! leetcode problems locally on your machine. And as such it is composed of two
//! parts, a tool to help with code download and testing setup, and helper code
//! to allow running solutions developed locally.
//!
//! ### Tool
//!
//! The **cargo-leet** subcommand is a command line tool developed with clap and
//! the associated help is probably the best way to get an idea of how to use
//! the tool. Screenshots of the help can be found in the
//! [readme](https://github.com/rust-practice/cargo-leet#screenshots) on github.
//! For the sake of maintainability features added will be documented there
//! instead of always needing to update multiple places.
//!
//! ### Leetcode Environment Support
//!
//! **cargo-leet** also includes helper code with structs and traits to simulate
//! the environment that you code would run in on the leetcode servers so that
//! you are able to run tests on your code locally. It also provides a few extra
//! types that facilitate testing especially as it relates to creating test
//! cases from the text provided by leetcode.
//!
//! ## Feature flags
//!
//! **cargo-leet** uses feature flags to control which code gets compiled based
//! on how the create is being used. This is especially important for the code
//! imported in the solution repository as this repo may be using a much older
//! version of the rust toolchain due to the fact that leetcode uses a much
//! older version on their servers and some users may want to use the same
//! version to ensure their code will always work upon upload. However, because
//! it is such an old version many of the creates used in the development of the
//! tool are not able to be compiled with that toolchain and as such they being
//! only compiled behind a feature flag makes that a non-issue. It also allows
//! users to not compile the code only needed to support leetcode solution
//! development when working on or using the tool.
//!
//! - `default`: Enables the `leet_env` feature as this is the most common use
//!   case
//! - `leet_env`: Includes the code for working on leetcode problem solutions
//! - `tool`: Enables the code and dependencies used to create the tool.

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
