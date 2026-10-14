use std::path::Path;

const FILE_NAME: &str = "words.txt";

struct IrregVerbs {
    ir_words: String,
}

impl IrregVerbs {
    pub fn new() -> Self {
        Self {
            ir_words: std::fs::read_to_string(FILE_NAME).expect("Should have read file"),
        }
    }
}
