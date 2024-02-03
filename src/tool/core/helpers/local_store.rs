//! Facilitates testing by providing local copies of responses from leetcode

use std::path::PathBuf;

const NAME_TEST_FOLDER: &str = "tests";
const NAME_LOCAL_STORE: &str = "local_store";
const NAME_LOCAL_STORE_CODE_SNIPPET: &str = "code_snippet";

pub(crate) fn path_local_store_code_snippet() -> PathBuf {
    PathBuf::from(NAME_TEST_FOLDER)
        .join(NAME_LOCAL_STORE)
        .join(NAME_LOCAL_STORE_CODE_SNIPPET)
}

#[cfg(test)]
pub(crate) mod tests {
    use rstest::fixture;

    pub(crate) type SlugList = &'static [&'static str];

    // Don't want to send them too fast and not at the same time for all tests so random in range
    pub(crate) const SECONDS_DELAY_BETWEEN_REQUESTS_MIN: u8 = 5;
    pub(crate) const SECONDS_DELAY_BETWEEN_REQUESTS_MAX: u8 = 9;

    #[fixture]
    pub(crate) fn title_slugs() -> SlugList {
        &["two-sum", "add-two-numbers", "validate-binary-search-tree"]
    }
}
