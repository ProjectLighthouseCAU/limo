use core::fmt;
use std::ops::Add;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use lighthouse_client::protocol::DirectoryTree;

use crate::{context::Context, path::{VirtualPath, VirtualPathBuf}};

#[derive(Parser)]
#[command(bin_name = "tree")]
struct Args {
    #[arg(default_value = ".", help = "The directory to list")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);

    let tree = ctx.lh.list(&path.as_lh_vec()).await?.payload;

    Ok(format!("{}", TreeListing {
        tree,
        cwd: &ctx.cwd,
    }))
}

struct TreeListing<'a> {
    tree: DirectoryTree,
    cwd: &'a VirtualPath,
}

impl<'a> fmt::Display for TreeListing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tree(f, &format!("{}", self.cwd), Some(&self.tree), "", "")?;

        let stats = Stats::from(&self.tree);
        writeln!(f)?;
        writeln!(f, "{}", stats)?;
        
        Ok(())
    }
}

fn write_tree(f: &mut fmt::Formatter<'_>, name: &str, tree: Option<&DirectoryTree>, indent: &str, branch_indent: &str) -> fmt::Result {
    write!(f, "{}", indent)?;
    if tree.is_some() {
        writeln!(f, "{}", name.blue())?;
    } else {
        writeln!(f, "{}", name)?;
    }
    if let Some(tree) = tree {
        let mut entries: Vec<_> = tree.entries.iter().collect();
        entries.sort_by_key(|(child_name, _)| *child_name);
        let mut it = entries.into_iter().peekable();
        while let Some((child_name, child)) = it.next() {
            let (child_indent, child_branch_indent) = if it.peek().is_some() {
                (format!("{}├── ", branch_indent), format!("{}│   ", branch_indent))
            } else {
                (format!("{}└── ", branch_indent), format!("{}    ", branch_indent))
            };
            write_tree(f, child_name, child.as_ref(), &child_indent, &child_branch_indent)?;
        }
    }
    Ok(())
}

struct Stats {
    directory_count: usize,
    resource_count: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            directory_count: 0,
            resource_count: 0,
        }
    }
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            directory_count: self.directory_count + rhs.directory_count,
            resource_count: self.resource_count + rhs.resource_count,
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
                    Self { resource_count: 1, ..Default::default() }
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
        if self.resource_count == 1 {
            write!(f, "{} resource", self.resource_count)?;
        } else {
            write!(f, "{} resources", self.resource_count)?;
        }
        Ok(())
    }
}
