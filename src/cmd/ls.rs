use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "ls")]
struct Args {
    #[arg(default_value = "")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    let entries: Vec<_> = response.payload.entries.into_keys().collect();
    println!("{}", entries.join("   "));
    Ok(())
}
