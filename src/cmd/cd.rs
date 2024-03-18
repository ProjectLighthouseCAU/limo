use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    ctx.cwd.push(VirtualPathBuf::from(args));
    Ok(())
}
