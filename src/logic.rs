use std::collections::HashMap;

const LINES_COUNT: usize = 176;
const WORDS_IN_LINE: usize = 3;
const FILE_NAME: &str = "words.txt";

pub type Forms = [String; WORDS_IN_LINE];

#[derive(Clone)]
pub struct IrregularVerbs {
    verbs: HashMap<String, Forms>,
}

impl IrregularVerbs {
    /// Create a hashmap from txt file
    pub fn new() -> Self {
        // Read the file
        let text = std::fs::read_to_string(FILE_NAME).expect("Should have read the file");
        // Allocate space for hashmap
        let mut verbs = HashMap::with_capacity(LINES_COUNT);

        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            // Iterator of words in line
            let mut words = line.split_whitespace().map(String::from);

            // put 3 words from line into array
            let forms: Forms = std::array::from_fn(|_| {
                words.next().unwrap_or_else(|| panic!("Bad line: {line:?}"))
            });

            verbs.insert(forms[0].clone(), forms);
        }

        Self { verbs }
    }

    pub fn get(&self, base: &str) -> Option<&Forms> {
        self.verbs.get(base)
    }
}
