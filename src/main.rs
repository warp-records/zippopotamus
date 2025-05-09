
use zippopotamus::zip::*;
use std::{env::args, fs, io::{BufRead, BufReader}};
use zippopotamus::{huffman::*, zip::compress_file};
use webbrowser;

fn main() {
    //let _ = webbrowser::open("https://youtu.be/q86g1aop6a8");
    println!("Zippopotamus: version {}", env!("CARGO_PKG_VERSION"));

    compress_file("test_data/philosophers_stone.txt", "test_data/compressed.zpp");
    decompress_file("test_data/compressed.zpp", "test_data/decompressed.txt");

    /*
    let test_str = fs::read_to_string("data.txt").unwrap();
    let mut huffman_tree = HuffmanTree::from_str(&test_str);
    let code_dict = huffman_tree.gen_dict();

    println!("Done!");

    println!("Huffman code dictionary: ");
    let mut len_sorted = code_dict.iter()
        .collect::<Vec<_>>();

    //sort by code length
    len_sorted.sort_by(|a, b| { a.1.1.cmp(&b.1.1).then(a.0.cmp(b.0)) });

    for (ch, code) in len_sorted {
        println!("{}: {:0width$b}", ch, code.0, width = code.1 as usize);
    } */


    //if let Ok(art) = fs::read_to_string("zipper.txt") {
    //    println!("{art}");
    //}

    //let filename = args().nth(1).expect("Expected file path as arugment");
    //let mut reader = BufReader::new(fs::File::open(filename).expect("Unable to open file {filename}"));
}
