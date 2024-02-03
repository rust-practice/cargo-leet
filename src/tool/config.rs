pub(crate) struct Config {}

impl Config {
    // assumed in the code using them URLs Must end with trailing "/"
    pub(crate) const LEETCODE_PROBLEM_URL: &'static str = "https://leetcode.com/problems/";
    pub(crate) const LEETCODE_GRAPH_QL: &'static str = "https://leetcode.com/graphql/";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_url_ends_with_slash() {
        // TODO Onè: Switch to using https://docs.rs/static_assertions/latest/static_assertions/
        assert!(Config::LEETCODE_PROBLEM_URL.ends_with('/'));
    }

    #[test]
    fn graph_ql_url_ends_with_slash() {
        assert!(Config::LEETCODE_GRAPH_QL.ends_with('/'));
    }
}
