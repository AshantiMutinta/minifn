pub mod common;
pub mod compiler;
pub mod system;

use crate::{
    common::error::MiniFnError,
    compiler::{parser::parse_lines_of_code, token::tokenize_lines},
};
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();
    args_iter.next();
    match &*args_iter
        .next()
        .map(|s| s.to_uppercase())
        .unwrap_or_else(String::new)
    {
        "--VERSION" | "-V" => println!("Minifn Version 1"),
        "--BUILD" | "BUILD" | "-B" => {
            build(args_iter.next().expect("Missing file path")).expect("Build failed")
        }
        s => println!("Minifn command {:?} cannot be recognized", s),
    }
}

fn build(path: &str) -> Result<(), MiniFnError> {
    let file = File::open(path).map_err(|e| {
        let error: MiniFnError = e.into();
        error
    })?;
    let mut buffer: Vec<u8> = Vec::new();
    let code_text = system::code_loader::load::<File>(file, &mut buffer)?;
    let code_blocks = tokenize_lines(code_text);
    println!("Tokenized code {:?}", code_blocks);
    parse_lines_of_code(&code_blocks)?;
    Ok(())
}
