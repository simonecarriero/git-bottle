extern crate serde_yaml;

mod config;
mod git;
mod prompt;
mod run;
mod structure;

use crate::git::{Git, GitCommand};
use crate::prompt::PromptInquire;
use std::process::exit;

fn main() -> Result<(), anyhow::Error> {
    let prompt = PromptInquire {};
    let git = GitCommand {};

    let structure = config::get()?;
    let message = run::run(&prompt, &git, structure)?;
    match git.commit(&message) {
        Ok(out) => println!("{}", out),
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
    Ok(())
}
