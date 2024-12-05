use crate::build::lexer;

pub fn parser(tokens: Vec<lexer::token::Token>){
    println!("{:#?}", tokens);
}
