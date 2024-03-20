use anyhow::{bail, Context as _, Result};
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "rmdir")]
struct Args {
    #[arg(default_value = ".", help = "The directory to remove")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let tree = ctx.lh.list(&path.as_lh_vec()).await.context("Not a directory, use rm instead!")?.payload;
    if !tree.entries.is_empty() {
        bail!("{} is not empty, use rm -r instead!", path);
    }
    ctx.lh.delete(&path.as_lh_vec()).await?;
    Ok(())
}
