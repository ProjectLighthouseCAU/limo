use anyhow::Result;

use crate::{cmd, context::Context};

use super::parse::CommandLine;

pub async fn interpret(line: CommandLine, ctx: &mut Context) -> Result<()> {
    let output = cmd::invoke(&line.args, ctx).await?;
    if !output.is_empty() {
        print!("{}", output);
    }
    Ok(())
}
