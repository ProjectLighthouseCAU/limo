use anyhow::{bail, Result};

use crate::{cmd, context::Context};

use super::parse::CommandLine;

pub async fn interpret(line: CommandLine, ctx: &mut Context) -> Result<()> {
    if line.args.is_empty() {
        bail!("Cannot interpret empty line");
    }
    let output = cmd::invoke(&line.args, ctx).await?;
    if !output.is_empty() {
        print!("{}", output);
    }
    Ok(())
}
