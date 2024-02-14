use crate::structure::{CommitStructure, MultiSelectTrailer, Trailer, Values, ValuesFromGitLog};
use anyhow::anyhow;
use std::env;

pub fn get() -> Result<CommitStructure, anyhow::Error> {
    match find_git_bottle_config() {
        None => Ok(default_config()),
        Some(filename) => {
            let file = std::fs::File::open(&filename)
                .map_err(|e| anyhow!("Could not open file: {}", e))?;
            serde_yaml::from_reader(file)
                .map_err(|e| anyhow!("Malformed config file {}: {}", filename, e))
        }
    }
}

fn default_config() -> CommitStructure {
    CommitStructure {
        trailers: vec![Trailer::MultiSelect(MultiSelectTrailer {
            name: "Co-authored-by".to_string(),
            values: Values::FromGitLog(ValuesFromGitLog {
                depth: None,
                format_strings: vec![
                    "%an <%ae>".to_string(),
                    "%(trailers:key=Co-authored-by,valueonly=true)".to_string(),
                ],
            }),
        })],
    }
}

fn find_git_bottle_config() -> Option<String> {
    let current_dir = env::current_dir().ok()?;

    for ancestor in current_dir.ancestors() {
        let file_path = ancestor.join(".gitbottle.yml");
        if file_path.exists() {
            return Some(file_path.to_string_lossy().into_owned());
        }
    }

    None
}
