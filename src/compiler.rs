

use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::token::Token;
use crate::bytecode::{ByteCode};


pub enum Data{
    Identifier(String),
    String(String),
}


// Generates bytecode from tokens
// Bytecode is later executed by VM
pub struct Compiler<'a>{
    lexer: Lexer<'a>,
    pub constants: Vec<Data>,
    data_table: HashMap<String, Data>, // not sure if it should be data or register index?
    pub bytecodes: Vec<ByteCode>
}


impl<'a> Compiler<'a>{
    pub fn new(lexer: Lexer<'a>) -> Compiler<'a>{
        Compiler {
            lexer,
            constants: Vec::new(),
            data_table: HashMap::new(),
            bytecodes: Vec::new()
        }
    }

    pub fn parse(&mut self) -> Vec<ByteCode>{
        while let Some(token) = self.lexer.next(){
            match token{
                Token::Identifier(data) => {
                    self.parse_statement(data);
                }


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
        // Didn't wana implement block analysis
        // So memory penalty instead of time/performance penalty
        // Not sure about parameters yet
        self.bytecodes.push(ByteCode::RETURN(0, 0));
        self.bytecodes.clone()
    }

    fn parse_statement(&mut self, data: String){
        if let Some(token) = self.lexer.next() {
            match token{
                Token::L_PAREN => self.handle_function(data),
                _ => panic!("cover me :(")
            }
        }
        else{
            panic!("calm down buddy");
        };
    }

    fn handle_function(&mut self, function_name: String) {
        let string = if let Some(Token::String(string)) = self.lexer.next() {
            string
        }
        else{
            panic!("expected string");

        };
        let r_paren = self.lexer.next();
        self.constants.push(Data::String(string));
        self.bytecodes.push(ByteCode::GGET(0, (self.constants.len() - 1) as u8));
        self.bytecodes.push(ByteCode::LOAD_VAL(1, (self.constants.len() - 1) as u8));
        // print to global table
        // string to constant table
        // call function 
        self.bytecodes.push(ByteCode::CALL(0, 0, 0));
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
        // let value = if let Some(Token::Int(value)) = self.lexer.next() {
        //     value
        // }
        // else{
        //     panic!("expected a value");
        // };
        // add value to register
        // generate insturction to move it to the constant table
        //let register = self.add_to_the_register(value);
        
        self.bytecodes.push(ByteCode::LOAD_VAL(0, self.data_table.len() as u8));
        //self.data_table.insert(Data::Identifier(variable_name), value,);
    }

    // fn add_to_the_register(&mut self, value: Data) -> u8{
    //     // what should be the limit of register???
    //     self.register.push(value);
    //     (self.register.len() - 1) as u8
    // }

}