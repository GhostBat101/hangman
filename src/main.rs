extern crate rand;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

struct Word {
    answer: String,
    length: usize,
    correct_count: usize,
    representation: String
}

trait CheckLetter {
    fn check_for_letter(&mut self, c: char) -> bool;
}

trait CheckComplete {
    fn check_complete(&self) -> bool;
}

impl CheckComplete for Word {
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }
}

impl CheckLetter for Word {
    fn check_for_letter(&mut self, c: char) -> bool {
        let mut count: usize = 0;
        let mut found: bool = false;
        let mut response = String::with_capacity(self.length);
        let mut index = 0;
        for letter in self.answer.chars() {
            if letter == c {
                found = true;
                count += 1;
                response.push(c);
            } else {
                if self.representation.chars().nth(index) != Some('_') {
                    response.push(self.representation.chars().nth(index).unwrap());
                }
                else {
                    response.push('_');
                }
            }
            index += 1;
        }
        if found {
            println!("Found a ")
        }
        self.representation = response;
        self.correct_count += count;
        count> 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: String) -> Vec::<String> {
    let mut v = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        for w in lines {
            let word:String = w.unwrap();
            if word.len()> 4 {
                v.push(word);
            } else { println!("We have an error while reading the list.") }
        }
    }
    v
}

fn main() {
    println!("Hello, world!");
}
