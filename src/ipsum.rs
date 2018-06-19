extern crate rand;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use self::rand::{thread_rng, Rng, sample};

fn read_file_by_lines<P>(filename: P) -> Vec<String> where P: AsRef<Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

pub fn get_quote() -> Vec<String> {
    let mut rng = thread_rng();
    let quotes = read_file_by_lines("quotes");
    let quote = sample(&mut rng, quotes, 1);
    println!("{:?}", quote);

    quote
}

pub fn get_paragraph() -> Vec<String> {
    let mut rng = thread_rng();
    let quotes = read_file_by_lines("quotes");
    let range = rng.gen_range(6, 11);
    let paragraph = sample(&mut rng, quotes, range);
    println!("{:?}", paragraph);

    paragraph
}

pub fn get_multiple_paragraph(count: u8) -> Vec<Vec<String>> {
    let mut paragraphs: Vec<Vec<String>> = Vec::new();

    for _ in 0..count {
        let paragraph = get_paragraph();
        paragraphs.push(paragraph);
    }

    println!("{:?}", paragraphs);
    paragraphs
}
