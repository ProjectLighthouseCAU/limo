use anyhow::{bail, Result};

macro_rules! lexer {
    ($(($op_name_upper:ident, $op_name_lower:ident, $op_char:literal)),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Token {
            $($op_name_upper,)*
            String(Vec<String>),
            Invalid(String),
        }

        pub fn lex(line: &str) -> Result<Vec<Token>> {
            let mut tokens = Vec::<Token>::new();
            let mut current: Option<Vec<String>> = None;
            let mut it = line.chars().into_iter();
            while let Some(c) = it.next() {
                match c {
                    $($op_char => {
                        if let Some(current) = current.take() {
                            tokens.push(Token::String(current));
                        }
                        current = None;
                        tokens.push(Token::$op_name_upper)
                    },)*
                    '\'' => {
                        // TODO: Escapes
                        if current.is_none() {
                            current = Some(vec![String::new()]);
                        }
                        loop {
                            let Some(c) = it.next() else {
                                bail!("Unexpectedly reached end of single quoted string");
                            };
                            if c == '\'' {
                                break;
                            }
                            if let Some(current) = current.as_mut() {
                                current.last_mut().unwrap().push(c);
                            }
                        }
                    },
                    '"' => {
                        // TODO: Escapes and interpolations
                        if current.is_none() {
                            current = Some(vec![String::new()]);
                        }
                        loop {
                            let Some(c) = it.next() else {
                                bail!("Unexpectedly reached end of double quoted string");
                            };
                            if c == '"' {
                                break;
                            }
                            if let Some(current) = current.as_mut() {
                                current.last_mut().unwrap().push(c);
                            }
                        }
                    },
                    _ => {
                        if c.is_whitespace() {
                            if let Some(current) = current.take() {
                                tokens.push(Token::String(current));
                            }
                            current = None;
                        } else {
                            if current.is_none() {
                                current = Some(vec![String::new()]);
                            }
                            if let Some(current) = current.as_mut() {
                                current.last_mut().unwrap().push(c);
                            }
                        }
                    },
                }
            }
            if let Some(current) = current.take() {
                tokens.push(Token::String(current));
            }
            Ok(tokens)
        }

        // Convenience functions for testing

        $(
            #[cfg(test)]
            fn $op_name_lower() -> Token {
                Token::$op_name_upper
            }
        )*

        #[cfg(test)]
        fn string(s: &str) -> Token {
            Token::String(vec![s.to_owned()])
        }

        #[cfg(test)]
        fn invalid(s: &str) -> Token {
            Token::Invalid(s.to_owned())
        }
    };
}

lexer! {
    (Redirect, redirect, '>'),
    (Assign, assign, '='),
}

#[cfg(test)]
mod tests {
    use super::{string, assign, invalid, lex, redirect};

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
        assert_eq!(lex(">").unwrap(), vec![redirect()]);
        assert_eq!(lex(">>").unwrap(), vec![redirect(), redirect()]);
        assert_eq!(lex(">a").unwrap(), vec![redirect(), string("a")]);
        assert_eq!(lex(">1").unwrap(), vec![redirect(), string("1")]);
        assert_eq!(lex("  >0>  1").unwrap(), vec![redirect(), string("0"), redirect(), string("1")]);
        assert_eq!(lex("echo Test > a").unwrap(), vec![string("echo"), string("Test"), redirect(), string("a")]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#).unwrap(), vec![string("echo"), string(r#"{"x": 23,"y":3}"#), redirect(), string("/dev/null")])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#).unwrap(), vec![string("hello"), assign(), string("123")]);
        assert_eq!(lex(r#"hello  ="1""#).unwrap(), vec![string("hello"), assign(), string("1")]);
        assert_eq!(lex(r#"hello = "1""#).unwrap(), vec![string("hello"), assign(), string("1")]);
        assert_eq!(lex(r#"hello='"123"'"#).unwrap(), vec![string("hello"), assign(), string("\"123\"")]);
        assert_eq!(lex(r#"hello '="123"'"#).unwrap(), vec![string("hello"), string("=\"123\"")]);
        assert_eq!(lex(r#"hello'="123"'"#).unwrap(), vec![string("hello=\"123\"")]);
    }
}
