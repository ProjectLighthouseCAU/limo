use anyhow::{bail, Result};
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "mkdir")]
struct Args {
    #[arg(short, long, help = "Create intermediate directories and don't error if the directory exists")]
    parents: bool,

    #[arg(help = "The directory to create")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    if path.is_root() {
        bail!("Cannot create root directory");
    }
    let parent_exists = ctx.lh.list(&path.parent().as_lh_vec()).await.is_ok();
    if !args.parents && !parent_exists {
        bail!("{} does not exist, pass -p to create intermediate directories", path.parent())
    }
    let result = ctx.lh.mkdir(&path.as_lh_vec()).await;
    if !args.parents {
        result?;
    }
    Ok(())
}
