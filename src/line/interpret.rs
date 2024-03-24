use anyhow::{bail, Result};
use async_recursion::async_recursion;
use lighthouse_client::protocol::{to_value, Value};

use crate::{cmd, context::Context, path::VirtualPathBuf};

use super::parse::{Argument, Command, Fragment, Statement};

pub async fn interpret(stmt: Statement, ctx: &mut Context) -> Result<()> {
    match stmt {
        Statement::Assignment(_) => {
            todo!()
        },
        Statement::Command(command) => {
            let interpretation = interpret_command(command, ctx).await?;
            if !interpretation.redirected {
                // Print output if not redirected
                println!("{}", interpretation.output.trim());
            }
        },
    }
    Ok(())
}

struct Interpretation {
    output: String,
    redirected: bool,
}

#[async_recursion]
async fn interpret_command(command: Command, ctx: &mut Context) -> Result<Interpretation> {
    match command {
        Command::Invocation { args } => {
            if args.is_empty() {
                bail!("Cannot interpret empty invocation");
            }
            let args = evaluate_arguments(args, ctx).await?;
            let output = cmd::invoke(&args, ctx).await?;
            Ok(Interpretation {
                output,
                redirected: false,
            })
        },
        Command::Redirect { inner, path } => {
            // The redirected output is interpreted as JSON and then written as MessagePack
            let inner = interpret_command(*inner, ctx).await?;
            let path = evaluate_argument(path, ctx).await?;
            let path = ctx.cwd.join(VirtualPathBuf::from(path.as_str()));
            let json_value: serde_json::Value = serde_json::from_str(&inner.output)?;
            let lh_value: Value = to_value(json_value)?;
            ctx.lh.post(&path.as_lh_vec(), lh_value).await?;
            Ok(Interpretation {
                output: inner.output,
                redirected: true,
            })
        },
    }
}

async fn evaluate_arguments(args: Vec<Argument>, ctx: &mut Context) -> Result<Vec<String>> {
    let mut evaluated = Vec::new();
    for arg in args {
        evaluated.push(evaluate_argument(arg, ctx).await?);
    }
    Ok(evaluated)
}

async fn evaluate_argument(arg: Argument, ctx: &mut Context) -> Result<String> {
    let mut evaluated = String::new();
    for fragment in arg.fragments {
        evaluated.push_str(&evaluate_fragment(fragment, ctx).await?);
    }
    Ok(evaluated)
}

async fn evaluate_fragment(fragment: Fragment, ctx: &mut Context) -> Result<String> {
    match fragment {
        Fragment::Literal(lit) => Ok(lit),
        _ => todo!()
    }
}
