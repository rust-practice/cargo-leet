//! Facilitates testing by providing local copies of responses from leetcode

use std::path::{Path, PathBuf};

const NAME_TEST_FOLDER: &str = "tests";
const NAME_LOCAL_STORE: &str = "local_store";
const NAME_LOCAL_STORE_CODE_SNIPPET: &str = "code_snippet";
const NAME_LOCAL_STORE_DAILY_CHALLENGE: &str = "daily_challenge";
const NAME_LOCAL_STORE_PROBLEM_METADATA: &str = "problem_metadata";
const NAME_LOCAL_STORE_PROBLEM_DESCRIPTION: &str = "problem_description";

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

pub(crate) fn path_local_store_problem_description<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(NAME_TEST_FOLDER)
        .join(NAME_LOCAL_STORE)
        .join(NAME_LOCAL_STORE_PROBLEM_DESCRIPTION)
        .join(path)
}

#[cfg(test)]
pub(crate) mod tests {
    use rand::{Rng, rng};
    use rstest::fixture;

    const NAME_SNAPSHOT_FOLDER: &str = "snapshots";

    use super::NAME_TEST_FOLDER;

    pub(crate) type SlugList = &'static [&'static str];

    // Don't want to send them too fast and not at the same time for all tests so
    // random in range
    const MILLISECONDS_DELAY_BETWEEN_REQUESTS_MIN: u64 = 5000;
    const MILLISECONDS_DELAY_BETWEEN_REQUESTS_MAX: u64 = 9000;

    pub(crate) fn get_rnd_request_delay() -> u64 {
        let mut rng = rng();
        rng.random_range(
            MILLISECONDS_DELAY_BETWEEN_REQUESTS_MIN..MILLISECONDS_DELAY_BETWEEN_REQUESTS_MAX,
        )
    }

    #[fixture]
    pub(crate) fn title_slugs() -> SlugList {
        &[
            "add-two-numbers",
            "construct-smallest-number-from-di-string",
            "count-days-without-meetings",
            "count-of-substrings-containing-every-vowel-and-k-consonants-ii",
            "flood-fill",
            "two-sum",
            "valid-anagram",
            "valid-parentheses",
            "validate-binary-search-tree",
        ]
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
