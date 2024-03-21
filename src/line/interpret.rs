use anyhow::Result;

use crate::{cmd, context::Context};

use super::parse::CommandLine;

pub async fn interpret(line: CommandLine, ctx: &mut Context) -> Result<()> {
    cmd::invoke(&line.args, ctx).await
}
