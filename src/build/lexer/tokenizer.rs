use core::panic;

use super::{
    reader::{Char, Reader},
    token::{Token, TokenType},
};
use crate::build::lexer::token::Location;

pub fn match_token(reader: &mut Reader) -> Token {
    let char = match reader.next() {
        Some(tk) => tk,

        None => {
            return Token {
                token: TokenType::EndOfFile,
                location: Location {
                    col: 0usize..0usize,
                    line: reader.lines.len()..reader.lines.len(),
                },
            }
        }
    };

    let mut last_chr = char;

    let tk_type: TokenType = match char.char {
        '!' => TokenType::Bang,
        '*' => TokenType::Star,
        '=' => TokenType::Equals,
        '-' => TokenType::Minus,
        '+' => TokenType::Plus,
        '/' => TokenType::Slash,
        '$' => TokenType::DollarSign,
        ',' => TokenType::Comma,
        '.' => TokenType::Period,
        '<' => TokenType::Lesser,
        '>' => TokenType::Greater,
        ';' => TokenType::SemiColon,
        ':' => TokenType::Colon,
        'Σ' => TokenType::Sigma,
        '{' => TokenType::StartScope,
        '}' => TokenType::CloseScope,
        '[' => TokenType::OpenBracket,
        ']' => TokenType::CloseBracket,
        '(' => TokenType::OpenParamn,
        ')' => TokenType::CloseParamn,
        _ => {
            let (tk_type, last_ch) = non_single(reader, char);
            last_chr = last_ch;
            tk_type
        }
    };

    return Token {
        token: tk_type,
        location: Location {
            col: char.column..last_chr.column + 1,
            line: char.line..last_chr.line,
        },
    };
}

fn non_single(reader: &mut Reader, begin_ch: Char) -> (TokenType, Char) {
    let mut chars: Vec<char> = vec![];
    let mut last_ch: Char = begin_ch;

    chars.push(begin_ch.char);

    if Reader::is_letter(begin_ch) {
        loop {
            let chr: Char = match reader.peek() {
                Some(tk) => *tk,
                None => break,
            };
            if !Reader::is_letter(chr) && !Reader::is_number(chr) {
                break;
            }
            reader.next();
            last_ch = chr;
            chars.push(chr.char);
        }

        let string = chars.into_iter().collect::<String>();

        let token_type: TokenType = match string.as_str() {
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "maybe" => TokenType::ElseIf,
            "elseif" => TokenType::ElseIf,
            "let" => TokenType::Let,
            "return" => TokenType::Return,
            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),
            _ => TokenType::Identifier(string),
        };

        return (token_type, last_ch);
    } else if Reader::is_number(begin_ch) {
        let mut float: bool = false;

        loop {
            let chr: Char = match reader.peek() {
                Some(tk) => *tk,
                None => break,
            };

            if !Reader::is_number(chr) && chr.char != '.' {
                break;
            }

            if chr.char == '.' {
                float = true;
            }

            reader.next();
            last_ch = chr;
            chars.push(chr.char);
        }

        match float {
            true => {
                return (TokenType::Float(chars.into_iter().collect()), last_ch);
            }
            false => {
                return (TokenType::Int(chars.into_iter().collect()), last_ch);
            }
        }
    } else if begin_ch.char == '\"' {
        chars = vec![];
        loop {
            let chr: Char = match reader.peek() {
                Some(tk) => *tk,
                None => {
                    panic!("string doesnt close! at {:#?} {:#?}", begin_ch.line, begin_ch.column);
                },
            };

            if chr.char != '\"' {
                reader.next();
                chars.push(chr.char);
            } else {
                reader.next();
                last_ch = chr;
                break
            }
        }
        
        return (TokenType::String(chars.into_iter().collect()), last_ch);
    }
    else {
        panic!(
            "unicode characters are NOT supported inside my programing languague! {:#?}",
            begin_ch
        );
    }
}
