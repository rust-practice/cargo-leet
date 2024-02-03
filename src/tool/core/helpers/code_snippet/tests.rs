use std::io::Write as _;

use anyhow::Context;
use rstest::rstest;

use crate::tool::core::helpers::{
    code_snippet::{external_request_code_snippet, get_code_snippets_response},
    local_store::tests::{get_rnd_request_delay, title_slugs, SlugList},
};

#[rstest]
#[ignore = "Only use for downloading responses"]
fn download_response_from_leetcode(title_slugs: SlugList) {
    for title_slug in title_slugs {
        let sleep_delay = std::time::Duration::from_millis(get_rnd_request_delay());
        println!(
            "Going to sleep for {} milliseconds before requesting and trying to save {title_slug}",
            sleep_delay.as_millis()
        );
        std::thread::sleep(sleep_delay); // Sleep to not go too hard on leetcode API
        let response_string = external_request_code_snippet(title_slug).unwrap();
        let path = super::path_local_store_code_snippet(title_slug);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .with_context(|| format!("failed to save response to {path:?}"))
            .unwrap();
        file.write_all(response_string.as_bytes()).unwrap();
        println!("Save of {title_slug:?} completed\n");
    }
    println!("Successfully saved all responses");
}

#[rstest]
fn conversion_from_leetcode_response(title_slugs: SlugList) {
    for title_slug in title_slugs {
        insta::assert_debug_snapshot!(get_code_snippets_response(title_slug).unwrap());
    }
}
