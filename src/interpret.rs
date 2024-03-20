use std::str::FromStr;

use anyhow::{Error, Result};

use crate::{cmd, context::Context};

pub struct CommandLine {
    args: Vec<String>,
}

impl CommandLine {
    pub async fn interpret(&self, ctx: &mut Context) -> Result<()> {
        cmd::invoke(&self.args, ctx).await
    }
}

impl FromStr for CommandLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        // TODO: Support quoting
        Ok(Self {
            args: line.split_whitespace().map(|s| s.to_owned()).collect()
        })
    }
}
