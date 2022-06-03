use std::collections::HashMap;

#[derive(Debug)]
pub struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    pub fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    pub fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);

        *count += 1;
    }

    pub fn display(&self, min_word_length: &u64) {
        let mut sorted_values: Vec<(&String, &u64)> = self.0.iter().collect();
        sorted_values.sort_by(|a, b| b.1.cmp(a.1));

        for (k, v) in sorted_values.iter() {
            if v >= &min_word_length {
                println!("{}: {}", k, v);
            }
        }
    }
}
