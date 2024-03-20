use anyhow::{bail, Result};
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "rm")]
struct Args {
    #[arg(short, long, action, help = "Recursively removes a directory")]
    recursive: bool,

    #[arg(required = true, help = "The paths to remove")]
    paths: Vec<VirtualPathBuf>,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    for path in args.paths {
        let path = ctx.cwd.join(path);
        let is_dir = ctx.lh.list(&path.as_lh_vec()).await.is_ok();
        if is_dir && !args.recursive {
            bail!("{} is a directory, pass -r to delete it!", path);
        }
        ctx.lh.delete(&path.as_lh_vec()).await?;
    }
    Ok(())
}
