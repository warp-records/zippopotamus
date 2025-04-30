
use std::{env::args, fs, io::{BufRead, BufReader}};
use zippopotamus::huffman::*;
use webbrowser;

fn main() {
    //let _ = webbrowser::open("https://youtu.be/q86g1aop6a8");
    println!("Zippopotamus: version {}", env!("CARGO_PKG_VERSION"));

    let test_str = fs::read_to_string("data.txt").unwrap();
    let mut huffman_tree = HuffmanTree::from_str(&test_str);
    let code_dict = huffman_tree.gen_dict();

    /*for ch in test_str.chars() {
        println!("{}: {:b}", ch, *code_dict.get(&ch).unwrap());
    }*/


   println!("Done!");

   println!("Huffman code dictionary: ");
   for (ch, code) in code_dict.iter() {
       println!("{}: {:b}", ch, code);
   }
   //if let Ok(art) = fs::read_to_string("zipper.txt") {
    //    println!("{art}");
    //}

    //let filename = args().nth(1).expect("Expected file path as arugment");
    //let mut reader = BufReader::new(fs::File::open(filename).expect("Unable to open file {filename}"));
}
