use anyhow::Result;

use crate::context::Context;

pub async fn invoke(_args: &str, ctx: &mut Context) -> Result<()> {
    let response = ctx.lh.list(&ctx.cwd.as_lh_vec()).await?;
    let entries: Vec<_> = response.payload.entries.into_keys().collect();
    println!("{}", entries.join("   "));
    Ok(())
}
