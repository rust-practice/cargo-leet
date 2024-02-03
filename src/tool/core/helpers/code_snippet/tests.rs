use rstest::rstest;

use crate::tool::core::helpers::{
    code_snippet::get_code_snippets_response,
    local_store::tests::{title_slugs, SlugList},
};

// TODO Onè: Create an ignored test to download the data for testing

#[rstest]
fn conversion_from_leetcode_response(title_slugs: SlugList) {
    for title_slug in title_slugs {
        insta::assert_debug_snapshot!(get_code_snippets_response(title_slug).unwrap());
    }
}
