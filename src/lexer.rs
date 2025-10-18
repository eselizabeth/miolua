


use std::cmp::min;
use crate::{token::Token};

pub struct Lexer<'a> {
    source: &'a [u8],
    line_index: usize,
    char_index: usize,
}


impl<'a>  Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item>{
        let current_char = self.peek();
        if current_char.is_none(){
            return None;
        }
        let current_char = current_char.unwrap();
        // println!("char_index is {}", self.char_index);
        // println!("current char is {}", current_char);
        let next_char = self.get_next_char();
        let two_next_char = self.get_two_next_char();
        let mut move_by = 1;
        let mut token: Token = Token::NONE;
        match current_char{
            b'/' => {
                if next_char == b'/'{
                    token = Token::INT_DIV;
                    move_by += 2;                
                }
                else{ token = Token::SLASH_SIGN }
            },
            b'+' => token = Token::PLUS_SIGN,
            b'-' => token = Token::MINUS_SIGN,
            b'*' => token = Token::STAR_SIGN,
            b'%' => token = Token::MOD_SIGN,
            b'^' => token = Token::CARET_SIGN,
            b'#' => token = Token::HASH_SIGN,
            b'&' => token = Token::AMP_SIGN,
            b'~' => {
                if next_char == b'='{
                    token = Token::NOT_EQUAL;
                    move_by += 2;                
                }
                else{ token = Token::TILDE_SIGN}
            },
            b'|' => token = Token::PIPE_SIGN,
            b'<' => {
                if next_char == b'<'{
                    token = Token::L_SHIFT;
                    move_by += 2;                
                }
                else if next_char == b'='{
                    token = Token::LESS_EQUAL;
                    move_by += 2;                
                }
                else{ token = Token::LESS}
            },
            b'>' => {
                if next_char == b'>'{
                    token = Token::R_SHIFT;
                    move_by = 2;                
                }
                else if next_char == b'='{
                    token = Token::GREATER_EQUAL;
                    move_by += 2;                
                }
                else { token = Token::GREATER}
            },
            b'=' => {
                if next_char == b'='{
                    token = Token::EQUAL;
                    move_by += 2;                
                }
                else { token = Token::ASSIGN }
            },
            b'(' => token = Token::L_PAREN,
            b')' => token = Token::R_PAREN,
            b'{' => token = Token::L_BRACE,
            b'}' => token = Token::R_BRACE,
            b'[' => token = Token::L_BRACKET,
            b']' => token = Token::R_BRACKET,

            b':' => {
                if next_char == b':'{
                    token = Token::DOUBLE_COLON;
                    move_by = 2;                
                }
                else { token = Token::COLON }
            }
            b';' => token = Token::SEMICOLON,
            b',' => token = Token::COMMA,
            b'.' =>
            if next_char == b'.' && two_next_char == b'.'{
                token = Token::THREE_DOT;
                move_by = 3;                
            }
            else if next_char == b'.'{
                token = Token::TWO_DOT;
                move_by = 2;              
            }
            else { token = Token::DOT }

            char => token = self.parse_text(char)
        };

        self.char_index += move_by;
        if token == Token::NONE{
            return None;
        }
        Some(token)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Lexer<'a>{
        Lexer{ 
            source: source,
            line_index: 0,
            char_index: 0,
        }
    }

    fn parse_text(&mut self, char: u8) -> Token{
        let mut token = Token::NONE;
        match char{
            b'"' => {
                token = Token::String(self.get_string())
            } 
            c if (c as char).is_ascii_digit() => {
                token = Token::Number(self.get_number())
            }
            c if (c as char).is_ascii_alphanumeric() => {
                token = Token::Identifier(self.get_identifier())
            }
            10 => token = Token::EOS,
            32 => token = Token::SPACE,
            _ => panic!("I shouldn't be here!")
        }
        return token;
    }

    fn get_string(&mut self) -> String {
        let subslice = &self.source[self.char_index + 1..];
        if let Some(second_pos) = subslice.iter().position(|&b| b == b'"') {
            self.char_index += 1 + second_pos;
            return String::from(str::from_utf8(&subslice[..second_pos]).unwrap());
        }
        else{
            String::from("DAHELL??")
        }
    }

    fn get_number(&mut self) -> i32 {
        let mut number = 0;
        let next_char = self.get_next_char();
        if next_char.is_ascii_digit(){
            number = number * 10 + next_char as i32;
        }
        return number;
    }

    fn get_identifier(&mut self) -> String {
        let subslice = &self.source[self.char_index..];
        let space_pos = subslice.iter().position(|&b| b == b' ');
        let eq_pos = subslice.iter().position(|&b| b == b'=');
        //if let Some(second_pos) = min(subslice.iter().position(|&b| b == b' ').o, subslice.iter().position(|&b| b == b'=')) {
            if let Some(second_pos) = [space_pos, eq_pos]
            .into_iter()
            .flatten()
            .min()
        {
            self.char_index += second_pos;
            return String::from(str::from_utf8(&subslice[..second_pos]).unwrap());
        }
        else{
            String::from("??")
        }
    }

    fn peek(&mut self) -> Option<u8>{
        if self.char_index == self.source.len(){
            return None;
        }
        let char = self.source[self.char_index];
        Some(char)
    }

    fn get_next_char(&mut self) -> u8{
        if self.char_index + 1 >= self.source.len(){
            return 0;
        }
        let char = self.source[self.char_index+1];
        char
    }

    fn get_two_next_char(&mut self) -> u8{
        if self.char_index + 2 >= self.source.len(){
            return 0;
        }
        let char = self.source[self.char_index+2];
        char
    }

    pub fn print_all_tokens(&mut self){
        for token in self.into_iter(){
            println!("{:?}", token);
        }
    }

}