
use bitvec::prelude::*;
use crate::huffman::*;
use std::collections::HashMap;
use std::fs;
///Use std::fmt::Error for error type because it's
///a convenient unit struct
use std::fmt::Error;
//use bincode::*;
use serde::{Serialize, Deserialize};

///Custom file format used for testing before implementing the PKware spec
#[derive(Serialize, Deserialize)]
struct Zpp {
    huff_table: HashMap<u8, (u16, u8)>,
    encoded: Vec<u8>,
    binary_len: usize,
}

//just a test to compress a file using huffman coding
pub fn compress_file(source: &str, dest: &str) -> Result<(), Error> {

    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        encoded: Vec::new(),
        binary_len: 0,
    };

    let text = fs::read_to_string(source).unwrap();
    let mut huff_coder = HuffmanTree::from_bytes(&text.as_bytes());

    let code_dict = huff_coder.gen_dict();
    let (encoded, len) = huff_encode(text.as_bytes(), &code_dict)?;

    zpp.huff_table = code_dict;
    zpp.encoded = encoded;
    zpp.binary_len = len;

    let serialized = bincode::serialize(&zpp).map_err(|_| Error)?;
    fs::write(dest, serialized).map_err(|_| Error)?;

    Ok(())
}

pub fn decompress_file(source: &str, dest: &str) -> Result<(), Error> {

    let serialized = fs::read(source).map_err(|_| Error)?;
    let zpp: Zpp = bincode::deserialize(&serialized[..]).map_err(|_| Error)?;

    let decoded = huff_decode(&zpp.encoded, &zpp.huff_table, zpp.binary_len).map_err(|_| Error)?;

    fs::write(dest, decoded).map_err(|_| Error)?;

    Ok(())
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
