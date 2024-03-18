use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let path = ctx.cwd.join(VirtualPathBuf::from(args));
    ctx.lh.create(&path.as_relative().as_str_vec()).await?;
    Ok(())
}
