use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let new_cwd = ctx.cwd.join(VirtualPathBuf::from(args));
    ctx.lh.list(&new_cwd.as_lh_vec()).await?;
    ctx.cwd = new_cwd;
    Ok(())
}
