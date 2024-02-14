use anyhow::anyhow;
use inquire::{MultiSelect, Select, Text};

#[cfg_attr(test, mockall::automock)]
pub trait Prompt {
    fn text(&self, key: &str) -> Result<String, anyhow::Error>;
    fn multi_text(&self, key: &str) -> Result<Vec<String>, anyhow::Error>;
    fn select(
        &self,
        key: &str,
        options: Vec<String>,
        allow_other: bool,
    ) -> Result<String, anyhow::Error>;
    fn multi_select(
        &self,
        key: &str,
        options: Vec<String>,
        allow_other: bool,
    ) -> Result<Vec<String>, anyhow::Error>;
}

pub struct PromptInquire {}

impl Prompt for PromptInquire {
    fn text(&self, key: &str) -> Result<String, anyhow::Error> {
        Text::new(key).prompt().map_err(|e| anyhow!(e))
    }

    fn multi_text(&self, key: &str) -> Result<Vec<String>, anyhow::Error> {
        let mut values: Vec<String> = vec![];
        loop {
            let value = self.text(key)?;
            if value.is_empty() {
                break;
            }
            values.push(value);
        }
        Ok(values)
    }

    fn select(
        &self,
        key: &str,
        options: Vec<String>,
        allow_other: bool,
    ) -> Result<String, anyhow::Error> {
        if allow_other {
            let mut options = options.clone();
            options.push("<other>".to_string());
            let value = Select::new(key, options).prompt().map_err(|e| anyhow!(e))?;
            if value.eq("<other>") {
                return self.text(key);
            }
            Ok(value)
        } else {
            Select::new(key, options).prompt().map_err(|e| anyhow!(e))
        }
    }

    fn multi_select(
        &self,
        key: &str,
        options: Vec<String>,
        allow_other: bool,
    ) -> Result<Vec<String>, anyhow::Error> {
        if allow_other {
            let mut options = options.clone();
            options.push("<other>".to_string());
            let mut values = MultiSelect::new(key, options)
                .prompt()
                .map_err(|e| anyhow!(e))?;
            if values.contains(&"<other>".to_string()) {
                values.retain(|x| x != "<other>");
                values.append(&mut self.multi_text(key)?);
            }
            Ok(values)
        } else {
            MultiSelect::new(key, options)
                .prompt()
                .map_err(|e| anyhow!(e))
        }
    }
}
