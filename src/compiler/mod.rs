pub mod constants;
mod tokenizer;
mod lexer;

pub mod compiler {
    use crate::{compiler::{tokenizer::tokenizer::tokenize, lexer::lexer}, language::language::load_lang};

    pub fn compile(src: &str) {
        load_lang();
        lexer::parse(tokenize(src));
    }
}