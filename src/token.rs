




// https://www.lua.org/manual/5.4/manual.html#3.1
//Lua is a case-sensitive language:
//A comment starts with a double hyphen (--) anywhere outside a string.
#[allow(non_camel_case_types)]
pub enum Token{
    // Keywords
    AND, BREAK, DO, ELSE, ELSEIF, END, 
    FALSE, FOR, FUNCTION, GOTO, IF, IN,
    LOCAL, NIL, NOT, OR, REPEAT, RETURN,
    THEN, TRUE, UNTIL, WHILE,

    // Other tokens
    PLUS_SIGN, MINUS_SIGN, STAR_SIGN, SLASH_SIGN, MOD_SIGN, CARET_SIGN, HASH_SIGN,
    AMP_SIGN, TILDE_SIGN, PIPE_SIGN, L_SHIFT, R_SIGHT, INT_DIV,
    EQUAL, NOT_EQUAL, LESS_EQUAL, GREATER_EQUAL, LESS, GREATER, ASSIGN,
    L_PAREN, R_PAREN, L_BRACE, R_BRACE, L_BRACKET, R_BRACKET, DOUBLE_COLON,
    SEMICOLON, COLON, COMMA, DOT, TWO_DOT, THREE_DOT
}