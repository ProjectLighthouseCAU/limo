use once_cell::sync::Lazy;
use regex::Regex;

macro_rules! lexer {
    ($(($op_name_upper:ident, $op_name_lower:ident, $op_regex:literal)),* $(,)?) => {
        const REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&[
            $(concat!(r"(?<op_", stringify!($op_name_lower), ">", $op_regex, ")"),)*
            r"'(?<singlequoted>[^']*)'",
            r#""(?<doublequoted>[^"]*)""#,
            r#"(?<strayquotes>['"]+)"#,
            concat!(r#"(?<unquoted>[^'""#, $($op_regex,)* r#"\s]+)"#),
        ].join("|")).unwrap());

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Token {
            $($op_name_upper,)*
            String(String),
            Invalid(String),
        }

        pub fn lex(line: &str) -> Vec<Token> {
            REGEX.captures_iter(line).map(|c| {
                if let Some(string) = c.name("singlequoted").or(c.name("doublequoted")).or(c.name("unquoted")) {
                    Token::String(string.as_str().to_owned())
                } $(else if c.name(concat!("op_", stringify!($op_name_lower))).is_some() {
                    Token::$op_name_upper
                })* else {
                    Token::Invalid(c[0].to_owned())
                }
            }).collect()
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
            Token::String(s.to_owned())
        }

        #[cfg(test)]
        fn invalid(s: &str) -> Token {
            Token::Invalid(s.to_owned())
        }
    };
}

lexer! {
    (Redirect, redirect, ">"),
    (Assign, assign, "="),
}

#[cfg(test)]
mod tests {
    use super::{string, assign, invalid, lex, redirect};

    #[test]
    fn whitespace() {
        assert_eq!(lex(""), vec![]);
        assert_eq!(lex("  "), vec![]);
    }

    #[test]
    fn simple_commands() {
        assert_eq!(lex("ls"), vec![string("ls")]);
        assert_eq!(lex("ls /"), vec![string("ls"), string("/")]);
        assert_eq!(lex("ls hello"), vec![string("ls"), string("hello")]);
        assert_eq!(lex("ls /hello/ world"), vec![string("ls"), string("/hello/"), string("world")]);
        assert_eq!(lex(" ls / "), vec![string("ls"), string("/")]);
        assert_eq!(lex(" pwd"), vec![string("pwd")]);
        assert_eq!(lex("echo Hello world123 !"), vec![string("echo"), string("Hello"), string("world123"), string("!")]);
    }

    #[test]
    fn quotes() {
        assert_eq!(lex("''"), vec![string("")]);
        assert_eq!(lex(r#""""#), vec![string("")]);
        assert_eq!(lex(r#" """"  "" "#), vec![string(""), string(""), string("")]);
        assert_eq!(lex(r#" "''"  "" "#), vec![string("''"), string("")]);
        assert_eq!(lex(r#" "''  "" "#), vec![string("''  "), invalid("\"")]);
        assert_eq!(lex(r#"echo "Hello world "  1234"#), vec![string("echo"), string("Hello world "), string("1234")]);
        assert_eq!(lex(r#"echo "Hello world   1234"#), vec![string("echo"), invalid("\""), string("Hello"), string("world"), string("1234")]);
        // TODO: Should we parse adjacent stuff as one string?
        assert_eq!(lex(r#"echo '"Hello 'world   1234"#), vec![string("echo"), string("\"Hello "), string("world"), string("1234")]);
    }

    #[test]
    fn redirects() {
        assert_eq!(lex(">"), vec![redirect()]);
        assert_eq!(lex(">>"), vec![redirect(), redirect()]);
        assert_eq!(lex(">a"), vec![redirect(), string("a")]);
        assert_eq!(lex(">1"), vec![redirect(), string("1")]);
        assert_eq!(lex("  >0>  1"), vec![redirect(), string("0"), redirect(), string("1")]);
        assert_eq!(lex("echo Test > a"), vec![string("echo"), string("Test"), redirect(), string("a")]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#), vec![string("echo"), string(r#"{"x": 23,"y":3}"#), redirect(), string("/dev/null")])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#), vec![string("hello"), assign(), string("123")]);
        assert_eq!(lex(r#"hello  ="1""#), vec![string("hello"), assign(), string("1")]);
        assert_eq!(lex(r#"hello = "1""#), vec![string("hello"), assign(), string("1")]);
        assert_eq!(lex(r#"hello='"123"'"#), vec![string("hello"), assign(), string("\"123\"")]);
        assert_eq!(lex(r#"hello'="123"'"#), vec![string("hello"), string("=\"123\"")]);
    }
}
