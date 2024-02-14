use anyhow::anyhow;
use std::process::{Command, Stdio};
use std::str;

#[cfg_attr(test, mockall::automock)]
pub trait Git {
    fn commit(&self, message: &str) -> Result<String, anyhow::Error>;
    fn log(
        &self,
        format_string: &str,
        max_count: &Option<i32>,
    ) -> Result<Vec<String>, anyhow::Error>;
}

pub struct GitCommand {}

impl Git for GitCommand {
    fn commit(&self, message: &str) -> Result<String, anyhow::Error> {
        let output = Command::new("git")
            .current_dir(".")
            .args(["commit", "-m", &message])
            .output()
            .map_err(|e| anyhow!(e))?;

        return if output.status.success() {
            let out = str::from_utf8(&output.stdout).unwrap_or_default();
            return Ok(out.to_string());
        } else {
            let out = str::from_utf8(&output.stdout).unwrap_or_default();
            let err = str::from_utf8(&output.stderr).unwrap_or_default();
            Err(anyhow!("{}\n{}", out, err))
        };
    }

    fn log(
        &self,
        format_string: &str,
        max_count: &Option<i32>,
    ) -> Result<Vec<String>, anyhow::Error> {
        let args: Vec<String> = vec![
            Some("log".to_string()),
            max_count.map(|d| format!("--max-count={}", d)),
            Some(format!("--format={}", format_string)),
        ]
        .into_iter()
        .flatten()
        .collect();

        let ps_child = Command::new("git")
            .args(&args)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!(e))?;
        let output = ps_child.wait_with_output().map_err(|e| anyhow!(e))?;
        let result = str::from_utf8(&output.stdout).map_err(|e| anyhow!(e))?;
        let mut lines: Vec<String> = result.split('\n').map(|l| l.to_string()).collect();

        lines.sort();
        lines.dedup();

        return Ok(lines
            .into_iter()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect::<Vec<String>>());
    }
}
