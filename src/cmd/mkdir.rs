use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "mkdir")]
struct Args {
    #[arg(default_value = "")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    ctx.lh.mkdir(&path.as_lh_vec()).await?;
    Ok(())
}
