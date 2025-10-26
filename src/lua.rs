use std::{env, fs};

use crate::translator::Translator;
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
            let lexer = Lexer::new(source.as_bytes());
            //let tokens = lexer.get_all_tokens();
            let mut translator = Translator::new(lexer);
            translator.parse();
            translator.print_bytecode();
        }
        else{
            panic!("Usage: miolua [script]");
        }
    }

    fn get_file_content(file_path: String) -> String{
        let content: String = match fs::read_to_string(file_path.clone()){
            Ok(val) => val,
            Err(e) => panic!("problem reading input file {e}")
        };
        content
    }

}