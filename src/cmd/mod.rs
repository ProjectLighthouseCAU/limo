use anyhow::{bail, Result};

use crate::context::Context;

mod cat;
mod cd;
mod echo;
mod ls;
mod mkdir;
mod pwd;
mod rm;
mod rmdir;
mod touch;
mod tree;

pub async fn interpret_line(line: &str, ctx: &mut Context) -> Result<()> {
    // TODO: Support quoting
    let args: Vec<_> = line.split_whitespace().collect();
    if args.is_empty() {
        return Ok(());
    }
    interpret(&args, ctx).await
}

async fn interpret(args: &[&str], ctx: &mut Context) -> Result<()> {
    match args[0] {
        "cat" => cat::invoke(args, ctx).await?,
        "cd" => cd::invoke(args, ctx).await?,
        "echo" => echo::invoke(args, ctx).await?,
        "ls" => ls::invoke(args, ctx).await?,
        "mkdir" => mkdir::invoke(args, ctx).await?,
        "pwd" => pwd::invoke(args, ctx).await?,
        "rm" => rm::invoke(args, ctx).await?,
        "rmdir" => rmdir::invoke(args, ctx).await?,
        "touch" => touch::invoke(args, ctx).await?,
        "tree" => tree::invoke(args, ctx).await?,
        cmd => bail!("Unrecognized command {}", cmd),
    }
    Ok(())
}
