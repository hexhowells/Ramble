#![allow(dead_code)]
use ramble::trie::Trie;

use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};


fn main() {
    println!("Loading dictionary into Trie");
    let trie = load_dictionary("data/NWL2020.txt");
    println!("Dictionary loaded!");
    let binary = wincode::serialize(&trie).unwrap();

    let mut file = match File::create("NWL2020.bin") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    let _ = file.write(&binary);
}


fn load_dictionary(file_path: &str) -> Trie {
    let mut trie = Trie::new();
    let file = File::open(file_path).expect("Could not open dictionary file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(word_str) = line {
            let mut word_u8: Vec<u8> = Vec::new();
            
            for ch in word_str.trim().to_uppercase().bytes() {
                let mapped_byte = ch - b'A';
                
                word_u8.push(mapped_byte);
            }
            
            trie.insert(word_u8);
        }
    }
    
    trie
}