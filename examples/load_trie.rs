#![allow(dead_code)]
use ramble::trie::Trie;

use std::fs;

fn main() {
    println!("Loading dictionary binary into memory");
    let binary = fs::read("data/bin/NWL2020.bin").expect("Could not open dictionary file");
    println!("Dictionary loaded into memory!");

    let trie: Trie = wincode::deserialize(&binary).unwrap();
    println!("Binary data deserialized into Trie!");

    let valid_word = "GINGERROOTS";
    let invalid_word = "RKIGUVBSRFK";
    let valid_array: Vec<u8> = valid_word.bytes().map(|c| c - b'A').collect();
    let invalid_array: Vec<u8> = invalid_word.bytes().map(|c| c - b'A').collect();
    
    println!("{} in trie = {}", valid_word, trie.contains(&valid_array));
    println!("{} in trie = {}", invalid_word, trie.contains(&invalid_array));
}
