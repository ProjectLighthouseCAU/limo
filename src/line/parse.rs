use anyhow::{bail, Result};
use multipeek::{IteratorExt, MultiPeek};

use super::lex::Token;

pub enum Line {
    Assignment { lhs: String, rhs: String },
    Invocation { args: Vec<String>, redirect: Option<String> },
}

impl FromIterator<Token> for Result<Line> {
    fn from_iter<T>(iter: T) -> Result<Line> where T: IntoIterator<Item = Token> {
        parse_line(&mut iter.into_iter().multipeek())
        
    }
}

fn parse_line<T>(tokens: &mut MultiPeek<T>) -> Result<Line> where T: Iterator<Item = Token> {
    parse_assignment(tokens).or_else(|_| parse_invocation(tokens))
}

fn parse_assignment<T>(tokens: &mut MultiPeek<T>) -> Result<Line> where T: Iterator<Item = Token> {
    let Some(Token::Arg(lhs)) = tokens.peek_nth(0).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    let Some(Token::Assign) = tokens.peek_nth(1) else {
        bail!("Parse error: Expected operator (=) in assignment");
    };
    let Some(Token::Arg(rhs)) = tokens.peek_nth(2).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    Ok(Line::Assignment { lhs, rhs })
}

fn parse_invocation<T>(tokens: &mut MultiPeek<T>) -> Result<Line> where T: Iterator<Item = Token> {
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
            _ => bail!("Parse error: Unexpected {:?} in invocation, did you close your quotes?", token)
        }
    }

    Ok(Line::Invocation { args, redirect })
}
