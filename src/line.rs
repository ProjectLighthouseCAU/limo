use std::str::FromStr;

use anyhow::{bail, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{cmd, context::Context};

pub struct CommandLine {
    args: Vec<String>,
    redirect: Option<String>,
}

impl CommandLine {
    pub async fn parse_interpret(line: &str, ctx: &mut Context) -> Result<()> {
        let cmd_line = Self::from_str(line)?;
        cmd_line.interpret(ctx).await
    }

    pub async fn interpret(&self, ctx: &mut Context) -> Result<()> {
        cmd::invoke(&self.args, ctx).await
    }
}

impl FromStr for CommandLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        const REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&[
            r"(?<redirect>>)",
            r"'(?<singlequoted>[^']+)'",
            r#""(?<doublequoted>[^"]+)""#,
            r#"(?<strayquotes>['"]+)"#,
            r#"(?<unquoted>[^'"\s]+)"#,
        ].join("|")).unwrap());

        #[derive(Debug, Clone, PartialEq, Eq)]
        enum Token {
            Redirect,
            Arg(String),
            Invalid(String),
        }

        fn lex(line: &str) -> Vec<Token> {
            REGEX.captures_iter(line).map(|c| {
                if c.name("redirect").is_some() {
                    Token::Redirect
                } else if let Some(arg) = c.name("singlequoted").or(c.name("doublequoted")).or(c.name("unquoted")) {
                    Token::Arg(arg.as_str().to_owned())
                } else {
                    Token::Invalid(c[0].to_owned())
                }
            }).collect()
        }

        fn parse(tokens: Vec<Token>) -> Result<CommandLine> {
            let mut args = Vec::new();
            let mut redirect = None;
            let mut in_redirect = false;

            for token in tokens {
                match token {
                    Token::Arg(arg) => {
                        if in_redirect {
                            redirect = Some(arg);
                        } else {
                            args.push(arg);
                        }
                    },
                    Token::Redirect => in_redirect = true,
                    Token::Invalid(token) => bail!("Parse error: Found invalid token {}, did you close your quotes?", token),
                }
            }

            Ok(CommandLine { args, redirect })
        }

        parse(lex(line))
    }
}
