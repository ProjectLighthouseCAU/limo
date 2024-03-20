use anyhow::{bail, Result};

use crate::context::Context;

mod cat;
mod cd;
mod echo;
mod ls;
mod mkdir;
mod pwd;
mod rm;
mod touch;
mod tree;

pub async fn interpret_line(line: &str, ctx: &mut Context) -> Result<()> {
    let (cmd, args) = line.split_once(' ').unwrap_or_else(|| (line.as_ref(), ""));
    interpret(cmd, args, ctx).await
}

pub async fn interpret(cmd: &str, args: &str, ctx: &mut Context) -> Result<()> {
    match cmd {
        "cat" => cat::invoke(args, ctx).await?,
        "cd" => cd::invoke(args, ctx).await?,
        "echo" => echo::invoke(args, ctx).await?,
        "ls" => ls::invoke(args, ctx).await?,
        "mkdir" => mkdir::invoke(args, ctx).await?,
        "pwd" => pwd::invoke(args, ctx).await?,
        "rm" => rm::invoke(args, ctx).await?,
        "touch" => touch::invoke(args, ctx).await?,
        "tree" => tree::invoke(args, ctx).await?,
        _ => bail!("Unrecognized command {}", cmd),
    }
    Ok(())
}
