use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "touch")]
struct Args {
    #[arg(required = true, help = "The resources to create")]
    paths: Vec<VirtualPathBuf>,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    for path in args.paths {
        let path = ctx.cwd.join(path);
        ctx.lh.create(&path.as_lh_vec()).await?;
    }
    Ok(String::new())
}
