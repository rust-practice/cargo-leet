use anyhow::Context;

use crate::tool::config::Config;

use super::local_store::path_local_store_daily_challenge;

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
    #[serde(rename = "titleSlu")] // TODO Onè: Fix error (missing g)
    title_slug: String,
}

pub(crate) fn get_daily_challenge_slug() -> anyhow::Result<String> {
    Ok(get_daily_challenge_response()?.into_title_slug())
}

fn get_daily_challenge_response() -> anyhow::Result<DailyChallengeResponse> {
    let json = if cfg!(test) {
        local_store_request_daily_challenge()
    } else {
        external_request_daily_challenge()
    }?;
    let result = serde_json::from_str(&json)
        .context("failed to convert from String as json to DailyChallengeResponse")?;
    Ok(result)
}

fn local_store_request_daily_challenge() -> anyhow::Result<String> {
    let path = path_local_store_daily_challenge();
    std::fs::read_to_string(&path).with_context(|| format!("failed to read string from {path:?}"))
}

fn external_request_daily_challenge() -> anyhow::Result<String> {
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

        .context("Get request for daily challenge failed")?
        .into_json::<DailyChallengeResponse>()
        .context("Failed to convert response for daily challenge from json")?;
    Ok(daily_challenge_response.title_slug)
}
