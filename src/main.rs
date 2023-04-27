mod compiler;
mod language;

use crate::compiler::compiler as Compiler;

fn main() {
    Compiler::compile("var hello = 5.5")
}