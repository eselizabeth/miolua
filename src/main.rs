
mod token;
mod lua;
mod lexer;
mod bytecode;
mod translator;


fn main() {
    lua::Luac::start();
}
