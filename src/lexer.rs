use crate::models::CompilationFailure;

enum Token {
    // Single-character tokens.
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(String),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Token {
    fn len(&self) -> usize {
        match self {
            Token::LEFTPAREN => 1,
            Token::RIGHTPAREN => 1,
            Token::LEFTBRACE => 1,
            Token::RIGHTBRACE => 1,
            Token::COMMA => 1,
            Token::DOT => 1,
            Token::MINUS => 1,
            Token::PLUS => 1,
            Token::SEMICOLON => 1,
            Token::SLASH => 1,
            Token::STAR => 1,
            Token::BANG => 1,
            Token::BANGEQUAL => 2,
            Token::EQUAL => 1,
            Token::EQUALEQUAL => 2,
            Token::GREATER => 1,
            Token::GREATEREQUAL => 2,
            Token::LESS => 1,
            Token::LESSEQUAL => 2,
            Token::IDENTIFIER(d) => d.len(),
            Token::STRING(d) => d.len() + 2, 
            Token::NUMBER(d) => d.len(),
            Token::AND => "and".len(),
            Token::CLASS => "class".len(),
            Token::ELSE => "else".len(),
            Token::FALSE => "false".len(),
            Token::FUN => "fun".len(),
            Token::FOR => "for".len(),
            Token::IF => "if".len(),
            Token::NIL => "nil".len(),
            Token::OR => "or".len(),
            Token::PRINT => "print".len(),
            Token::RETURN => "return".len(),
            Token::SUPER => "super".len(),
            Token::THIS => "this".len(),
            Token::TRUE => "true".len(),
            Token::VAR => "var".len(),
            Token::WHILE => "while".len(),
            Token::EOF => 0,
        }
    }
}

/////////////////////////// Utility Constructors

fn one_char_token(c: char) -> Option<Token> {
    Some(match c {
        '(' => Token::LEFTPAREN,
        ')' => Token::RIGHTPAREN,
        '{' => Token::LEFTBRACE,
        '}' => Token::RIGHTBRACE,
        ',' => Token::COMMA,
        '.' => Token::DOT,
        '-' => Token::MINUS,
        '+' => Token::PLUS,
        ';' => Token::SEMICOLON,
        '*' => Token::STAR,
        _ => return None,
    })
}

fn one_or_two_char_token(cur_char: char, next_char: char) -> Option<(Token, usize)> {
    match cur_char {
        '!' => {
            if next_char == '=' {
                return Some((Token::BANGEQUAL, 2));
            } else {
                return Some((Token::BANG, 1));
            }
        }
        '=' => {
            if next_char == '=' {
                return Some((Token::EQUALEQUAL, 2));
            } else {
                return Some((Token::EQUAL, 1));
            }
        }
        '>' => {
            if next_char == '=' {
                return Some((Token::GREATEREQUAL, 2));
            } else {
                return Some((Token::GREATER, 1));
            }
        }
        '<' => {
            if next_char == '=' {
                return Some((Token::LESS, 2));
            } else {
                return Some((Token::LESSEQUAL, 1));
            }
        }
        _ => None,
    }
}

fn string<I>(mut next_chars: I) -> Option<Result<Token, String>>
where
    I: Iterator<Item = char>,
{
    if let Some('\"') = next_chars.next() {
        let mut ret_data = String::new();

        for c in next_chars {
            if c == '\"' {
                return Some(Ok(Token::STRING(ret_data)));
            }
            ret_data.push(c);
        }

        Some(Err("Unterminated string".to_string()))
    } else {
        return None;
    }
}

fn number<I>(mut next_chars: I) -> Option<Result<Token, String>>
where
    I: Iterator<Item = char>,
{
    let d = next_chars.next()?;
    if !d.is_digit(10) {
        return None;
    }

    let mut ret_data = String::new();
    ret_data.push(d);
    let mut isfloat = false;

    while let Some(c) = next_chars.next() {
        if !isfloat && c == '.' {
            if let Some(c) = next_chars.next() {
                if c.is_digit(10) {
                    ret_data.push(c);
                    isfloat = true;
                } else {
                    return Some(Err(format!(
                        "Unexpected character: {} after number decimal point.",
                        c
                    )
                    .to_string()));
                }
            } else {
                return Some(Err(
                    "Unexpected EOF while parsing number ending with .".to_string()
                ));
            }
        } else if c == ' ' {
            return Some(Ok(Token::NUMBER(ret_data)));
        } else if c.is_digit(10) {
            ret_data.push(c)
        } else {
            return Some(Err(
                format!("Unexpected character: {} after number", c).to_string()
            ));
        }
    }

    // Number at the end of a file
    return Some(Ok(Token::NUMBER(ret_data)));
}

fn keywords(right: &String) -> Option<Result<Token, String>> {
    let mut check_last = 0;
    let mut ret: Option<Result<Token, String>> = None;
    if right.starts_with("and") {
        ret = Some(Ok(Token::AND));
        check_last = "and".len();
    } else if right.starts_with("class") {
        ret = Some(Ok(Token::CLASS));
        check_last = "class".len();
    } else if right.starts_with("else") {
        ret = Some(Ok(Token::ELSE));
        check_last = "else".len();
    } else if right.starts_with("false") {
        ret = Some(Ok(Token::FALSE));
        check_last = "false".len();
    } else if right.starts_with("fun") {
        ret = Some(Ok(Token::FUN));
        check_last = "fun".len();
    } else if right.starts_with("for") {
        ret = Some(Ok(Token::FOR));
        check_last = "for".len();
    } else if right.starts_with("if") {
        ret = Some(Ok(Token::IF));
        check_last = "if".len();
    } else if right.starts_with("nil") {
        ret = Some(Ok(Token::NIL));
        check_last = "nil".len();
    } else if right.starts_with("or") {
        ret = Some(Ok(Token::OR));
        check_last = "or".len();
    } else if right.starts_with("print") {
        ret = Some(Ok(Token::PRINT));
        check_last = "print".len();
    } else if right.starts_with("return") {
        ret = Some(Ok(Token::RETURN));
        check_last = "return".len();
    } else if right.starts_with("super") {
        ret = Some(Ok(Token::SUPER));
        check_last = "super".len();
    } else if right.starts_with("this") {
        ret = Some(Ok(Token::THIS));
        check_last = "this".len();
    } else if right.starts_with("true") {
        ret = Some(Ok(Token::TRUE));
        check_last = "true".len();
    } else if right.starts_with("var") {
        ret = Some(Ok(Token::VAR));
        check_last = "var".len();
    } else if right.starts_with("while") {
        ret = Some(Ok(Token::WHILE));
        check_last = "while".len();
    }
    if ret.is_none() {
        return None;
    }
    let next_char = right.chars().nth(check_last);
    if next_char.is_none() {
        return ret;
    }
    let next_char = next_char.unwrap();
    if next_char != ' ' {
        return Some(Err(format!(
            "Unexpected token: {}",
            &right[0..check_last + 1]
        )));
    }
    return ret;
}

fn identifier<I>(mut next_chars: I) -> Option<Result<Token, String>>
where
    I: Iterator<Item = char>,
{
    let next_char = next_chars.next()?;
    if next_char.is_ascii() {
        let mut ret_data = String::new();
        ret_data.push(next_char);
        for c in next_chars {
            if c == ' ' {
                return Some(Ok(Token::IDENTIFIER(ret_data)));
            } else if c.is_digit(10) || c.is_ascii() {
                ret_data.push(c);
            } else {
                return Some(Err(format!(
                    "Invalid token: {} in the middle of identifier",
                    c
                )));
            }
        }
        return Some(Ok(Token::IDENTIFIER(ret_data)));
    } else {
        return None;
    }
}

fn parse_comment_to_next_line<I>(mut next_chars: I) -> usize
where
    I: Iterator<Item = char>,
{
     
}


struct Scanner {
    data: String, // The string to compile
    left: usize,  // An index to the start of the next token
    right: usize, // An index to the end of the next token
    line: usize,  // The current line number
}

impl Scanner {
    fn new(data: String) -> Self {
        Self {
            data,
            left: 0,
            right: 0,
            line: 0,
        }
    }

    fn scan_token(&mut self) -> Result<Token, CompilationFailure> {
        
    }
}
