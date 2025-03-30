use anyhow::Context as _;

pub(crate) mod code_snippet;
pub(crate) mod daily_challenge;
pub(crate) mod local_store;
pub(crate) mod problem_code;
pub(crate) mod problem_description;
pub(crate) mod problem_metadata;
pub(crate) mod write_to_disk;

fn get_response<
    T: for<'de> serde::Deserialize<'de>,
    FLocal: FnOnce(&str) -> anyhow::Result<String>,
    FExternal: FnOnce(&str) -> anyhow::Result<String>,
>(
    title_slug: &str,
    local_store_fn: FLocal,
    external_request_fn: FExternal,
) -> anyhow::Result<T> {
    let json = if cfg!(test) {
        local_store_fn(title_slug)
    } else {
        external_request_fn(title_slug)
    }?;
    let result = serde_json::from_str(&json).context("failed to convert from String as json")?;
    Ok(result)
}
