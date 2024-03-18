use anyhow::Result;
use lighthouse_client::protocol::Value;

use crate::{context::Context, path::VirtualPathBuf};

pub async fn invoke(args: &str, ctx: &mut Context) -> Result<()> {
    let path = ctx.cwd.join(VirtualPathBuf::from(args));
    let result: Value = ctx.lh.get(&path.as_lh_vec()).await?.payload;
    println!("{}", result);
    Ok(())
}
