use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use lighthouse_client::protocol::DirectoryTree;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "tree")]
struct Args {
    #[arg(default_value = ".", help = "The directory to list")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    print_tree(&format!("{}", ctx.cwd), Some(&response.payload), "", "");
    Ok(())
}

fn print_tree(name: &str, tree: Option<&DirectoryTree>, indent: &str, branch_indent: &str) {
    print!("{}", indent);
    if tree.is_some() {
        println!("{}", name.blue());
    } else {
        println!("{}", name);
    }
    if let Some(tree) = tree {
        let mut it = tree.entries.iter().peekable();
        while let Some((child_name, child)) = it.next() {
            let (child_indent, child_branch_indent) = if it.peek().is_some() {
                (format!("{}├──", branch_indent), format!("{}│  ", branch_indent))
            } else {
                (format!("{}└──", branch_indent), format!("{}   ", branch_indent))
            };
            print_tree(child_name, child.as_ref(), &child_indent, &child_branch_indent);
        }
    }
}
