use anyhow::Result;

use crate::{cmd, context::Context};

pub async fn interpret_line(line: &str, ctx: &mut Context) -> Result<()> {
    // TODO: Support quoting
    let args: Vec<_> = line.split_whitespace().collect();
    if args.is_empty() {
        return Ok(());
    }
    cmd::invoke(&args, ctx).await
}
