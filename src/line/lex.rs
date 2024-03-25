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

/// The kind of string segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentKind {
    /// A literal string segment.
    Literal,
    /// A variable interpolation segment.
    Variable,
}

/// A fragment of a string token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    /// The text of the segment. Excludes the interpolation character ($) if not literal.
    pub text: String,
    /// The kind of string segment. Indicates e.g. whether this segment represents an interpolation.
    pub kind: SegmentKind,
}

impl Segment {
    pub fn empty_literal() -> Self {
        Self { text: String::new(), kind: SegmentKind::Literal }
    }

    pub fn empty_variable() -> Self {
        Self { text: String::new(), kind: SegmentKind::Variable }
    }
}

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Operator(Operator),
    String(Vec<Segment>),
}

const INTERPOLATION_CHAR: char = '$';
const ESCAPE_CHAR: char = '\\';

/// Tokenizes the line. This handles quoting and removes whitespace.
pub fn lex(line: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut current: Option<Vec<Segment>> = None;
    let mut it = line.chars().into_iter();
    while let Some(c) = it.next() {
        if let Ok(op) = Operator::try_from(c) { // Operator
            if let Some(current) = current.take() {
                tokens.push(Token::String(current));
            }
            current = None;
            tokens.push(Token::Operator(op));
        } else if c == '\'' || c == '"' { // Opening quote
            let quote = c;
            let mut is_escaped = false;
            let mut in_interpolation = false;
            if current.is_none() {
                current = Some(vec![Segment::empty_literal()]);
            }
            loop {
                let Some(c) = it.next() else {
                    bail!("Unexpectedly reached end of {}-quoted string", quote);
                };
                if !is_escaped && c == ESCAPE_CHAR {
                    is_escaped = true;
                } else if !is_escaped && quote == '"' && c == INTERPOLATION_CHAR {
                    // Entering interpolation
                    in_interpolation = true;
                    current.as_mut().unwrap().push(Segment::empty_variable());
                } else {
                    if !is_escaped {
                        if in_interpolation && !c.is_ascii_alphanumeric() && c != '_' {
                            // Exiting interpolation
                            in_interpolation = false;
                            current.as_mut().unwrap().push(Segment::empty_literal());
                        }
                        if c == quote {
                            break;
                        }
                    }
                    current.as_mut().unwrap().last_mut().unwrap().text.push(c);
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
                current = Some(vec![Segment::empty_literal()]);
            }
            if c == INTERPOLATION_CHAR { // Unquoted interpolation
                current.as_mut().unwrap().push(Segment::empty_variable());
            } else {
                current.as_mut().unwrap().last_mut().unwrap().text.push(c);
            }
        }
    }
    if let Some(current) = current.take() {
        tokens.push(Token::String(current));
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{assign, lex, redirect, Operator, Segment, SegmentKind, Token};

    fn op(op: Operator) -> Token {
        Token::Operator(op)
    }

    fn lit(s: &str) -> Segment {
        Segment { text: s.to_owned(), kind: SegmentKind::Literal }
    }

    fn var(s: &str) -> Segment {
        Segment { text: s.to_owned(), kind: SegmentKind::Variable }
    }

    fn string(s: impl IntoIterator<Item = Segment>) -> Token {
        Token::String(s.into_iter().map(|s| s.to_owned()).collect())
    }

    fn lit_string(s: impl IntoIterator<Item = &'static str>) -> Token {
        string(s.into_iter().map(lit))
    }

    #[test]
    fn whitespace() {
        assert_eq!(lex("").unwrap(), vec![]);
        assert_eq!(lex("  ").unwrap(), vec![]);
    }

    #[test]
    fn simple_commands() {
        assert_eq!(lex("ls").unwrap(), vec![lit_string(["ls"])]);
        assert_eq!(lex("ls /").unwrap(), vec![lit_string(["ls"]), lit_string(["/"])]);
        assert_eq!(lex("ls hello").unwrap(), vec![lit_string(["ls"]), lit_string(["hello"])]);
        assert_eq!(lex("ls /hello/ world").unwrap(), vec![lit_string(["ls"]), lit_string(["/hello/"]), lit_string(["world"])]);
        assert_eq!(lex(" ls / ").unwrap(), vec![lit_string(["ls"]), lit_string(["/"])]);
        assert_eq!(lex(" pwd").unwrap(), vec![lit_string(["pwd"])]);
        assert_eq!(lex("echo Hello world123 !").unwrap(), vec![lit_string(["echo"]), lit_string(["Hello"]), lit_string(["world123"]), lit_string(["!"])]);
    }

    #[test]
    fn quotes() {
        assert_eq!(lex("''").unwrap(), vec![lit_string([""])]);
        assert_eq!(lex(r#""""#).unwrap(), vec![lit_string([""])]);
        assert_eq!(lex(r#" "" ""  "" "#).unwrap(), vec![lit_string([""]), lit_string([""]), lit_string([""])]);
        assert_eq!(lex(r#" """"  "" "#).unwrap(), vec![lit_string([""]), lit_string([""])]);
        assert_eq!(lex(r#" "''"  "" "#).unwrap(), vec![lit_string(["''"]), lit_string([""])]);
        assert_eq!(lex(r#"echo "Hello world "  1234"#).unwrap(), vec![lit_string(["echo"]), lit_string(["Hello world "]), lit_string(["1234"])]);
        assert_eq!(lex(r#"echo '"Hello 'world   1234"#).unwrap(), vec![lit_string(["echo"]), lit_string(["\"Hello world"]), lit_string(["1234"])]);
        assert!(lex(r#" "''  "" "#).is_err());
        assert!(lex(r#"echo "Hello world   1234"#).is_err());
    }

    #[test]
    fn escapes() {
        assert!(lex("'''").is_err());
        assert_eq!(lex(r#"'\''"#).unwrap(), vec![lit_string(["'"])]);
        assert_eq!(lex(r#""\'""#).unwrap(), vec![lit_string(["'"])]);
        assert_eq!(lex(r#"'\"'"#).unwrap(), vec![lit_string(["\""])]);
        assert_eq!(lex(r#"Hello " \"world\"""#).unwrap(), vec![lit_string(["Hello"]), lit_string([" \"world\""])]);
        assert_eq!(lex(r#""\\""#).unwrap(), vec![lit_string(["\\"])]);
        assert_eq!(lex(r#"'This\\\\is a double escape'"#).unwrap(), vec![lit_string(["This\\\\is a double escape"])]);
        assert_eq!(lex(r#""Escaped dollar sign: \$test 123""#).unwrap(), vec![lit_string(["Escaped dollar sign: $test 123"])]);
        assert!(lex(r#"'Unclosed: \\\'"#).is_err());
        // TODO: We should handle backslashes outside quoted contexts too
        assert_eq!(lex("\\").unwrap(), vec![lit_string(["\\"])]);
        // TODO: Should we insert the backslash with unrecognized characters? Or error?
        assert_eq!(lex(r#"'\another char'"#).unwrap(), vec![lit_string(["another char"])]);
    }

    #[test]
    fn interpolations() {
        assert_eq!(lex("$x").unwrap(), vec![string([lit(""), var("x")])]);
        assert_eq!(lex("test$x").unwrap(), vec![string([lit("test"), var("x")])]);
        assert_eq!(lex("test$x$y").unwrap(), vec![string([lit("test"), var("x"), var("y")])]);
        assert_eq!(lex("test $x").unwrap(), vec![lit_string(["test"]), string([lit(""), var("x")])]);
        assert_eq!(lex("'test$x'").unwrap(), vec![lit_string(["test$x"])]);
        assert_eq!(lex(r#""$abc""#).unwrap(), vec![string([lit(""), var("abc"), lit("")])]);
        assert_eq!(lex(r#""$abc$ghi""#).unwrap(), vec![string([lit(""), var("abc"), var("ghi"), lit("")])]);
        assert_eq!(lex(r#""test$x""#).unwrap(), vec![string([lit("test"), var("x"), lit("")])]);
        assert_eq!(lex(r#""$var_with_underscore abc""#).unwrap(), vec![string([lit(""), var("var_with_underscore"), lit(" abc")])]);
        assert_eq!(lex(r#""$var_with-hyphen""#).unwrap(), vec![string([lit(""), var("var_with"), lit("-hyphen")])]);
        assert_eq!(lex(r#""/$var_with/slash""#).unwrap(), vec![string([lit("/"), var("var_with"), lit("/slash")])]);
    }

    #[test]
    fn redirects() {
        assert_eq!(lex(">").unwrap(), vec![op(redirect())]);
        assert_eq!(lex(">>").unwrap(), vec![op(redirect()), op(redirect())]);
        assert_eq!(lex(">a").unwrap(), vec![op(redirect()), lit_string(["a"])]);
        assert_eq!(lex(">1").unwrap(), vec![op(redirect()), lit_string(["1"])]);
        assert_eq!(lex("  >0>  1").unwrap(), vec![op(redirect()), lit_string(["0"]), op(redirect()), lit_string(["1"])]);
        assert_eq!(lex("echo Test > a").unwrap(), vec![lit_string(["echo"]), lit_string(["Test"]), op(redirect()), lit_string(["a"])]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(), vec![lit_string(["echo"]), string([r#"{"x": 23,"y":3}"#].map(lit)), op(redirect()), lit_string(["/dev/null"])])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#).unwrap(), vec![lit_string(["hello"]), op(assign()), lit_string(["123"])]);
        assert_eq!(lex(r#"hello  ="1""#).unwrap(), vec![lit_string(["hello"]), op(assign()), lit_string(["1"])]);
        assert_eq!(lex(r#"hello = "1""#).unwrap(), vec![lit_string(["hello"]), op(assign()), lit_string(["1"])]);
        assert_eq!(lex(r#"hello='"123"'"#).unwrap(), vec![lit_string(["hello"]), op(assign()), lit_string(["\"123\""])]);
        assert_eq!(lex(r#"hello '="123"'"#).unwrap(), vec![lit_string(["hello"]), lit_string(["=\"123\""])]);
        assert_eq!(lex(r#"hello'="123"'"#).unwrap(), vec![lit_string(["hello=\"123\""])]);
    }
}
