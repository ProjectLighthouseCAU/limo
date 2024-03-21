use anyhow::Result;

use crate::context::Context;

use self::{interpret::interpret, lex::lex, parse::CommandLine};

mod interpret;
mod lex;
mod parse;

pub async fn parse_interpret(line: &str, ctx: &mut Context) -> Result<()> {
    let tokens = lex(line);
    let cmd_line = Result::<CommandLine>::from_iter(tokens)?;
    interpret(cmd_line, ctx).await
}
