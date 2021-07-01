use std::path::PathBuf;

pub struct Scanner {
    grammar_file: PathBuf,
}

impl Scanner {
    pub fn new(grammar_file: PathBuf) -> Self {
        Scanner { grammar_file }
    }
}
