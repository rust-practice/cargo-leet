use std::io::Write as _;

use anyhow::Context;
use rstest::rstest;

use crate::tool::core::helpers::{
    daily_challenge::{external_request_daily_challenge, get_daily_challenge_response},
    local_store::tests::get_rnd_request_delay,
};

#[rstest]
#[ignore = "Only use for downloading responses"]
fn download_response_from_leetcode() {
    let sleep_delay = std::time::Duration::from_millis(get_rnd_request_delay());
    println!(
        "Going to sleep for {} milliseconds before requesting and trying to save daily_challenge",
        sleep_delay.as_millis()
    );
    std::thread::sleep(sleep_delay); // Sleep to not go too hard on leetcode API
    let response_string = external_request_daily_challenge("unneeded").unwrap();
    let path = super::path_local_store_daily_challenge();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .with_context(|| format!("failed to save response to {path:?}"))
        .unwrap();
    file.write_all(response_string.as_bytes()).unwrap();
    println!("Successfully saved daily response, possibly not be useful to commit the new one");
}

#[rstest]
fn conversion_from_leetcode_response() {
    insta::assert_debug_snapshot!(get_daily_challenge_response().unwrap());
}
