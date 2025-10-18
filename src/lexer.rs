



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

            char => todo!()
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

    fn peek(&mut self) -> Option<u8>{
        if self.char_index == self.source.len(){
            return None;
        }
        let char = self.source[self.char_index];
        Some(char)
    }

    fn get_next_char(&mut self) -> u8{
        if self.char_index + 1 == self.source.len(){
            return 0;
        }
        let char = self.source[self.char_index+1];
        char
    }

    fn get_two_next_char(&mut self) -> u8{
        if self.char_index + 2 == self.source.len(){
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