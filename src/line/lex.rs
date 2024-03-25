use anyhow::{bail, Error, Result};

macro_rules! operators {
    ($(($op_name_upper:ident, $op_name_lower:ident, $op_char:literal)),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Operator {
            $($op_name_upper,)*
        }

        impl From<Operator> for char {
            fn from(op: Operator) -> Self {
                match op {
                    $(Operator::$op_name_upper => $op_char,)*
                }
            }
        }

        impl TryFrom<char> for Operator {
            type Error = Error;

            fn try_from(c: char) -> Result<Self> {
                match c {
                    $($op_char => Ok(Operator::$op_name_upper),)*
                    _ => bail!("Unknown operator: {}", c),
                }
            }
        }

        $(#[cfg(test)]
        fn $op_name_lower() -> Operator {
            Operator::$op_name_upper
        })*
    };
}

operators! {
    (Redirect, redirect, '>'),
    (Assign, assign, '='),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Operator(Operator),
    String(Vec<String>),
}

pub fn lex(line: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut current: Option<Vec<String>> = None;
    let mut it = line.chars().into_iter();
    while let Some(c) = it.next() {
        if let Ok(op) = Operator::try_from(c) { // Operator
            if let Some(current) = current.take() {
                tokens.push(Token::String(current));
            }
            current = None;
            tokens.push(Token::Operator(op));
        } else if c == '\'' || c == '"' { // (Opening) quote
            // TODO: Escapes (and interpolations for "")
            let quote = c;
            if current.is_none() {
                current = Some(vec![String::new()]);
            }
            loop {
                let Some(c) = it.next() else {
                    bail!("Unexpectedly reached end of {}-quoted string", quote);
                };
                if c == quote {
                    break;
                }
                if let Some(current) = current.as_mut() {
                    current.last_mut().unwrap().push(c);
                }
            }
        } else if c.is_whitespace() { // Whitespace
            if let Some(current) = current.take() {
                tokens.push(Token::String(current));
            }
            current = None;
        } else { // Non-whitespace
            if current.is_none() {
                current = Some(vec![String::new()]);
            }
            if let Some(current) = current.as_mut() {
                current.last_mut().unwrap().push(c);
            }
        }
    }
    if let Some(current) = current.take() {
        tokens.push(Token::String(current));
    }
    Ok(tokens)
}

#[cfg(test)]
fn op(op: Operator) -> Token {
    Token::Operator(op)
}

#[cfg(test)]
fn string(s: &str) -> Token {
    Token::String(vec![s.to_owned()])
}

#[cfg(test)]
mod tests {
    use crate::line::lex::op;

    use super::{string, assign, lex, redirect};

    #[test]
    fn whitespace() {
        assert_eq!(lex("").unwrap(), vec![]);
        assert_eq!(lex("  ").unwrap(), vec![]);
    }

    #[test]
    fn simple_commands() {
        assert_eq!(lex("ls").unwrap(), vec![string("ls")]);
        assert_eq!(lex("ls /").unwrap(), vec![string("ls"), string("/")]);
        assert_eq!(lex("ls hello").unwrap(), vec![string("ls"), string("hello")]);
        assert_eq!(lex("ls /hello/ world").unwrap(), vec![string("ls"), string("/hello/"), string("world")]);
        assert_eq!(lex(" ls / ").unwrap(), vec![string("ls"), string("/")]);
        assert_eq!(lex(" pwd").unwrap(), vec![string("pwd")]);
        assert_eq!(lex("echo Hello world123 !").unwrap(), vec![string("echo"), string("Hello"), string("world123"), string("!")]);
    }

    #[test]
    fn quotes() {
        assert_eq!(lex("''").unwrap(), vec![string("")]);
        assert_eq!(lex(r#""""#).unwrap(), vec![string("")]);
        assert_eq!(lex(r#" "" ""  "" "#).unwrap(), vec![string(""), string(""), string("")]);
        assert_eq!(lex(r#" """"  "" "#).unwrap(), vec![string(""), string("")]);
        assert_eq!(lex(r#" "''"  "" "#).unwrap(), vec![string("''"), string("")]);
        assert_eq!(lex(r#"echo "Hello world "  1234"#).unwrap(), vec![string("echo"), string("Hello world "), string("1234")]);
        assert_eq!(lex(r#"echo '"Hello 'world   1234"#).unwrap(), vec![string("echo"), string("\"Hello world"), string("1234")]);
        assert!(lex(r#" "''  "" "#).is_err());
        assert!(lex(r#"echo "Hello world   1234"#).is_err());
    }

    #[test]
    fn redirects() {
        assert_eq!(lex(">").unwrap(), vec![op(redirect())]);
        assert_eq!(lex(">>").unwrap(), vec![op(redirect()), op(redirect())]);
        assert_eq!(lex(">a").unwrap(), vec![op(redirect()), string("a")]);
        assert_eq!(lex(">1").unwrap(), vec![op(redirect()), string("1")]);
        assert_eq!(lex("  >0>  1").unwrap(), vec![op(redirect()), string("0"), op(redirect()), string("1")]);
        assert_eq!(lex("echo Test > a").unwrap(), vec![string("echo"), string("Test"), op(redirect()), string("a")]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(), vec![string("echo"), string(r#"{"x": 23,"y":3}"#), op(redirect()), string("/dev/null")])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#).unwrap(), vec![string("hello"), op(assign()), string("123")]);
        assert_eq!(lex(r#"hello  ="1""#).unwrap(), vec![string("hello"), op(assign()), string("1")]);
        assert_eq!(lex(r#"hello = "1""#).unwrap(), vec![string("hello"), op(assign()), string("1")]);
        assert_eq!(lex(r#"hello='"123"'"#).unwrap(), vec![string("hello"), op(assign()), string("\"123\"")]);
        assert_eq!(lex(r#"hello '="123"'"#).unwrap(), vec![string("hello"), string("=\"123\"")]);
        assert_eq!(lex(r#"hello'="123"'"#).unwrap(), vec![string("hello=\"123\"")]);
    }
}
