use anyhow::Result;

use crate::context::Context;

pub async fn invoke(_args: &str, ctx: &mut Context) -> Result<()> {
    let response = ctx.lh.list(&ctx.cwd.as_relative().as_str_vec()).await?;
    println!("{}", response.payload);
    Ok(())
}
