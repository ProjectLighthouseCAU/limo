use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "ln")]
struct Args {
    #[arg(default_value = ".", help = "The source resource from which to forward events")]
    src_path: VirtualPathBuf,

    #[arg(default_value = ".", help = "The destination resource to which to forward events")]
    dest_path: VirtualPathBuf,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    let [src_path, dest_path] = [args.src_path, args.dest_path].map(|p| ctx.cwd.join(p));
    ctx.lh.link(&src_path.as_lh_vec(), &dest_path.as_lh_vec()).await?;
    Ok(String::new())
}
