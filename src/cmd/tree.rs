use anyhow::Result;
use lighthouse_client::protocol::DirectoryTree;

use crate::context::Context;

pub async fn invoke(_args: &str, ctx: &mut Context) -> Result<()> {
    let response = ctx.lh.list(&ctx.cwd.as_relative().as_str_vec()).await?;
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
