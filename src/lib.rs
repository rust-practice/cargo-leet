#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)] // these might be false positives, so we need to check these on a case-by-case basis
#![allow(clippy::redundant_pub_crate)] // this lint is giving too many false positives

//! The main aim of **cargo-leet** is to make it easier to develop solutions to
//! leetcode problems locally on your machine. And as such it is composed of two
//! parts, a tool to help with code download and testing setup, and helper code
//! to allow running solutions developed locally.
//!
//! ### Tool
//!
//! The **cargo-leet** subcommand is a command line tool developed with clap and
//! the associated help is probably the best way to get an idea of how to use
//! the tool. Help messages can be found in the
//! [readme](https://github.com/rust-practice/cargo-leet#Help#Messages) on GitHub.
//! For the sake of maintainability features added will be documented there
//! instead of always needing to update multiple places.
//!
//! ### Leetcode Environment Support
//!
//! **cargo-leet** also includes helper code with structs and traits to simulate
//! the environment that your code would run in on the leetcode servers so that
//! you are able to run tests on your code locally. It also provides a few extra
//! types that facilitate testing especially as it relates to creating test
//! cases from the text provided by leetcode.
//!
//! ## Feature flags
//! **cargo-leet** uses feature flags to control which code gets compiled based
//! on how the crate is being used. This is especially important for the code
//! imported in the solution repository as this repo may be using an older
//! version of the rust toolchain (as of 2024-02-02 the version leetcode uses
//! is 1.74.1 found in their
//! [Help Center](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages)
//! ) due to the fact that leetcode uses a much
//! older version on their servers and some users may want to use the same
//! version to ensure their code will always work upon upload.
//! However, because the toolchain used by leetcode cannot compile many of the
//! crates used in the development of the tool due to leetcode's old version,
//! compiling them only behind a feature flag makes that a non-issue. It also
//! allows users to not compile the code only needed to support leetcode
//! solution development when working on or using the tool.
//!
//! - `default`: Enables the `leet_env` feature as this is the most common use
//!   case
//! - `leet_env`: Includes the code for working on leetcode problem solutions
//! - `tool`: Enables the code and dependencies used to create the tool.

#[cfg(feature = "leet_env")]
pub use leetcode_env::{
    list::{ListHead, ListNode},
    tree::{TreeNode, TreeRoot},
};

#[cfg(feature = "tool")]
pub use crate::tool::{cli::TopLevel, core::run, log::init_logging};

#[cfg(feature = "leet_env")]
mod leetcode_env;
#[cfg(feature = "tool")]
mod tool;
