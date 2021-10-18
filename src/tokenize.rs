#[derive(Debug)]
pub enum Token {
    SingleQuote,
    DoubleQuote,
    Backslash,
    Slash,
    Newline,
    Space,
    Feed,
    Tab,
    CR,
    OpenSquare,
    CloseSquare,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Asterisk,
    Colon,
    At,
}

const RE_AT_END: &str = r##"[\t\n\f\r "#'()/;[\\\]{}]"##;
const RE_WORD_END: &str = r##"[\t\n\f\r !"#'():;@[\\\]{}]|\/(?=\*)"##;
const RE_BAD_BRACKET: &str = r#"#.[\n"'(/\\]"#;
const RE_HEX_ESCAPE: &str = r#"[\da-f]"#;

// let css = input.css.valueOf()
// let ignore = options.ignoreErrors
//
// let code, next, quote, content, escape
// let escaped, escapePos, prev, n, currentToken
//
// let length = css.length
// let pos = 0
// let buffer = []
// let returned = []
pub struct Tokenizer {
    css: String,
    ignore: bool,

    pos: i32,
}

impl Tokenizer {
    pub fn new(input: i32, options: ()) -> Self {
        Tokenizer {}
    }
}
