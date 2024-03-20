use anyhow::Result;

use crate::context::Context;

// TODO: Handle quoting etc.

pub async fn invoke(args: &[&str], _ctx: &mut Context) -> Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}
