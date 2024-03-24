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
            Arg(String),
            Invalid(String),
        }

        pub fn lex(line: &str) -> Vec<Token> {
            REGEX.captures_iter(line).map(|c| {
                if let Some(arg) = c.name("singlequoted").or(c.name("doublequoted")).or(c.name("unquoted")) {
                    Token::Arg(arg.as_str().to_owned())
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
        fn arg(s: &str) -> Token {
            Token::Arg(s.to_owned())
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
    use crate::line::lex::{arg, assign, invalid, lex, redirect};

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

    #[test]
    fn redirects() {
        assert_eq!(lex(">"), vec![redirect()]);
        assert_eq!(lex(">>"), vec![redirect(), redirect()]);
        assert_eq!(lex(">a"), vec![redirect(), arg("a")]);
        assert_eq!(lex(">1"), vec![redirect(), arg("1")]);
        assert_eq!(lex("  >0>  1"), vec![redirect(), arg("0"), redirect(), arg("1")]);
        assert_eq!(lex("echo Test > a"), vec![arg("echo"), arg("Test"), redirect(), arg("a")]);
        assert_eq!(lex(r#"echo '{"x": 23,"y":3}' > /dev/null"#), vec![arg("echo"), arg(r#"{"x": 23,"y":3}"#), redirect(), arg("/dev/null")])
    }

    #[test]
    fn assignments() {
        assert_eq!(lex(r#"hello="123""#), vec![arg("hello"), assign(), arg("123")]);
        assert_eq!(lex(r#"hello  ="1""#), vec![arg("hello"), assign(), arg("1")]);
        assert_eq!(lex(r#"hello = "1""#), vec![arg("hello"), assign(), arg("1")]);
        assert_eq!(lex(r#"hello='"123"'"#), vec![arg("hello"), assign(), arg("\"123\"")]);
        assert_eq!(lex(r#"hello'="123"'"#), vec![arg("hello"), arg("=\"123\"")]);
    }
}
