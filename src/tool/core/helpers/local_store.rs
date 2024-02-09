//! Facilitates testing by providing local copies of responses from leetcode

use std::path::{Path, PathBuf};

const NAME_TEST_FOLDER: &str = "tests";
const NAME_LOCAL_STORE: &str = "local_store";
const NAME_LOCAL_STORE_CODE_SNIPPET: &str = "code_snippet";
const NAME_LOCAL_STORE_DAILY_CHALLENGE: &str = "daily_challenge";
const NAME_LOCAL_STORE_PROBLEM_METADATA: &str = "problem_metadata";

pub(crate) fn path_local_store_code_snippet<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(NAME_TEST_FOLDER)
        .join(NAME_LOCAL_STORE)
        .join(NAME_LOCAL_STORE_CODE_SNIPPET)
        .join(path)
}

pub(crate) fn path_local_store_daily_challenge() -> PathBuf {
    PathBuf::from(NAME_TEST_FOLDER)
        .join(NAME_LOCAL_STORE)
        .join(NAME_LOCAL_STORE_DAILY_CHALLENGE)
}

pub(crate) fn path_local_store_problem_metadata<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(NAME_TEST_FOLDER)
        .join(NAME_LOCAL_STORE)
        .join(NAME_LOCAL_STORE_PROBLEM_METADATA)
        .join(path)
}

#[cfg(test)]
pub(crate) mod tests {
    use rand::{thread_rng, Rng};
    use rstest::fixture;

    const NAME_SNAPSHOT_FOLDER: &str = "snapshots";

    use super::NAME_TEST_FOLDER;

    pub(crate) type SlugList = &'static [&'static str];

    // Don't want to send them too fast and not at the same time for all tests so random in range
    const MILLISECONDS_DELAY_BETWEEN_REQUESTS_MIN: u64 = 5000;
    const MILLISECONDS_DELAY_BETWEEN_REQUESTS_MAX: u64 = 9000;

    pub(crate) fn get_rnd_request_delay() -> u64 {
        let mut rng = thread_rng();
        rng.gen_range(
            MILLISECONDS_DELAY_BETWEEN_REQUESTS_MIN..MILLISECONDS_DELAY_BETWEEN_REQUESTS_MAX,
        )
    }

    #[fixture]
    pub(crate) fn title_slugs() -> SlugList {
        &["two-sum", "add-two-numbers", "validate-binary-search-tree"]
    }

    #[fixture]
    pub(crate) fn insta_settings() -> insta::Settings {
        let mut result = insta::Settings::clone_current();
        let cwd = std::env::current_dir().expect("failed to get cwd");
        let path = cwd.join(NAME_TEST_FOLDER).join(NAME_SNAPSHOT_FOLDER);
        result.set_snapshot_path(path);
        result
    }
}
