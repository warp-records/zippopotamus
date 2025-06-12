
use zippopotamus::zip::*;
use std::{env::args, fs, io::{BufRead, BufReader}};
use zippopotamus::{huffman::*, lz77::*, zip::compress_file};

fn main() {
    //let _ = webbrowser::open("https://youtu.be/q86g1aop6a8");
    println!("Zippopotamus: version {}", env!("CARGO_PKG_VERSION"));

    //compress_file("test_data/data.txt", "test_data/compressed.zpp");
    //decompress_file("test_data/compressed.zpp", "test_data/decompressed.txt");

    let vec: Vec<u8> = "hello world this is a test".as_bytes().to_vec();
    let encoded = lz77_encode(&vec);
    println!("{:?}", encoded);

    let decoded = String::from_utf8(lz77_decode(&encoded).unwrap());
    println!("\nDecoded: {:?}", decoded);
    return;

    //let selection = args().nth(1)
    let filename = args().nth(1).unwrap_or("data.txt".to_string());

    //let compressedname = if let Some(dot_pos) = filename.rfind('.') {
    //     format!("{}.zpp", &filename[..dot_pos])
    // } else {
    //     format!("{}.zpp", filename)
    //};
    compress_file(&filename, "compressed.zpp").expect("Failed to compress file. Does the file exist?");
    decompress_file("compressed.zpp", "decompressed.zpp").expect("Failed to decompress file");

    println!("Done!");

    if let Ok(art) = fs::read_to_string("zipper.txt") {
        println!("{art}");
    }

}


pub fn print_codes(dict: CodeDict) {
    println!("Huffman code dictionary: ");
    let mut len_sorted = dict.iter()
        .collect::<Vec<_>>();

    //sort by code length
    len_sorted.sort_by(|a, b| { a.1.1.cmp(&b.1.1).then(a.0.cmp(b.0)) });

    for (ch, code) in len_sorted {
        println!("{}: {:0width$b}", char::from_u32(*ch as u32).unwrap_or('?'), code.0, width = code.1 as usize);
    }

}
