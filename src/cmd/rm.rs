use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

// TODO: Add a failsafe to only delete individual resources and not directories
// We could support the classic -rf flags for that.

#[derive(Parser)]
#[command(bin_name = "rm")]
struct Args {
    #[arg(default_value = ".")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    ctx.lh.delete(&path.as_lh_vec()).await?;
    Ok(())
}
