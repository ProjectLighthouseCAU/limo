use once_cell::sync::Lazy;
use regex::Regex;

const REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&[
    r"(?<redirect>>)",
    r"'(?<singlequoted>[^']*)'",
    r#""(?<doublequoted>[^"]*)""#,
    r#"(?<strayquotes>['"]+)"#,
    r#"(?<unquoted>[^'"\s]+)"#,
].join("|")).unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Redirect,
    Arg(String),
    Invalid(String),
}

pub fn lex(line: &str) -> Vec<Token> {
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

#[cfg(test)]
mod tests {
    use crate::line::lex::{lex, Token};

    fn redirect() -> Token {
        Token::Redirect
    }

    fn arg(s: &str) -> Token {
        Token::Arg(s.to_owned())
    }

    fn invalid(s: &str) -> Token {
        Token::Invalid(s.to_owned())
    }

    #[test]
    fn whitespace() {
        assert_eq!(lex(""), vec![]);
        assert_eq!(lex("  "), vec![]);
    }

    #[test]
    fn simple_commands() {
        assert_eq!(lex("ls"), vec![arg("ls")]);
        assert_eq!(lex("ls /"), vec![arg("ls"), arg("/")]);
        assert_eq!(lex("ls hello"), vec![arg("ls"), arg("hello")]);
        assert_eq!(lex("ls /hello/ world"), vec![arg("ls"), arg("/hello/"), arg("world")]);
        assert_eq!(lex(" ls / "), vec![arg("ls"), arg("/")]);
        assert_eq!(lex(" pwd"), vec![arg("pwd")]);
        assert_eq!(lex("echo Hello world123 !"), vec![arg("echo"), arg("Hello"), arg("world123"), arg("!")]);
    }

    #[test]
    fn quotes() {
        assert_eq!(lex("''"), vec![arg("")]);
        assert_eq!(lex(r#""""#), vec![arg("")]);
        assert_eq!(lex(r#" """"  "" "#), vec![arg(""), arg(""), arg("")]);
        assert_eq!(lex(r#" "''"  "" "#), vec![arg("''"), arg("")]);
        assert_eq!(lex(r#" "''  "" "#), vec![arg("''  "), invalid("\"")]);
        assert_eq!(lex(r#"echo "Hello world "  1234"#), vec![arg("echo"), arg("Hello world "), arg("1234")]);
        assert_eq!(lex(r#"echo "Hello world   1234"#), vec![arg("echo"), invalid("\""), arg("Hello"), arg("world"), arg("1234")]);
        // TODO: Should we parse adjacent stuff as one arg?
        assert_eq!(lex(r#"echo '"Hello 'world   1234"#), vec![arg("echo"), arg("\"Hello "), arg("world"), arg("1234")]);
    }
}
