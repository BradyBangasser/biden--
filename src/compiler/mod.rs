pub mod constants;
mod tokenizer;

pub mod compiler {
    use crate::{compiler::{tokenizer::tokenizer::tokenize, constants::constants::Constants}, language::language::load_lang};

    pub fn compile(src: &str) {
        load_lang();
        let hello = Constants::FALSE_T;
        println!("Here, {}", hello);
        tokenize(src)
    }
}