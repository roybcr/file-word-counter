mod word_counter;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use word_counter::WordCounter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let min_word_length = &args[2];

    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let counter = &mut WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(' ');
        for word in words {
            if word.is_empty() {
                continue;
            } else {
                counter.increment(word);
            }
        }
    }

    let min = (min_word_length as &str).parse::<u64>();
    match min {
        Result::Ok(x) => counter.display(&x),
        _ => println!("Could not filter by minimum word length"),
    }
}
