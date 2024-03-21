use once_cell::sync::Lazy;
use regex::Regex;

const REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&[
    r"(?<redirect>>)",
    r"'(?<singlequoted>[^']+)'",
    r#""(?<doublequoted>[^"]+)""#,
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
