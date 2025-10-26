

use crate::lexer::Lexer;
use crate::token::Token;
use crate::bytecode::{ByteCode};


pub enum Data{
    Identifier(String),
    String(String),
}

// Generates bytecode from tokens
// Bytecode is later executed by 
pub struct Translator<'a>{
    lexer: Lexer<'a>,
    data_table_index: u8,
    data_table: Vec<Data>,
    bytecodes: Vec<ByteCode>
}


impl<'a> Translator<'a>{
    pub fn new(lexer: Lexer<'a>) -> Translator<'a>{
        Translator {
            lexer,
            data_table_index: 0,
            data_table: Vec::new(),
            bytecodes: Vec::new()
        }
    }

    pub fn parse(&mut self){
        while let Some(token) = self.lexer.next(){
            match token{
                Token::LOCAL => {
                    self.load_value();
                }

                // Token::Identifier(val) => {
                //     self.data_table.push(Data::String(val));
                //     self.bytecodes.push(ByteCode::LOAD_VAL(0, 0));
                // }
                // Token::String(val) => {
                //     self.data_table.push(Data::String(val));
                //     self.bytecodes.push(ByteCode::LOAD_VAL(0, 1));

                // }
                tok => println!("shouldnt happen {:?}", tok)
            }
        }
    }

    fn current_index(&mut self) -> u8{
        self.data_table_index += 1;
        self.data_table_index

    }

    pub fn print_bytecode(&mut self){
        for (index, bytecode) in self.bytecodes.iter().enumerate() {
            println!("{} {:?}", index, bytecode)
        }
    }

    fn load_value(&mut self){
        let variable_name = if let Some(Token::Identifier(variable_name)) = self.lexer.next() {
            variable_name
        }
        else{
            panic!("expected var name");
        };
        let operator = if let Some(operator) = self.lexer.next() {
            operator
        }
        else{
            panic!("expected '='");
        };
        if operator != Token::ASSIGN{
            panic!("expected '='");
        }
        let value = if let Some(Token::String(value)) = self.lexer.next() {
            value
        }
        else{
            panic!("expected a value");
        };
        self.bytecodes.push(ByteCode::LOAD_VAL(0, self.data_table.len() as u8));
        self.data_table.push(Data::Identifier(variable_name));
    }

}