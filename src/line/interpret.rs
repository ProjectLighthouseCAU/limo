use anyhow::{bail, Result};
use lighthouse_client::protocol::{to_value, Value};

use crate::{cmd, context::Context, path::VirtualPathBuf};

use super::parse::Statement;

pub async fn interpret(stmt: Statement, ctx: &mut Context) -> Result<()> {
    match stmt {
        Statement::Assignment { lhs, rhs } => {
            todo!()
        },
        Statement::Invocation { args, redirect } => {
            if args.is_empty() {
                bail!("Cannot interpret empty invocation");
            }

            let output = cmd::invoke(&args, ctx).await?;

            if let Some(path) = redirect {
                // The redirected output is interpreted as JSON and then written as MessagePack
                let path = ctx.cwd.join(VirtualPathBuf::from(path.as_str()));
                let json_value: serde_json::Value = serde_json::from_str(&output)?;
                let lh_value: Value = to_value(json_value)?;
                ctx.lh.post(&path.as_lh_vec(), lh_value).await?;
            } else if !output.is_empty() {
                // Print output if not redirected
                println!("{}", output.trim());
            }
        },
    }
    Ok(())
}
