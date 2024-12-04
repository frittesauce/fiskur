mod lexer;
mod parser;

use crate::config;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{ fs, io};

pub fn build() -> io::Result<()> {
    
    let steps = 200;

    let mp = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]",
    )
    .unwrap()
    .progress_chars("=> ");


    let pb = mp.add(ProgressBar::new(steps));
    pb.set_style(
        sty.clone()
    );

    let data = config::get_toml();

    mp.println(format!(
        "Building project {}:{}, with compiler version {}",
        data.config.name, data.config.version, data.config.edition
    )).unwrap();

    pb.inc(1);

    let main_folder = data.config.src;
    let main = data.config.main;
    let path = main_folder + &main + ".kty";

    let main_file = fs::read_to_string(path)?;

    pb.println(format!("Fetched main file sucesefully!"));
    pb.inc(1);

    let tokens = lexer::lexer(main_file, mp, &pb);

    pb.println("sucesefully tokenized code!");
    pb.inc(1);

    let abstract_syntax_tree = parser::parser(tokens);

    pb.finish_and_clear();
    println!("completed compiling!");
    Ok(())
}
