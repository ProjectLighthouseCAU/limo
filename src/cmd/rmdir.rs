use anyhow::{bail, Context as _, Result};
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "rmdir")]
struct Args {
    #[arg(required = true, help = "The directories to remove")]
    paths: Vec<VirtualPathBuf>,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    for path in args.paths {
        let path = ctx.cwd.join(path);
        let tree = ctx.lh.list(&path.as_lh_vec()).await.context("Not a directory, use rm instead!")?.payload;
        if !tree.entries.is_empty() {
            bail!("{} is not empty, use rm -r instead!", path);
        }
        ctx.lh.delete(&path.as_lh_vec()).await?;
    }
    Ok(())
}
