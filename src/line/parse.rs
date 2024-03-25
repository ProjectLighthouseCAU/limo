use anyhow::{bail, Result};
use multipeek::{IteratorExt, MultiPeek};

use super::lex::{lex, Token};

/// A fragment of an argument (a string fragment after evaluation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    /// A literal string fragment.
    Literal(String),
    /// A variable substitution of the form $var
    Variable(String),
    /// A cmd substitution of the form $(...)
    Command(Command),
}

/// An argument, i.e. an unevaluated string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument {
    /// The fragments of the potentially interpolated string.
    pub fragments: Vec<Fragment>
}

/// An variable assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    /// The variable name.
    pub lhs: Argument,
    /// The assigned value.
    pub rhs: Argument,
}

/// A command "expression".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Invocation { args: Vec<Argument> },
    Redirect { inner: Box<Command>, path: Argument },
}

/// A script statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Assignment(Assignment),
    Command(Command),
}

pub fn parse(line: &str) -> Result<Statement> {
    Result::<Statement>::from_iter(lex(line)?)
}

impl FromIterator<Token> for Result<Statement> {
    fn from_iter<T>(iter: T) -> Result<Statement> where T: IntoIterator<Item = Token> {
        parse_statement(&mut iter.into_iter().multipeek())
        
    }
}

fn parse_statement<T>(tokens: &mut MultiPeek<T>) -> Result<Statement> where T: Iterator<Item = Token> {
    parse_assignment(tokens)
        .map(|a| Statement::Assignment(a))
        .or_else(|_| parse_command(tokens).map(|c| Statement::Command(c)))
}

fn parse_assignment<T>(tokens: &mut MultiPeek<T>) -> Result<Assignment> where T: Iterator<Item = Token> {
    let Some(Token::String(lhs)) = tokens.peek_nth(0).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    let Some(Token::Assign) = tokens.peek_nth(1) else {
        bail!("Parse error: Expected operator (=) in assignment");
    };
    let Some(Token::String(rhs)) = tokens.peek_nth(2).cloned() else {
        bail!("Parse error: Expected variable name in assignment");
    };
    let lhs = parse_argument(&lhs)?;
    let rhs = parse_argument(&rhs)?;
    Ok(Assignment { lhs, rhs })
}

fn parse_command<T>(tokens: &mut MultiPeek<T>) -> Result<Command> where T: Iterator<Item = Token> {
    let mut args = Vec::<Argument>::new();
    let mut redirects = Vec::<Argument>::new();
    let mut in_redirect = false;

    while let Some(token) = tokens.peek().cloned() {
        match token {
            Token::String(arg) => {
                tokens.next();
                let arg = parse_argument(&arg)?;
                if in_redirect {
                    redirects.push(arg);
                } else {
                    args.push(arg);
                }
            },
            Token::Redirect => {
                tokens.next();
                in_redirect = true
            },
            _ => bail!("Parse error: Unexpected {:?} in invocation, did you close your quotes?", token)
        }
    }

    Ok(redirects.into_iter().fold(
        Command::Invocation { args },
        |inner, path| {
            Command::Redirect { inner: Box::new(inner), path }
        }
    ))
}

fn parse_argument(args: &[String]) -> Result<Argument> {
    // TODO
    let fragments = args.into_iter().map(|a| Fragment::Literal(a.to_owned())).collect();
    Ok(Argument { fragments })
}

#[cfg(test)]
mod tests {
    use super::{parse, Argument, Command, Fragment, Statement};

    fn lit(value: &str) -> Fragment {
        Fragment::Literal(value.to_owned())
    }

    fn arg(fragments: impl IntoIterator<Item = Fragment>) -> Argument {
        Argument { fragments: fragments.into_iter().collect() }
    }

    fn invocation(args: impl IntoIterator<Item = Argument>) -> Command {
        Command::Invocation { args: args.into_iter().collect() }
    }

    fn redirect(command: Command, path: Argument) -> Command {
        Command::Redirect { inner: Box::new(command), path }
    }

    fn lit_invocation(lits: impl IntoIterator<Item = &'static str>) -> Command {
        invocation(lits.into_iter().map(|l| arg([lit(l)])))
    }

    fn lit_redirect(command: Command, path: &str) -> Command {
        redirect(command, arg([lit(path)]))
    }

    fn cmd_stmt(command: Command) -> Statement {
        Statement::Command(command)
    }

    #[test]
    fn whitespace() {
        assert_eq!(parse("").unwrap(), cmd_stmt(lit_invocation([])));
        assert_eq!(parse("  ").unwrap(), cmd_stmt(lit_invocation([])));
    }

    #[test]
    fn simple_commands() {
        assert_eq!(parse("echo").unwrap(), cmd_stmt(lit_invocation(["echo"])));
        assert_eq!(parse("echo 123").unwrap(), cmd_stmt(lit_invocation(["echo", "123"])));
        assert_eq!(parse("echo \"123\"").unwrap(), cmd_stmt(lit_invocation(["echo", "123"])));
    }

    #[test]
    fn redirects() {
        assert_eq!(
            parse("cat hello >123").unwrap(),
            cmd_stmt(lit_redirect(lit_invocation(["cat", "hello"]), "123"))
        );
        assert_eq!(
            parse("echo 123   > /dev/null").unwrap(),
            cmd_stmt(lit_redirect(lit_invocation(["echo", "123"]), "/dev/null"))
        );
        assert_eq!(
            parse("\"\">a/b").unwrap(),
            cmd_stmt(lit_redirect(lit_invocation([""]), "a/b"))
        );
        assert_eq!(
            parse("\"\">a/b>c/d").unwrap(),
            cmd_stmt(lit_redirect(lit_redirect(lit_invocation([""]), "a/b"), "c/d"))
        );
        assert_eq!(
            parse("\"\">a/b  > c/d").unwrap(),
            cmd_stmt(lit_redirect(lit_redirect(lit_invocation([""]), "a/b"), "c/d"))
        );
        assert_eq!(
            parse(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(),
            cmd_stmt(lit_redirect(lit_invocation(["echo", "{\"x\": 23,\"y\":3}"]), "/dev/null"))
        );
    }

    #[test]
    fn quotes() {
        assert!(parse("'").is_err());
        assert!(parse(r#"""#).is_err());
        assert!(parse(r#" "''  "" "#).is_err());
        assert_eq!(parse("''").unwrap(), cmd_stmt(lit_invocation([""])));
        assert_eq!(parse(r#" "''"  "" "#).unwrap(), cmd_stmt(lit_invocation(["''", ""])));
    }
}
