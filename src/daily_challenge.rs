use serde::Deserialize;
use serde_flat_path::flat_path;

#[flat_path]
#[derive(Deserialize)]
struct DailyChallengeResponse {
    #[flat_path("data.activeDailyCodingChallengeQuestion.question.titleSlug")]
    title_slug: String,
}

pub fn get_daily_challenge_slug() -> String {
    let daily_challenge_response = ureq::get("https://leetcode.com/graphql/")
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
        .unwrap()
        .into_json::<DailyChallengeResponse>()
        .unwrap();
    daily_challenge_response.title_slug
}
