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
    let min_count = &args[2];

    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let counter = &mut WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                counter.increment(word);
            }
        }
    }

    let min = u64::from_str_radix(min_count as &str, 10);
    match min {
        Result::Ok(x) => counter.display(&x),
        _ => println!("Couldn't filter by min count!"),
    }
}
