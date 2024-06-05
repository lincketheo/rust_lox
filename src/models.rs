
pub struct CompilationFailure {
    pub line_no: usize,
    pub msg: String,
    pub context: String
}

impl CompilationFailure {
    pub fn print_error(&self) {
        eprintln!("[line: {}] Error {}: {}", self.line_no, self.context, self.msg);
    }
}

