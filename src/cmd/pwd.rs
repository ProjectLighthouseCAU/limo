use anyhow::Result;

use crate::context::Context;

pub async fn invoke(_args: &[String], ctx: &mut Context) -> Result<String> {
    Ok(format!("{}", ctx.cwd))
}
