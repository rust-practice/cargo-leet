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
        .context("failed to get request for code_snippet failed")?
        .into_string()
        .context("failed to convert response into String")
}

#[cfg(test)]
mod tests;
