use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let path = ctx.cwd.join(VirtualPathBuf::from(args));
    ctx.lh.delete(&path.as_lh_vec()).await?;
    Ok(())
}
