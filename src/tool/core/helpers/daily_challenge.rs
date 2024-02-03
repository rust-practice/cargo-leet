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
    ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
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
        .into_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests;
