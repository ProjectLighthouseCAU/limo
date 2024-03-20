use anyhow::Result;
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "ls")]
struct Args {
    #[arg(short, long, action, help = "Include . and ..")]
    all: bool,

    #[arg(short, long, action, help = "Use a long listing format")]
    long: bool,

    #[arg(default_value = ".", help = "The directory to list")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    let mut entries: Vec<_> = response.payload.entries.into_keys().collect();

    if args.all {
        entries.extend([".", ".."].map(|s| s.to_string()));
    }

    entries.sort();

    if args.long {
        // TODO: Print more metadata
        for entry in entries {
            println!("{}", entry);
        }
    } else {
        println!("{}", entries.join("   "));
    }

    Ok(())
}
