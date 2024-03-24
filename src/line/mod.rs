use anyhow::Result;

use crate::context::Context;

use self::{interpret::interpret, lex::lex, parse::Statement};

mod interpret;
mod lex;
mod parse;

pub async fn parse_interpret(line: &str, ctx: &mut Context) -> Result<()> {
    let tokens = lex(line);
    let stmt = Result::<Statement>::from_iter(tokens)?;
    interpret(stmt, ctx).await
}
