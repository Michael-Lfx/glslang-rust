#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// An identifier, like a function or variable name.
    Identifier(String),
    /// An integer.
    Integer(u64),
    /// A boolean.
    Boolean(bool),
    /// A floating point,
    Float(f64),

    /// A white space.
    Whitespace(String),


    /// # Operators
    ///
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LowerThan,
    GreaterThan,
    Equal,
    NotEqual,
    /// `<`
    LeftAngle,
    /// `>`
    RightAngle,
    /// `|`
    VerticalBar,
    /// `^`
    Caret,
    /// `&`
    Ampersand,
    /// `?`
    Question,
    /// `%`
    Percent,

    /// # Delimiters
    ///
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    /// # Keywords
    ///
    /// `attribute` keyword.
    Attribute,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

//impl Default for Token {
//    fn default() -> Token {
//        Token::Illegal
//    }
//}

//pub fn lookup_ident(ident: &str) -> Token {
//    match ident {
//        "fn" => Token::Function,
//        "let" => Token::Let,
//        "true" => Token::True,
//        "false" => Token::False,
//        "if" => Token::If,
//        "else" => Token::Else,
//        "return" => Token::Return,
//        _ => Token::Ident(ident.to_string()),
//    }
//}

//#[test]
//mod test {
//    use super::*;
//
//    #[test]
//    fn lookup_ident_test() {
//        assert_eq!(lookup_ident("fn"), Token::Function);
//    }
//}
