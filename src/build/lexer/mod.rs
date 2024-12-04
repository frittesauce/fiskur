use std::{sync::mpsc, thread::spawn};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use token::{Token, TokenType};

mod reader;
pub mod token;
mod tokenizer;

pub fn lexer(file: String, mp: MultiProgress, mainpg: &ProgressBar) -> Vec<token::Token> {
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]",
    )
    .unwrap()
    .progress_chars("=> ");

    let steps = file.len();

    let mut reader = reader::Reader::new(file);

    let pb1 = mp.insert_after(mainpg, ProgressBar::new(steps as u64));
    pb1.set_style(sty);

    let (tx, rx) = mpsc::channel::<u64>();

    let handle = spawn({
        move || loop {
            match rx.recv() {
                Ok(progress) => {
                    pb1.set_position(progress);
                    pb1.tick();
                }
                Err(_) => {
                    pb1.finish_and_clear();
                    break;
                }
            }
        }
    });

    let mut tokens: Vec<Token> = Vec::new();
    let mut position;

    loop {
        reader.skip_whitespace();

        let token = tokenizer::match_token(&mut reader);

        if token.token == TokenType::EndOfFile {
            tokens.push(token);
            break;
        }

        tokens.push(token);

        position = reader.pos;

        tx.send(position as u64).unwrap();
    }

    drop(tx);

    handle.join().unwrap();

    tokens
}
