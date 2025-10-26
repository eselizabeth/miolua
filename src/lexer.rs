


use std::{cmp::min, str::{self, Bytes}};
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
                token = self.get_number()
            }
            c if (c as char).is_ascii_alphanumeric() => {
                token = self.handle_alphanumeric()
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

    fn get_number(&mut self) -> Token {
        let mut buffer: Vec<u8> = vec!(self.source[self.char_index]);
        
        let mut next_char = self.get_next_char();
        let mut is_float = false;
        while next_char.is_ascii_digit() || next_char == b'.' {
            if next_char == b'.'{ is_float = true}
            buffer.push(next_char);
            self.char_index += 1;
            next_char = self.get_next_char();
        }
        let num = str::from_utf8(&buffer).ok().unwrap();
        if is_float{
            return Token::Float(str::parse(num).expect("da hell"));
        }
        else {
            return Token::Int(str::parse(num).expect("da hell"));
        }
    }

    fn handle_alphanumeric(&mut self) -> Token {
        let subslice = &self.source[self.char_index..];
        let space_pos = subslice.iter().position(|&b| b == b' ' || b == b'\n');
        let eq_pos = subslice.iter().position(|&b| Token::is_keyword(&b));
        //println!("im important {}", space_pos > eq_pos);
        //if let Some(second_pos) = min(subslice.iter().position(|&b| b == b' ').o, subslice.iter().position(|&b| b == b'=')) {
        if let Some(second_pos) = [space_pos, eq_pos]
            .into_iter()
            .flatten()
            .min()
        {
            self.char_index += second_pos;
            if space_pos > eq_pos{ self.char_index -=1 }
            return Lexer::get_alphan_token(&subslice[..second_pos]);
            //let text = &subslice[..second_pos];
            // let mut token = Token::Identifier(String::from(str::from_utf8(text).unwrap()));
            // Lexer::handle_keyword(&mut token);
        }
        else{
            Token::NONE
        }
    }

    fn get_alphan_token(subslice: &[u8]) -> Token{
        let text = str::from_utf8(subslice).unwrap();
        match text{
            "AND" | "and" => Token::AND,
            "OR" | "or" => Token::OR,
            "BREAK" | "break" => Token::BREAK,
            "DO" | "do" => Token::DO,
            "ELSE" | "else" => Token::ELSE,
            "ELSEIF" | "elseif" => Token::ELSEIF,
            "END" | "end" => Token::END,
            "FALSE" | "false" => Token::FALSE,
            "FOR" | "for" => Token::FOR,
            "FUNCTION" | "function" => Token::FUNCTION,
            "GOTO" | "goto" => Token::GOTO,
            "IF" | "if" => Token::IF,
            "IN" | "in" => Token::IN,
            "LOCAL" | "local" => Token::LOCAL,
            "NIL" | "nil" => Token::NIL,
            "NOT" | "not" => Token::NOT,
            "REPEAT" | "repeat" => Token::REPEAT,
            "RETURN" | "return" => Token::RETURN,
            "THEN" | "then" => Token::THEN,
            "TRUE" | "true" => Token::TRUE,
            "UNTIL" | "until" => Token::UNTIL,
            "WHILE" | "while" => Token::WHILE,
            _ => Token::Identifier(String::from(text))
        }
    }

    fn peek(&mut self) -> Option<u8>{
        if self.char_index == self.source.len(){
            return None;
        }
        // IGNORE WHITESPACE
        let mut char = self.source[self.char_index];
        while char == b' ' || char== b'\t'{
            if self.char_index + 1 == self.source.len(){
                return None;
            }        
            self.char_index += 1;
            char = self.source[self.char_index];
        }
        // IGNORE COMMENT
        //println!("xyz {:?}", self.get_next_char() as char);
        if char == b'-' && self.get_next_char() == b'-'{
            loop {
                if char == 10 || self.is_last_char(self.char_index + 1) { break; }
                self.char_index += 1;
                char = self.source[self.char_index];
            }
        }
        Some(char)
    }

    fn is_last_char(&mut self, index: usize) -> bool{
        index == self.source.len()
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

    pub fn get_all_tokens(&mut self) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();
        for token in self.into_iter(){
            if token != Token::EOS{
                println!("{:?}", token);
                tokens.push(token);
            }
        }
        return tokens;
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    fn confirm_tokens(source: &str, expected_tokens: Vec<Token>){
        let mut lex = Lexer::new(source.as_bytes());
        let actual_tokens = lex.get_all_tokens();
        for (pos, expected_token) in expected_tokens.iter().enumerate(){
            let actual_token = actual_tokens.get(pos).unwrap();
            assert_eq!(actual_token, expected_token);
        }
    }


    #[test]
    fn hello_world() {
        let source = "
        print \"hello world\";
        print \"hello earth\";
        ";
        let tokens: Vec<Token> = vec![
            Token::Identifier("print".to_owned()), Token::String("hello world".to_owned()), Token::SEMICOLON,
            Token::Identifier("print".to_owned()), Token::String("hello earth".to_owned()), Token::SEMICOLON
            ];
        confirm_tokens(source, tokens);
    }

    #[test]
    fn comment() {
        let source = "local i; -- im'a comment";
        let tokens: Vec<Token> = vec![Token::LOCAL, Token::Identifier("i".to_owned()), Token::SEMICOLON];
        confirm_tokens(source, tokens);
    }

    #[test]
    fn variables() {
        let source = "
            y=7;
            x = 1.223;
            local i = 99;
        ";
        let tokens: Vec<Token> = vec![
            Token::Identifier("y".to_owned()), Token::ASSIGN, Token::Int(7), Token::SEMICOLON,
            Token::Identifier("x".to_owned()), Token::ASSIGN, Token::Float(1.223), Token::SEMICOLON,
            Token::LOCAL, Token::Identifier("i".to_owned()), Token::ASSIGN, Token::Int(99), Token::SEMICOLON,
            ];
        confirm_tokens(source, tokens);

    }

    #[test]
    fn operators() {
        let source = "
            +-*::...
        ";
        let tokens: Vec<Token> = vec![
            Token::PLUS_SIGN, Token::MINUS_SIGN, Token::STAR_SIGN, Token::DOUBLE_COLON, Token::THREE_DOT
            ];
        confirm_tokens(source, tokens);

    }

    #[test]
    fn keyword(){
        let source = "
        
        function sum(n1,n2) -- i'm a comment
            sum=n1+n2
        return sum
        end
        ";
        let tokens: Vec<Token> = vec![
            Token::FUNCTION, Token::Identifier("sum".to_owned()), Token::L_PAREN, Token::Identifier("n1".to_owned()), Token::COMMA, Token::Identifier("n2".to_owned()), Token::R_PAREN,
            Token::Identifier("sum".to_owned()), Token::ASSIGN, Token::Identifier("n1".to_owned()), Token::PLUS_SIGN, Token::Identifier("n2".to_owned()),
            Token::RETURN, Token::Identifier("sum".to_owned()),
            Token::END
            ];
        confirm_tokens(source, tokens);
    }

}