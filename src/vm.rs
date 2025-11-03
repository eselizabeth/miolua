
use crate::token::Token;
use crate::compiler::{Data, Compiler};
use crate::bytecode::{ByteCode};


pub struct VM{
    constant_table: Vec<Data>,
    global_table: Vec<Data>,
    bytecodes: Vec<ByteCode>
}


impl VM {
    pub fn new(compiler: Compiler) -> VM{
        VM{
            global_table: Vec::new(),
            constant_table: compiler.constants,
            bytecodes: compiler.bytecodes
        }
    }

    pub fn execute(&mut self){
        todo!()
    }

}


