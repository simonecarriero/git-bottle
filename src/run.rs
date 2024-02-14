use crate::git::Git;
use crate::prompt::Prompt;
use crate::structure::{CommitStructure, Trailer, Values};
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};

pub fn run(
    prompt: &impl Prompt,
    git: &impl Git,
    commit_structure: CommitStructure,
) -> Result<String, anyhow::Error> {
    inquire::set_global_render_config(get_render_config());

    let message = prompt.text("Message:")?;

    let mut all_trailers: Vec<String> = vec![];
    for trailer in commit_structure.trailers {
        match trailer {
            Trailer::Text(m) => {
                let key = format!("{}:", m.name);
                let value = prompt.text(&key)?;
                add_trailer(&mut all_trailers, &key, &value);
            }
            Trailer::MultiSelect(m) => match &m.values {
                Values::FromOptions(v) => {
                    let key = format!("{}:", m.name);
                    let values = prompt.multi_select(&key, v.options.clone(), false)?;
                    add_trailers(&mut all_trailers, &key, &values);
                }
                Values::FromGitLog(v) => {
                    let key = format!("{}:", m.name);
                    let options = find_in_git_log(git, &v.format_strings, &v.depth)?;
                    if options.is_empty() {
                        let values = prompt.multi_text(&key)?;
                        add_trailers(&mut all_trailers, &key, &values);
                    } else {
                        let values = prompt.multi_select(&key, options, true)?;
                        add_trailers(&mut all_trailers, &key, &values);
                    }
                }
            },
            Trailer::Select(m) => match &m.values {
                Values::FromOptions(v) => {
                    let key = format!("{}:", m.name);
                    let value = prompt.select(&key, v.options.clone(), false);
                    add_trailer(&mut all_trailers, &key, &value?);
                }
                Values::FromGitLog(v) => {
                    let key = format!("{}:", m.name);
                    let options = find_in_git_log(git, &v.format_strings, &v.depth)?;
                    if options.is_empty() {
                        let value = prompt.text(&key)?;
                        add_trailer(&mut all_trailers, &key, &value);
                    } else {
                        let value = prompt.select(&key, options, true)?;
                        add_trailer(&mut all_trailers, &key, &value);
                    }
                }
            },
        }
    }
    let lines: Vec<String> = vec![vec![message, "".to_string()], all_trailers]
        .into_iter()
        .flatten()
        .collect();
    Ok(lines.join("\n"))
}

fn add_trailer(trailers: &mut Vec<String>, key: &str, value: &str) {
    if !value.is_empty() {
        trailers.push(format!("{} {}", key, value));
    }
}

fn add_trailers(trailers: &mut Vec<String>, key: &str, values: &[String]) {
    if !values.is_empty() {
        trailers.push(
            values
                .iter()
                .map(|v| format!("{} {}", key, v))
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }
}

fn get_render_config() -> RenderConfig {
    RenderConfig {
        prompt_prefix: Styled::new(">"),
        answered_prompt_prefix: Styled::new(">").with_fg(Color::LightGreen),
        selected_checkbox: Styled::new("[x]"),
        answer: StyleSheet::new().with_fg(Color::LightGreen),
        ..Default::default()
    }
}

fn find_in_git_log(
    git: &impl Git,
    format_strings: &[String],
    max_count: &Option<i32>,
) -> Result<Vec<String>, anyhow::Error> {
    let mut values = vec![];
    for format_string in format_strings {
        values.append(&mut git.log(format_string, max_count)?);
    }
    values.sort();
    values.dedup();
    Ok(values)
}

#[cfg(test)]
mod test {
    use crate::git::MockGit;
    use crate::prompt::MockPrompt;
    use crate::run::run;
    use crate::structure::{
        CommitStructure, MultiSelectTrailer, SelectTrailer, TextTrailer, Trailer, Values,
        ValuesFromGitLog, ValuesFromOptions,
    };
    use mockall::predicate::eq;

    #[test]
    fn test_text() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::Text(TextTrailer {
                name: "Issue".to_string(),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        prompt
            .expect_text()
            .with(eq("Issue:".to_string()))
            .returning(|_| Ok("#42".to_string()));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(commit_message, "First commit\n\nIssue: #42".to_string())
    }

    #[test]
    fn test_select_from_options() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::Select(SelectTrailer {
                name: "Issue".to_string(),
                values: Values::FromOptions(ValuesFromOptions {
                    options: vec!["#1".to_string(), "#2".to_string()],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        prompt
            .expect_select()
            .with(
                eq("Issue:".to_string()),
                eq(vec!["#1".to_string(), "#2".to_string()]),
                eq(false),
            )
            .returning(|_, _, _| Ok("#2".to_string()));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(commit_message, "First commit\n\nIssue: #2".to_string())
    }

    #[test]
    fn test_select_from_git_log() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::Select(SelectTrailer {
                name: "Issue".to_string(),
                values: Values::FromGitLog(ValuesFromGitLog {
                    depth: None,
                    format_strings: vec!["%(trailers:key=Issue,valueonly=true)".to_string()],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let mut git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        git.expect_log()
            .with(
                eq("%(trailers:key=Issue,valueonly=true)".to_string()),
                eq(None),
            )
            .returning(|_, _| Ok(vec!["#1".to_string(), "#2".to_string()]));

        prompt
            .expect_select()
            .with(
                eq("Issue:".to_string()),
                eq(vec!["#1".to_string(), "#2".to_string()]),
                eq(true),
            )
            .returning(|_, _, _| Ok("#2".to_string()));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(commit_message, "First commit\n\nIssue: #2".to_string())
    }

    #[test]
    fn test_select_from_git_log_when_it_is_empty() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::Select(SelectTrailer {
                name: "Issue".to_string(),
                values: Values::FromGitLog(ValuesFromGitLog {
                    depth: None,
                    format_strings: vec!["%(trailers:key=Issue,valueonly=true)".to_string()],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let mut git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        git.expect_log()
            .with(
                eq("%(trailers:key=Issue,valueonly=true)".to_string()),
                eq(None),
            )
            .returning(|_, _| Ok(vec![]));

        prompt
            .expect_text()
            .with(eq("Issue:".to_string()))
            .returning(|_| Ok("#42".to_string()));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(commit_message, "First commit\n\nIssue: #42".to_string())
    }

    #[test]
    fn test_multi_select_from_options() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::MultiSelect(MultiSelectTrailer {
                name: "Co-authored-by".to_string(),
                values: Values::FromOptions(ValuesFromOptions {
                    options: vec![
                        "NAME <NAME@EXAMPLE.COM>".to_string(),
                        "ANOTHER-NAME <ANOTHER-NAME@EXAMPLE.COM>".to_string(),
                    ],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        prompt
            .expect_multi_select()
            .with(
                eq("Co-authored-by:".to_string()),
                eq(vec![
                    "NAME <NAME@EXAMPLE.COM>".to_string(),
                    "ANOTHER-NAME <ANOTHER-NAME@EXAMPLE.COM>".to_string(),
                ]),
                eq(false),
            )
            .returning(|_, _, _| Ok(vec!["NAME <NAME@EXAMPLE.COM>".to_string()]));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(
            commit_message,
            "First commit\n\nCo-authored-by: NAME <NAME@EXAMPLE.COM>".to_string()
        )
    }

    #[test]
    fn test_multi_select_from_git_log() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::MultiSelect(MultiSelectTrailer {
                name: "Co-authored-by".to_string(),
                values: Values::FromGitLog(ValuesFromGitLog {
                    depth: None,
                    format_strings: vec![
                        "%(trailers:key=Co-authored-by,valueonly=true)".to_string()
                    ],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let mut git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        git.expect_log()
            .with(
                eq("%(trailers:key=Co-authored-by,valueonly=true)".to_string()),
                eq(None),
            )
            .returning(|_, _| {
                Ok(vec![
                    "James Smith <james.smith@example.org>".to_string(),
                    "Jane Doe <jane.doe@example.org>".to_string(),
                    "Joe Shmoe <joe.shmoe@example.org>".to_string(),
                ])
            });

        prompt
            .expect_multi_select()
            .with(
                eq("Co-authored-by:".to_string()),
                eq(vec![
                    "James Smith <james.smith@example.org>".to_string(),
                    "Jane Doe <jane.doe@example.org>".to_string(),
                    "Joe Shmoe <joe.shmoe@example.org>".to_string(),
                ]),
                eq(true),
            )
            .returning(|_, _, _| Ok(vec!["Jane Doe <jane.doe@example.org>".to_string()]));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(
            commit_message,
            "First commit\n\nCo-authored-by: Jane Doe <jane.doe@example.org>".to_string()
        )
    }

    #[test]
    fn test_multi_select_from_git_log_when_it_is_empty() {
        let commit_structure = CommitStructure {
            trailers: vec![Trailer::MultiSelect(MultiSelectTrailer {
                name: "Co-authored-by".to_string(),
                values: Values::FromGitLog(ValuesFromGitLog {
                    depth: None,
                    format_strings: vec![
                        "%(trailers:key=Co-authored-by,valueonly=true)".to_string()
                    ],
                }),
            })],
        };

        let mut prompt: MockPrompt = MockPrompt::new();
        let mut git: MockGit = MockGit::new();

        prompt
            .expect_text()
            .with(eq("Message:".to_string()))
            .returning(|_| Ok("First commit".to_string()));

        git.expect_log()
            .with(
                eq("%(trailers:key=Co-authored-by,valueonly=true)".to_string()),
                eq(None),
            )
            .returning(|_, _| Ok(vec![]));

        prompt
            .expect_multi_text()
            .with(eq("Co-authored-by:".to_string()))
            .returning(|_| Ok(vec!["Jane Doe <jane.doe@example.org>".to_string()]));

        let commit_message = run(&prompt, &git, commit_structure).unwrap();

        assert_eq!(
            commit_message,
            "First commit\n\nCo-authored-by: Jane Doe <jane.doe@example.org>".to_string()
        )
    }
}
