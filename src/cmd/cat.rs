use anyhow::Result;
use clap::Parser;
use lighthouse_client::protocol::Value;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "cat")]
struct Args {
    #[arg(default_value = ".", help = "The resource to output")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let result: Value = ctx.lh.get(&args.path.as_lh_vec()).await?.payload;
    println!("{}", result);
    Ok(())
}
