use anyhow::Result;
use clap::Parser;
use lighthouse_client::protocol::Value;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "cp")]
struct Args {
    #[arg(help = "The source path, i.e. the resource to copy")]
    src_path: VirtualPathBuf,
    #[arg(help = "The destination path")]
    dest_path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let [src_path, dest_path] = [args.src_path, args.dest_path].map(|p| ctx.cwd.join(p));
    let payload: Value = ctx.lh.get(&src_path.as_lh_vec()).await?.payload;
    ctx.lh.post(&dest_path.as_lh_vec(), payload).await?;
    Ok(())
}
