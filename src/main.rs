mod compiler;
mod language;

use crate::compiler::compiler as Compiler;

fn main() {
    Compiler::compile("Hello \"world\"\n\"hello\"\n1.51345 hello there-")
}