use anyhow::{bail, Result};

use crate::context::Context;

mod cd;
mod ls;
mod pwd;
mod tree;

pub async fn interpret(cmd: &str, args: &str, ctx: &mut Context) -> Result<()> {
    match cmd {
        "cd" => cd::invoke(args, ctx).await?,
        "ls" => ls::invoke(args, ctx).await?,
        "pwd" => pwd::invoke(args, ctx).await?,
        "tree" => tree::invoke(args, ctx).await?,
        _ => bail!("Unrecognized command {}", cmd),
    }
    Ok(())
}
