
use bitvec::prelude::*;
use crate::huffman::*;
use std::collections::HashMap;
use std::fs;
use bincode::*;
use serde::{Serialize, Deserialize};

///Custom file format used for testing before implementing the PKware spec
#[derive(Serialize, Deserialize)]
struct Zpp {
    huff_table: HashMap<u8, (u16, u8)>,
    encoded: Vec<u8>,
    binary_len: usize,
}

//just a test to compress a file using huffman coding
pub fn compress_file(source: &str, dest: &str) {
    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        encoded: Vec::new(),
        binary_len: 0,
    };

    let text = fs::read_to_string(source).unwrap();
    let mut huff_coder = HuffmanTree::from_bytes(&text.as_bytes());

    let code_dict = huff_coder.gen_dict();
    let (encoded, len) = huff_encode(text.as_bytes(), &code_dict).expect("Failed to encode data");

    zpp.huff_table = code_dict;
    zpp.encoded = encoded;
    zpp.binary_len = len;

    let serialized = bincode::serialize(&zpp).expect("Failed to serialize Zpp");
    fs::write(dest, serialized).expect("Failed to write compressed file");
}

pub fn decompress_file(source: &str, dest: &str) {

    let serialized = fs::read(source).expect("Failed to read file");
    let mut zpp: Zpp = bincode::deserialize(&serialized[..]).expect("Failed to deserialize");

    let decoded = huff_decode(&zpp.encoded, &zpp.huff_table, zpp.binary_len).expect("Failed to decode data");

    fs::write(dest, decoded).expect("Failed to write file");
}


/*
Fuck I forgot what I wrote this shit for

use std::io::{Read, Write};

pub fn get_meta<R: Read>(reader: R) -> Meta {
    Meta {}
}

pub fn deflate<R: Read, W: Write>(reader: R, writer: W) ->
    Result<u64, DeflateError> {


}

pub enum DeflateError {
    Io(std::io::Error),
    //InvalidData(&str),
}


struct Meta {

}
//pub fn parse_meta() ->
*/
