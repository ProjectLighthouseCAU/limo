use anyhow::Result;

use crate::context::Context;

pub async fn invoke(args: &[String], _ctx: &mut Context) -> Result<()> {
    println!("{}", args[1..].join(" "));
    Ok(())
}
