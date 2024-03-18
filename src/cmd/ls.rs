use anyhow::Result;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let path = ctx.cwd.join(VirtualPathBuf::from(args));
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    let entries: Vec<_> = response.payload.entries.into_keys().collect();
    println!("{}", entries.join("   "));
    Ok(())
}
