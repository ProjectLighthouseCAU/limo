use anyhow::{bail, Result};
use lighthouse_client::protocol::{to_value, Value};

use crate::{cmd, context::Context, path::VirtualPathBuf};

use super::parse::CommandLine;

pub async fn interpret(line: CommandLine, ctx: &mut Context) -> Result<()> {
    if line.args.is_empty() {
        bail!("Cannot interpret empty line");
    }

    let output = cmd::invoke(&line.args, ctx).await?;

    if let Some(path) = line.redirect {
        // The redirected output is interpreted as JSON and then written as MessagePack
        let path = ctx.cwd.join(VirtualPathBuf::from(path.as_str()));
        let json_value: serde_json::Value = serde_json::from_str(&output)?;
        let lh_value: Value = to_value(json_value)?;
        ctx.lh.post(&path.as_lh_vec(), lh_value).await?;
    } else if !output.is_empty() {
        // Print output if not redirected
        println!("{}", output.trim());
    }

    Ok(())
}
