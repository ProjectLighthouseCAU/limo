use anyhow::{bail, Error, Result};

macro_rules! operators {
    ($(($op_name_upper:ident, $op_name_lower:ident, $op_char:literal)),* $(,)?) => {
        /// An operator understood by the lexer.
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

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Operator(Operator),
    String(Vec<String>),
}

/// Tokenizes the line. This handles quoting and removes whitespace.
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
            // TODO: Command interpolation for "-quotes
            let quote = c;
            let mut is_escaped = false;
            let mut in_interpolation = false;
            if current.is_none() {
                current = Some(vec![String::new()]);
            }
            loop {
                let Some(c) = it.next() else {
                    bail!("Unexpectedly reached end of {}-quoted string", quote);
                };
                if !is_escaped && c == '\\' { // Escape backslash
                    is_escaped = true;
                } else if !is_escaped && quote == '"' && c == '$' { // Variable interpolation
                    in_interpolation = true;
                } else { // Escaped or normal character
                    if !is_escaped && c == quote {
                        break;
                    }
                    if let Some(current) = current.as_mut() {
                        current.last_mut().unwrap().push(c);
                    }
                    is_escaped = false;
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
fn string(s: impl IntoIterator<Item = &'static str>) -> Token {
    Token::String(s.into_iter().map(|s| s.to_owned()).collect())
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
        assert_eq!(lex("ls").unwrap(), vec![string(["ls"])]);
        assert_eq!(lex("ls /").unwrap(), vec![string(["ls"]), string(["/"])]);
        assert_eq!(lex("ls hello").unwrap(), vec![string(["ls"]), string(["hello"])]);
        assert_eq!(lex("ls /hello/ world").unwrap(), vec![string(["ls"]), string(["/hello/"]), string(["world"])]);
        assert_eq!(lex(" ls / ").unwrap(), vec![string(["ls"]), string(["/"])]);
        assert_eq!(lex(" pwd").unwrap(), vec![string(["pwd"])]);
        assert_eq!(lex("echo Hello world123 !").unwrap(), vec![string(["echo"]), string(["Hello"]), string(["world123"]), string(["!"])]);
    }

    #[test]
    fn quotes() {
        assert_eq!(lex("''").unwrap(), vec![string([""])]);
        assert_eq!(lex(r#""""#).unwrap(), vec![string([""])]);
        assert_eq!(lex(r#" "" ""  "" "#).unwrap(), vec![string([""]), string([""]), string([""])]);
        assert_eq!(lex(r#" """"  "" "#).unwrap(), vec![string([""]), string([""])]);
        assert_eq!(lex(r#" "''"  "" "#).unwrap(), vec![string(["''"]), string([""])]);
        assert_eq!(lex(r#"echo "Hello world "  1234"#).unwrap(), vec![string(["echo"]), string(["Hello world "]), string(["1234"])]);
        assert_eq!(lex(r#"echo '"Hello 'world   1234"#).unwrap(), vec![string(["echo"]), string(["\"Hello world"]), string(["1234"])]);
        assert!(lex(r#" "''  "" "#).is_err());
        assert!(lex(r#"echo "Hello world   1234"#).is_err());
    }

    #[test]
    fn escapes() {
        assert!(lex("'''").is_err());
        assert_eq!(lex(r#"'\''"#).unwrap(), vec![string(["'"])]);
        assert_eq!(lex(r#""\'""#).unwrap(), vec![string(["'"])]);
        assert_eq!(lex(r#"'\"'"#).unwrap(), vec![string(["\""])]);
        assert_eq!(lex(r#"Hello " \"world\"""#).unwrap(), vec![string(["Hello"]), string([" \"world\""])]);
        assert_eq!(lex(r#""\\""#).unwrap(), vec![string(["\\"])]);
        assert_eq!(lex(r#"'This\\\\is a double escape'"#).unwrap(), vec![string(["This\\\\is a double escape"])]);
        assert_eq!(lex(r#""Escaped dollar sign: \$test 123""#).unwrap(), vec![string(["Escaped dollar sign: $test 123"])]);
        assert!(lex(r#"'Unclosed: \\\'"#).is_err());
        // TODO: We should handle backslashes outside quoted contexts too
        assert_eq!(lex("\\").unwrap(), vec![string(["\\"])]);
        // TODO: Should we insert the backslash with unrecognized characters? Or error?
        assert_eq!(lex(r#"'\another char'"#).unwrap(), vec![string(["another char"])]);
    }

    #[test]
    fn interpolations() {
        assert_eq!(lex("'test$x'").unwrap(), vec![string(["test$x"])]);
        assert_eq!(lex(r#""test$x""#).unwrap(), vec![string(["testx"])]); // FIXME
    }

    #[test]
    fn redirects() {
        assert_eq!(lex(">").unwrap(), vec![op(redirect())]);
        assert_eq!(lex(">>").unwrap(), vec![op(redirect()), op(redirect())]);
        assert_eq!(lex(">a").unwrap(), vec![op(redirect()), string(["a"])]);
        assert_eq!(lex(">1").unwrap(), vec![op(redirect()), string(["1"])]);
        assert_eq!(lex("  >0>  1").unwrap(), vec![op(redirect()), string(["0"]), op(redirect()), string(["1"])]);
        assert_eq!(lex("echo Test > a").unwrap(), vec![string(["echo"]), string(["Test"]), op(redirect()), string(["a"])]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(), vec![string(["echo"]), string([r#"{"x": 23,"y":3}"#]), op(redirect()), string(["/dev/null"])])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#).unwrap(), vec![string(["hello"]), op(assign()), string(["123"])]);
        assert_eq!(lex(r#"hello  ="1""#).unwrap(), vec![string(["hello"]), op(assign()), string(["1"])]);
        assert_eq!(lex(r#"hello = "1""#).unwrap(), vec![string(["hello"]), op(assign()), string(["1"])]);
        assert_eq!(lex(r#"hello='"123"'"#).unwrap(), vec![string(["hello"]), op(assign()), string(["\"123\""])]);
        assert_eq!(lex(r#"hello '="123"'"#).unwrap(), vec![string(["hello"]), string(["=\"123\""])]);
        assert_eq!(lex(r#"hello'="123"'"#).unwrap(), vec![string(["hello=\"123\""])]);
    }
}
