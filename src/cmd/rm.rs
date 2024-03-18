use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

// TODO: Add a failsafe to only delete individual resources and not directories
// We could support the classic -rf flags for that.

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let path = ctx.cwd.join(VirtualPathBuf::from(args));
    ctx.lh.delete(&path.as_lh_vec()).await?;
    Ok(())
}
