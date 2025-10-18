use std::{env, fs};

use crate::token::Token;
use crate::{lexer::Lexer};

pub struct Luac{
}


impl Luac{
    pub fn start(){
        let args: Vec<String> = env::args().collect();
        if args.len() > 2{
            panic!("Usage: miolua [script]");
        }
        else if args.len() == 2{
            let source = Luac::get_file_content(args[1].clone());
            let mut lexer = Lexer::new(source.as_bytes());
            lexer.print_all_tokens();
            //Luac::execute_file(content);
        }
        else{
            panic!("Usage: miolua [script]");
        }
    }

    fn execute_file(content: String) -> Vec<Token>{
        todo!();

    }

    fn get_file_content(file_path: String) -> String{
        let content: String;
        match fs::read_to_string(file_path.clone()){
            Ok(val) => content = val,
            Err(e) => panic!("problem reading input file {e}")
        }
        content
    }

}