use anyhow::Result;

use crate::context::Context;

use self::{interpret::interpret, lex::lex, parse::Line};

mod interpret;
mod lex;
mod parse;

pub async fn parse_interpret(line: &str, ctx: &mut Context) -> Result<()> {
    let tokens = lex(line);
    let line = Result::<Line>::from_iter(tokens)?;
    interpret(line, ctx).await
}
