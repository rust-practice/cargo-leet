use crate::tool::config::Config;
use anyhow::Context;
use log::info;

use super::{get_response, local_store::path_local_store_problem_metadata};

pub(crate) mod data_structure;

pub(crate) fn get_problem_metadata(
    title_slug: &str,
) -> anyhow::Result<data_structure::ProblemMetadata> {
    info!("Attempting to get problem metadata");
    get_problem_metadata_response(title_slug)?.into_problem_metadata()
}

fn get_problem_metadata_response(
    title_slug: &str,
) -> anyhow::Result<data_structure::ProblemMetaDataResponse> {
    get_response(
        title_slug,
        local_store_request_problem_metadata,
        external_request_problem_metadata,
    )
}

fn local_store_request_problem_metadata(title_slug: &str) -> anyhow::Result<String> {
    let path = path_local_store_problem_metadata(title_slug);
    std::fs::read_to_string(&path).with_context(|| format!("failed to read string from {path:?}"))
}

fn external_request_problem_metadata(title_slug: &str) -> anyhow::Result<String> {
    info!("[External] Going to send request for problem meta data for problem with title: {title_slug}");
    ureq::get(Config::LEETCODE_GRAPH_QL)
        .send_json(ureq::json!({
            "query": r"query consolePanelConfig($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                questionFrontendId
                questionTitle
                exampleTestcaseList
            }
        }",
            "variables":{"titleSlug": title_slug},
            "operationName":"consolePanelConfig"
        }))
        .context("failed to get request for meta data failed")?
        .into_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use anyhow::Context;
    use rstest::rstest;

    use crate::tool::core::helpers::{
        local_store::{
            path_local_store_problem_metadata,
            tests::{get_rnd_request_delay, insta_settings, title_slugs, SlugList},
        },
        problem_metadata::{external_request_problem_metadata, get_problem_metadata},
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
            let response_string = external_request_problem_metadata(title_slug).unwrap();
            let path = path_local_store_problem_metadata(title_slug);
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
    fn conversion_from_leetcode_response(title_slugs: SlugList, insta_settings: insta::Settings) {
        for title_slug in title_slugs {
            insta_settings.bind(|| {
                insta::assert_debug_snapshot!(get_problem_metadata(title_slug).unwrap());
            });
        }
    }
}
