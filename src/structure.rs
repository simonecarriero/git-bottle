use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitStructure {
    pub trailers: Vec<Trailer>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trailer {
    #[serde(rename = "text")]
    Text(TextTrailer),
    #[serde(rename = "select")]
    Select(SelectTrailer),
    #[serde(rename = "multi_select")]
    MultiSelect(MultiSelectTrailer),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextTrailer {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectTrailer {
    pub name: String,
    pub values: Values,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiSelectTrailer {
    pub name: String,
    pub values: Values,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Values {
    #[serde(rename = "from_options")]
    FromOptions(ValuesFromOptions),
    #[serde(rename = "from_git_log")]
    FromGitLog(ValuesFromGitLog),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValuesFromOptions {
    pub options: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValuesFromGitLog {
    pub max_count: Option<i32>,
    pub format_strings: Vec<String>,
}
