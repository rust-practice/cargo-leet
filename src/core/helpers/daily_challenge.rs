use anyhow::Context;
use serde::Deserialize;
use serde_flat_path::flat_path;

use crate::config::Config;

#[flat_path]
#[derive(Deserialize)]
struct DailyChallengeResponse {
    #[flat_path("data.activeDailyCodingChallengeQuestion.question.titleSlug")]
    title_slug: String,
}

pub fn get_daily_challenge_slug() -> anyhow::Result<String> {
    let daily_challenge_response = ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
            "query": r#"query questionOfToday {
                activeDailyCodingChallengeQuestion {
                    question {
                        titleSlug
                    }
                }
            }"#,
            "variables":{},
            "operationName":"questionOfToday"
        }))
        .context("Get request for daily challenge failed")?
        .into_json::<DailyChallengeResponse>()
        .context("Failed to convert response for daily challenge from json")?;
    Ok(daily_challenge_response.title_slug)
}
