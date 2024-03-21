use anyhow::Result;

use crate::context::Context;

pub async fn invoke(args: &[String], _ctx: &mut Context) -> Result<String> {
    Ok(format!("{}", args[1..].join(" ")))
}
