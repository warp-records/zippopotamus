
use bitvec::prelude::*;
use crate::huffman::*;
use std::collections::HashMap;
use std::fs;
use bincode::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Zpp {
    huff_table: HashMap<char, (u16, u8)>,
    binary_data: Vec<u8>,
    remainder_bits: u8,
}

//just a test to compress a file using huffman coding
pub fn compress_file(filename: &str) {
    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        binary_data: Vec::new(),
        remainder_bits: 0,
    };

    let text = fs::read_to_string("data.txt").unwrap();
    let mut huff_coder = HuffmanTree::from_str(&text);

    let mut huff_table = huff_coder.gen_dict();

    let mut bit_stream = BitVec::<u8, Lsb0>::new();

    for ch in text.chars() {
        let (next_code, code_len) = *huff_table.get(&ch).unwrap();

        //push bits starting from the leftmost 1
        //to the LSB
        for i in 16-code_len..code_len {
            bit_stream.push(next_code>>i == 1);
        }

        //fill up the last byte in the vec with 0s
        zpp.remainder_bits = (bit_stream.len() % 8) as u8;
        for _ in 0..zpp.remainder_bits {
            bit_stream.push(false);
        }
    }

    zpp.huff_table = huff_table;
    zpp.binary_data = Vec::<u8>::from(bit_stream);

    let serialized = bincode::serialize(&zpp).expect("Failed to serialize Zpp");
    fs::write("data.zpp", serialized).expect("Failed to write compressed file");
}

/*
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
