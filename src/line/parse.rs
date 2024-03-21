use anyhow::{bail, Result};

use super::lex::Token;

pub struct CommandLine {
    pub args: Vec<String>,
    pub redirect: Option<String>,
}

impl FromIterator<Token> for Result<CommandLine> {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Result<CommandLine> {
        let mut args = Vec::new();
        let mut redirect = None;
        let mut in_redirect = false;

        for token in iter {
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
}
