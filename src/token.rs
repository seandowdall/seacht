#[derive(Debug, Clone, Copy, PartialEQ)]
pub struct Token {
    // token type
    pub kind: Kind,

    //start offset in source
    pub start: usize,

    // End offset in source
    pub end: usize,
}

// This is the alphabet of JavaScript
// a single + gives us:
/*
{
    Token { kind: Kind::Plus, start: 0, end: 1},
    Token { kind: Kind::Eof,  start: 1, end: 1}
}
*/
#[derive(Debug, Clone, Copy, PartialEQ)]
pub enum Kind {
    //End of file
    Eof,
    // Keywords
    Let,
    Const,
    //Identifiers and Literals
    Identifier(String),
    Number(f64),
    //Operators
    Equals,
    Plus,
    //Punctuation
    Semicolon,
}
