use anyhow::Result;

use crate::context::Context;

use self::{interpret::interpret, parse::parse};

mod interpret;
mod lex;
mod parse;

pub async fn parse_interpret(line: &str, ctx: &mut Context) -> Result<()> {
    let stmt = parse(line)?;
    interpret(stmt, ctx).await
}
