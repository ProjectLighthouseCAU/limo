use anyhow::Result;
use clap::Parser;
use lighthouse_client::protocol::Value;

use crate::{context::Context, path::VirtualPathBuf};

// TODO: Add support for moving (potentially multiple) src resources into a directory
// TODO: Add support for moving directories

#[derive(Parser)]
#[command(bin_name = "mv")]
struct Args {
    #[arg(help = "The source path, i.e. the resource to move")]
    src_path: VirtualPathBuf,
    #[arg(help = "The destination path")]
    dest_path: VirtualPathBuf,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    let [src_path, dest_path] = [args.src_path, args.dest_path].map(|p| ctx.cwd.join(p));
    let payload: Value = ctx.lh.get(&src_path.as_lh_vec()).await?.payload;
    ctx.lh.post(&dest_path.as_lh_vec(), payload).await?;
    ctx.lh.delete(&src_path.as_lh_vec()).await?;
    Ok(String::new())
}
