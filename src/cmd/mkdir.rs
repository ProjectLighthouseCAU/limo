use anyhow::{bail, Result};
use clap::Parser;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "mkdir")]
struct Args {
    #[arg(short, long, help = "Create intermediate directories and don't error if the directory exists")]
    parents: bool,

    #[arg(required = true, help = "The directories to create")]
    paths: Vec<VirtualPathBuf>,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    for path in args.paths {
        let path = ctx.cwd.join(path);
        if path.is_root() {
            bail!("Cannot create root directory");
        }

        let parent_exists = ctx.lh.list(&path.parent().as_lh_vec()).await.is_ok();

        if parent_exists {
            let result = ctx.lh.mkdir(&path.as_lh_vec()).await;
            if !args.parents {
                result?;
            }
        } else {
            if !args.parents {
                bail!("{} does not exist, pass -p to create intermediate directories", path.parent())
            }

            let lh_path = path.as_lh_vec();
            for i in 1..=lh_path.len() {
                ctx.lh.mkdir(&lh_path[..i]).await?;
            }
        }
        
    }
    Ok(String::new())
}
