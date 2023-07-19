pub(crate) struct Config {}

impl Config {
    // TODO: Add tests on URLs to ensure they end in the trailing "/" as this is
    // assumed in the code using them URLs Must include trailing "/"
    pub(crate) const LEETCODE_PROBLEM_URL: &str = "https://leetcode.com/problems/";
    pub(crate) const LEETCODE_GRAPH_QL: &str = "https://leetcode.com/graphql/";
}
