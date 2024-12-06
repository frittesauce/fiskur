use std::{ ops::Range, usize};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub location: Location
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    EndOfFile,
    SemiColon,
    Slash,

    Function, 
    Let,
    Identifier(String),
    Return,
    Break,

    StartScope,
    CloseScope,
    OpenParamn,
    CloseParamn,
    OpenBracket,
    CloseBracket,
    
    String(String),
    Int(String),
    Float(String),
    Boolean(bool),

    Equals,
    Plus,
    Minus,
    Star,
    Greater,
    Lesser,
    Colon,
    
    Bang,
    Comma,
    Period,
    
    DollarSign,
    Sigma,

    If,
    Else,
    ElseIf,

}

#[derive(Debug, PartialEq)]
pub struct Location {
    pub col: Range<usize>,
    pub line: Range<usize>,
}

