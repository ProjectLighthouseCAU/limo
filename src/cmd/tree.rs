use core::fmt;
use std::ops::Add;

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

    let tree = ctx.lh.list(&path.as_lh_vec()).await?.payload;
    print_tree(&format!("{}", ctx.cwd), Some(&tree), "", "");

    let stats = Stats::from(&tree);
    println!();
    println!("{}", stats);

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
                (format!("{}├── ", branch_indent), format!("{}│   ", branch_indent))
            } else {
                (format!("{}└── ", branch_indent), format!("{}    ", branch_indent))
            };
            print_tree(child_name, child.as_ref(), &child_indent, &child_branch_indent);
        }
    }
}

struct Stats {
    directory_count: usize,
    file_count: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            directory_count: 0,
            file_count: 0,
        }
    }
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            directory_count: self.directory_count + rhs.directory_count,
            file_count: self.file_count + rhs.file_count,
        }
    }
}

impl From<&DirectoryTree> for Stats {
    fn from(tree: &DirectoryTree) -> Self {
        let mut aggregate = tree.entries.iter()
            .map(|(_, child)| {
                if let Some(child) = child {
                    Self::from(child)
                } else {
                    Self { file_count: 1, ..Default::default() }
                }
            })
            .reduce(Add::add)
            .unwrap_or_default();
        aggregate.directory_count += 1;
        aggregate
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.directory_count == 1 {
            write!(f, "{} directory", self.directory_count)?;
        } else {
            write!(f, "{} directories", self.directory_count)?;
        }
        write!(f, ", ")?;
        if self.file_count == 1 {
            write!(f, "{} file", self.file_count)?;
        } else {
            write!(f, "{} files", self.file_count)?;
        }
        Ok(())
    }
}
