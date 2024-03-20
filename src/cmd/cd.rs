use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "cd")]
struct Args {
    #[arg(default_value = ".", help = "The directory to switch to")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let new_cwd = ctx.cwd.join(args.path);
    ctx.lh.list(&new_cwd.as_lh_vec()).await?;
    ctx.cwd = new_cwd;
    Ok(())
}
