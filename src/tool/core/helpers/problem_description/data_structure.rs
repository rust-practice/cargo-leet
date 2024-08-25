use log::info;
use regex::Regex;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ProblemDescriptionResponse {
    data: Data,
}

#[derive(serde::Deserialize, Debug)]
struct Data {
    question: Question,
}

#[derive(serde::Deserialize, Debug)]
struct Question {
    content: String,
}

#[derive(Debug)]
pub(crate) struct ProblemDescription {
    content: String,
}

impl ProblemDescription {
    pub(crate) fn get_solutions(&self) -> Vec<String> {
        info!("Extracting solutions from description");
        let re = Regex::new(r"Output:<\/strong> (.+?)\n").expect("compiling static regex");
        re.captures_iter(&self.content)
            .map(|mat| mat[1].to_string())
            .collect()
    }
}

impl TryFrom<ProblemDescriptionResponse> for ProblemDescription {
    type Error = anyhow::Error;

    fn try_from(value: ProblemDescriptionResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            content: value.data.question.content,
        })
    }
}
