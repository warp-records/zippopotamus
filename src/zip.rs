
use bitvec::prelude::*;
use crate::huffman::*;
use std::collections::HashMap;
use std::fs;
use bincode::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Zpp {
    huff_table: HashMap<char, (u16, u8)>,
    binary_data: Vec<u8>,
    binary_len: usize,
}

//just a test to compress a file using huffman coding
pub fn compress_file(source: &str, dest: &str) {
    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        binary_data: Vec::new(),
        binary_len: 0,
    };

    let text = fs::read_to_string(source).unwrap();
    let mut huff_coder = HuffmanTree::from_str(&text);

    let mut huff_table = huff_coder.gen_dict();

    let mut bit_stream = BitVec::<u8, Lsb0>::new();
    let mut real_length = 0;

    for ch in text.chars() {
        let (next_code, code_len) = *huff_table.get(&ch).unwrap();

        //push bits starting from the leftmost bit
        //to the rightmost
        for i in (0..code_len).rev() {
            bit_stream.push((next_code>>i)&0b1 == 1);
            real_length += 1;
        }
    }

    zpp.huff_table = huff_table;
    zpp.binary_data = Vec::<u8>::from(bit_stream);
    zpp.binary_len = real_length;

    let serialized = bincode::serialize(&zpp).expect("Failed to serialize Zpp");
    fs::write(dest, serialized).expect("Failed to write compressed file");
}

pub fn decompress_file(source: &str, dest: &str) {

    let serialized = fs::read(source).expect("Failed to read file");
    let mut zpp: Zpp = bincode::deserialize(&serialized[..]).expect("Failed to deserialize");
    let mut bit_stream: BitVec<u8, Lsb0> = BitVec::from_vec(zpp.binary_data);

    let mut output = String::new();

    //converts the symbol to code map into a
    //code to symbol map
    let inverted_code_dict: HashMap<(u16, u8), char> = zpp.huff_table.iter()
                                            .map(|(&k, &v)| { (v, k) }).collect();

    let mut next_code: u16 = 0;
    let mut code_len: u8 = 0;
    for i in 0..zpp.binary_len {
        next_code <<= 1;
        next_code |= bit_stream[i] as u16;
        code_len += 1;

        if let Some(symbol) = inverted_code_dict.get(&(next_code, code_len)) {
            output.push(*symbol);
            next_code = 0;
            code_len = 0;
        }
    }

    fs::write(dest, output);
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
