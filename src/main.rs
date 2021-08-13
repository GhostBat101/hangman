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


fn main() {
    println!("Hello, world!");
}
