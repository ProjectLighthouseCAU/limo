use anyhow::{bail, Result};
use multipeek::{IteratorExt, MultiPeek};

use super::lex::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Assignment { lhs: String, rhs: String },
    Invocation { args: Vec<String>, redirect: Option<String> },
}

impl FromIterator<Token> for Result<Statement> {
    fn from_iter<T>(iter: T) -> Result<Statement> where T: IntoIterator<Item = Token> {
        parse_statement(&mut iter.into_iter().multipeek())
        
    }
}

fn parse_statement<T>(tokens: &mut MultiPeek<T>) -> Result<Statement> where T: Iterator<Item = Token> {
    parse_assignment(tokens).or_else(|_| parse_invocation(tokens))
}

fn parse_assignment<T>(tokens: &mut MultiPeek<T>) -> Result<Statement> where T: Iterator<Item = Token> {
    let Some(Token::Arg(lhs)) = tokens.peek_nth(0).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    let Some(Token::Assign) = tokens.peek_nth(1) else {
        bail!("Parse error: Expected operator (=) in assignment");
    };
    let Some(Token::Arg(rhs)) = tokens.peek_nth(2).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    Ok(Statement::Assignment { lhs, rhs })
}

fn parse_invocation<T>(tokens: &mut MultiPeek<T>) -> Result<Statement> where T: Iterator<Item = Token> {
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

    Ok(Statement::Invocation { args, redirect })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::line::lex::lex;

    use super::Statement;

    fn parse(line: &str) -> Result<Statement> {
        Result::<Statement>::from_iter(lex(line))
    }

    fn invocation(args: &[&str], redirect: Option<&str>) -> Statement {
        Statement::Invocation {
            args: args.into_iter().map(|&s| s.to_owned()).collect(),
            redirect: redirect.map(|s| s.to_owned())
        }
    }

    #[test]
    fn whitespace() {
        assert_eq!(parse("").unwrap(), invocation(&[], None));
        assert_eq!(parse("  ").unwrap(), invocation(&[], None));
    }

    #[test]
    fn simple_commands() {
        assert_eq!(parse("echo").unwrap(), invocation(&["echo"], None));
        assert_eq!(parse("echo 123").unwrap(), invocation(&["echo", "123"], None));
        assert_eq!(parse("echo \"123\"").unwrap(), invocation(&["echo", "123"], None));
    }

    #[test]
    fn redirects() {
        assert_eq!(parse("cat hello >123").unwrap(), invocation(&["cat", "hello"], Some("123")));
        assert_eq!(parse("echo 123   > /dev/null").unwrap(), invocation(&["echo", "123"], Some("/dev/null")));
        assert_eq!(parse("\"\">a/b").unwrap(), invocation(&[""], Some("a/b")));
        assert_eq!(parse("\"\">a/b>c/d").unwrap(), invocation(&[""], Some("c/d")));
        assert_eq!(parse("\"\">a/b  > c/d").unwrap(), invocation(&[""], Some("c/d")));
        assert_eq!(parse(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(), invocation(&["echo", "{\"x\": 23,\"y\":3}"], Some("/dev/null")));
    }

    #[test]
    fn quotes() {
        assert!(parse("'").is_err());
        assert!(parse(r#"""#).is_err());
        assert!(parse(r#" "''  "" "#).is_err());
        assert_eq!(parse("''").unwrap(), invocation(&[""], None));
        assert_eq!(parse(r#" "''"  "" "#).unwrap(), invocation(&["''", ""], None));
    }
}
