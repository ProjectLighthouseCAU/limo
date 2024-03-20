use anyhow::Result;
use clap::Parser;
use lighthouse_client::protocol::DirectoryTree;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "tree")]
struct Args {
    #[arg(default_value = "")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    print_tree(&format!("{}", ctx.cwd), Some(&response.payload), 0);
    Ok(())
}

// TODO: Add unicode "branches"

fn print_tree(name: &str, tree: Option<&DirectoryTree>, indent: i32) {
    for _ in 0..indent {
        print!(" ");
    }
    println!("{}", name);
    if let Some(tree) = tree {
        for (child_name, child) in &tree.entries {
            print_tree(child_name, child.as_ref(), indent + 2);
        }
    }
}
