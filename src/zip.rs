
use bitvec::prelude::*;
use crate::huffman::*;
use std::collections::HashMap;
use std::fs;
use bincode::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Zpp {
    huff_table: HashMap<char, u16>,
    binary_data: Vec<u8>,
}

//just a test to compress a file using huffman coding
pub fn compress_file(filename: &str) {
    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        binary_data: Vec::new(),
    };

    let text = fs::read_to_string("data.txt").unwrap();
    let mut huff_coder = HuffmanTree::from_str(&text);

    let mut huff_table = huff_coder.gen_dict();

    let mut bit_stream = BitVec::<u8, Msb0>::new();

    for ch in text.chars() {
        let next_code = *huff_table.get(&ch).unwrap();

        if next_code == 0 {
            bit_stream.push(false);
        } else {
            let start_idx = 15 - next_code.trailing_zeros();
            for i in (start_idx..0).rev() {
                bit_stream.push(((next_code&(1 << i))>>i) != 0);
            }
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
