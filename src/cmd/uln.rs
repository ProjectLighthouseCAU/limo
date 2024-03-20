use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

// NOTE: We intentionally use a different name than 'unlink' to emphasize that
// it has different semantics than the Unix command of same name (which is
// really more similar to 'rm')

#[derive(Parser)]
#[command(bin_name = "uln")]
struct Args {
    #[arg(default_value = ".", help = "The source resource of the link to remove")]
    src_path: VirtualPathBuf,

    #[arg(default_value = ".", help = "The destination resource of the link to remove")]
    dest_path: VirtualPathBuf,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let [src_path, dest_path] = [args.src_path, args.dest_path].map(|p| ctx.cwd.join(p));
    ctx.lh.unlink(&src_path.as_lh_vec(), &dest_path.as_lh_vec()).await?;
    Ok(())
}
