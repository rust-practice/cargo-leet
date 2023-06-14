pub struct ProblemCode {
    code: String,
}

impl From<String> for ProblemCode {
    fn from(value: String) -> Self {
        Self { code: value }
    }
}

impl From<ProblemCode> for String {
    fn from(value: ProblemCode) -> Self {
        value.code
    }
}

impl AsRef<str> for ProblemCode {
    fn as_ref(&self) -> &str {
        &self.code
    }
}

impl ProblemCode {
    pub fn is_design(&self) -> bool {
        !self.code.starts_with("impl Solution {")
    }
}
