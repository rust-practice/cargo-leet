use anyhow::Context;
use log::info;

use crate::tool::config::Config;

use super::{get_response, local_store::path_local_store_daily_challenge};

#[derive(serde::Deserialize, Debug)]
struct DailyChallengeResponse {
    data: Data,
}
impl DailyChallengeResponse {
    fn into_title_slug(self) -> String {
        self.data
            .active_daily_coding_challenge_question
            .question
            .title_slug
    }
}

#[derive(serde::Deserialize, Debug)]
struct Data {
    #[serde(rename = "activeDailyCodingChallengeQuestion")]
    active_daily_coding_challenge_question: ActiveDailyCodingChallengeQuestion,
}

#[derive(serde::Deserialize, Debug)]
struct ActiveDailyCodingChallengeQuestion {
    question: Question,
}

#[derive(serde::Deserialize, Debug)]
struct Question {
    #[serde(rename = "titleSlug")]
    title_slug: String,
}

pub(crate) fn get_daily_challenge_slug() -> anyhow::Result<String> {
    info!("Attempting to get daily challenge");
    Ok(get_daily_challenge_response()?.into_title_slug())
}

fn get_daily_challenge_response() -> anyhow::Result<DailyChallengeResponse> {
    get_response(
        "unused_just_to_match_sig",
        local_store_request_daily_challenge,
        external_request_daily_challenge,
    )
}

fn local_store_request_daily_challenge(_needed_to_match_signature: &str) -> anyhow::Result<String> {
    let path = path_local_store_daily_challenge();
    std::fs::read_to_string(&path).with_context(|| format!("failed to read string from {path:?}"))
}

fn external_request_daily_challenge(_needed_to_match_signature: &str) -> anyhow::Result<String> {
    info!("[External] Going to send request of daily challenge");
    ureq::post(Config::LEETCODE_GRAPH_QL)
        .send_json(serde_json::json!({
            "query": "query questionOfToday {
                activeDailyCodingChallengeQuestion {
                    question {
                        titleSlug
                    }
                }
            }",
            "variables":{},
            "operationName":"questionOfToday"
        }))
        .context("get request for daily challenge failed")?
        .body_mut()
        .read_to_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use anyhow::Context;
    use rstest::rstest;

    use crate::tool::core::helpers::{
        daily_challenge::{external_request_daily_challenge, get_daily_challenge_response},
        local_store::tests::{get_rnd_request_delay, insta_settings},
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
    fn conversion_from_leetcode_response(insta_settings: insta::Settings) {
        insta_settings.bind(|| {
            insta::assert_debug_snapshot!(get_daily_challenge_response().unwrap());
        });
    }
}
