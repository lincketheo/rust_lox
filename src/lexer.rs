use crate::models::CompilationFailure;


enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

struct Token {
    ttype: TokenType,
    lexeme: String,
}

struct Scanner {
    data: String,
    left: usize,
    right: usize,
    line: usize
}

impl Scanner {
    fn new(data: String) -> Self {
        Self {
            data, 
            left: 0,
            right: 0,
            line: 0
        }
    }

    fn scan_token(&mut self) -> Option<Result<Token, CompilationFailure>> {
        match self.data.chars().nth(self.left) {
            Some(c) => {
                let token = match c {
                    '(' => TokenType::LEFT_PAREN,
                    ')' => TokenType::LEFT_PAREN,
                    '{' => TokenType::LEFT_PAREN,
                    '}' => TokenType::LEFT_PAREN,
                    ',' => TokenType::LEFT_PAREN,
                    '.' => TokenType::LEFT_PAREN,
                    '-' => TokenType::LEFT_PAREN,
                    '+' => TokenType::LEFT_PAREN,
                    ';' => TokenType::LEFT_PAREN,
                    '*' => TokenType::LEFT_PAREN,
                    '!' => match self.data.chars().nth(self.left + 1)  {
                            Some(_c) => match _c {
                                    '=' => TokenType::BANG_EQUAL,
                                    _ => TokenType::BANG
                            },
                            None => {
                                TokenType::BANG
                            }
                    },
                    '=' => match self.data.chars().nth(self.left + 1)  {
                            Some(_c) => match _c {
                                    '=' => TokenType::BANG_EQUAL,
                                    _ => TokenType::BANG
                            },
                            None => {
                                TokenType::BANG
                            }
                    },
                    '>' => match self.data.chars().nth(self.left + 1)  {
                            Some(_c) => match _c {
                                    '=' => TokenType::BANG_EQUAL,
                                    _ => TokenType::BANG
                            },
                            None => {
                                TokenType::BANG
                            }
                    },
                    '<' => match self.data.chars().nth(self.left + 1)  {
                            Some(_c) => match _c {
                                    '=' => TokenType::BANG_EQUAL,
                                    _ => TokenType::BANG
                            },
                            None => {
                                TokenType::BANG
                            }
                    },
                    '\r' | '\t' | '\n' => return None,
                    _ => return None,
                    
                };
                Some(Ok(Token{ ttype: token, lexeme: "".to_string() }))
            },
            None => {
                Some(Ok(Token{ ttype: TokenType::EOF, lexeme: "".to_string()}))
            }
        }
    }
}

impl Iterator for Scanner {
    type Item = Result<Token, CompilationFailure>;

    fn next(&mut self) -> Option<Result<Token, CompilationFailure>> {
        let value = self.scan_token();
        while match value.is_none() {
            value = self.scan_token();
        }
        Some();
        while (let _value = self.scan_token()).is_ {}
        None
    }
}

fn scan_tokens(data: &String) -> (Vec<Token>, Vec<CompilationFailure>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut failures: Vec<CompilationFailure> = Vec::new();

    let ctx = ScannerCtx {
        start: 0,
        current: 0,
        line: 0
    };

    // TODO - change ownership of ctx
    while ctx.current < data.len() {
        match scan_next_token(data, &ctx) {
            Ok(token) => {
                tokens.push(token);
            },
            Err(failure) => {
                failures.push(failure);
            }
        }
    }

    (tokens, failures)
}

enum ScanTokenResult {
    Success(Token),
    Continue,
    Failure(CompilationFailure)
}

fn ctx_inc(ctx: ScannerCtx, len: usize) -> ScannerCtx {
    ScannerCtx {
        start: ctx.start + len,
        current: ctx.start + len,
        line: ctx.line
    }
}

fn ctx_nl(ctx: ScannerCtx) -> ScannerCtx {
    ScannerCtx {
        start: ctx.start + 1, 
        current: ctx.start + 1,
        line: ctx.line + 1,
    }
}

fn cpl_flr(data: &String, ctx: &ScannerCtx) -> CompilationFailure {
    CompilationFailure {
        line_no: ctx.line,
        msg: "TODO Error".to_string(),
        context: "TODO".to_string(),
    }
}

fn scan_next_token(data: &String, ctx: ScannerCtx) -> (ScanTokenResult, ScannerCtx) {
    match data.chars().nth(ctx.current) {
        None => {
            let token = Token {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
            };
            let res = ScanTokenResult::Success(token);
            return (res, ctx);
        },
        Some(c) => {
            match c {
                '(' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                ')' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '{' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '}' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                ',' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: '('.to_string() }), ctx_inc(ctx, 1)),
                '.' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '-' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '+' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                ';' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '*' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '!' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '=' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '<' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '>' => (ScanTokenResult::Success(Token{ ttype: TokenType::LEFT_PAREN, lexeme: "(".to_string() }), ctx_inc(ctx, 1)),
                '\t' | '\r' | ' ' => (ScanTokenResult::Continue, ctx_inc(ctx, 1)),
                '\n' => (ScanTokenResult::Continue, ctx_nl(ctx)),
                _ => (ScanTokenResult::Failure(cpl_flr(data, &ctx)), ctx_inc(ctx, 1))
            }
        }

    }
}

