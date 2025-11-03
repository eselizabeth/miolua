
mod token;
mod lua;
mod lexer;
mod bytecode;
mod compiler;
mod vm;

fn main() {
    lua::Luac::start();
}
