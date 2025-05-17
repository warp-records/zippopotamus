

//use home baked implementation in the future
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use std::fmt::Error;
use std::process::Output;
//use caffeine::q::PriorityQ;
use bitvec::prelude::*;
use std::fs;
use serde::{Serialize, Deserialize};

///Store huffman codes using a Hashmap
///Map represented using the symbol as the key
///and a tuple containing the code, and the code length
///HashMap<symbol, (huffman code, len)>
pub type CodeDict = HashMap<u8, (u16, u8)>;

///Create a huffman tree
pub struct HuffmanTree {
    //represent as an implicit data structure
    tree: Vec<Node>,
    //index of the root element
    root_idx: usize,
    //only used during private function call which uses recursion
    dict: CodeDict,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
struct Node {
    symbol: Option<u8>,
    weight: usize,
    //indices of children if internal node
    children: Option<(usize, usize)>,
    parent: Option<usize>,
    idx: usize,
}

impl HuffmanTree {

    //Max length of each code in bits
    const MAX_LEN: u8 = 15;

    pub fn from_bytes(bytes: &[u8]) -> Self {

        let mut freq_map = HashMap::new();
        for byte in bytes {
            *freq_map.entry(*byte).or_insert(0) += 1;
        }
        let mut elems: Vec<(u8, usize)> = freq_map.into_iter().collect();
        //necessary to sort for stable tree generation
        elems.sort_by(|a, b| a.0.cmp(&b.0));
        Self::new(elems)
    }

    ///Returns Vec<(symbol, frequency)>
    pub fn new(elems: Vec<(u8, usize)>) -> Self {
        let mut nodes = Vec::with_capacity(elems.len());
        //insert into priority queue based on frequency
        let mut pq = BinaryHeap::new();
        for (i, elem) in elems.iter().enumerate() {

            let node = Node {
                symbol: Some(elem.0),
                weight: elem.1,
                children: None,
                parent: None,
                idx: i,
            };

            nodes.push(node);
            pq.push(Reverse(node));
        }

        while pq.len() > 1 {
            let first = pq.pop().unwrap().0;
            let second = pq.pop().unwrap().0;

            let parent = Node {
                symbol: None,
                weight: first.weight + second.weight,
                children: Some((first.idx, second.idx)),
                parent: None,
                idx: nodes.len(),
            };
            nodes[first.idx].parent = Some(parent.idx);
            nodes[second.idx].parent = Some(parent.idx);

            nodes.push(parent);
            pq.push(Reverse(parent));
        }

        Self { root_idx: nodes.len()-1, tree: nodes, dict: HashMap::new() }
    }

    pub fn gen_dict(&mut self) -> CodeDict {
        //self.limit_length();
        self.recurse(self.root_idx, &mut Vec::new());
        std::mem::take(&mut self.dict)
    }

    //doesn't work
    fn limit_length(&mut self) {

        //calculate the depth by repurposing the function
        //which recursively generates codes
        let mut bitstack = Vec::new();
        self.recurse(self.root_idx, &mut bitstack);
        let mut depth = self.dict.iter().max_by(|a, b| { a.1.1.cmp(&b.1.1).then(a.0.cmp(b.0)) }).unwrap().1.1;
        //recurse alters the dictionary
        self.dict.clear();
        println!("depth: {depth}");

    }

    //recurse down huffman tree in pre order, maintaining
    //a bitstack which corresponds to each symbol's code,
    //and generate a dictionary along the way
    fn recurse(&mut self, idx: usize, bitstack: &mut Vec<u8>) {

        //don't generate a code if we hit an internal node
        if self.tree[idx].symbol.is_some() {
            //generate huffman code
            let mut code: u16 = 0;
            for bit in bitstack.iter() {
                code <<= 1;
                code |= *bit as u16;
            }

            //embed into lookup table
            self.dict.insert(self.tree[idx].symbol.expect("Expected symbol in this node"),
                (code, bitstack.len() as u8));
        }

        //recurse tree pre order
        if let Some((left, right)) = self.tree[idx].children {
            bitstack.push(0);
            self.recurse(left, bitstack);
            bitstack.pop();

            bitstack.push(1);
            self.recurse(right, bitstack);
            bitstack.pop();
        }
    }
}

//is this right?
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //must have a way to represent a consistent order
        //in the case of two characters having the same weight
        self.weight.partial_cmp(&other.weight)
            .and_then(|ord| match ord {
                std::cmp::Ordering::Equal => self.symbol.partial_cmp(&other.symbol),
                _ => Some(ord),
            })
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
            .then(self.symbol.cmp(&other.symbol))
    }
}


#[derive(Serialize, Deserialize)]
struct Zpp {
    huff_table: CodeDict,
    binary_data: Vec<u8>,
    binary_len: usize,
}

///Convert a slice of unencoded bytes into a Vec of encoded bytes
///Returns (Vec of encoded data, length in bits)
pub fn huff_encode(source: &[u8], dict: &CodeDict) -> Result<(Vec<u8>, usize), Error> {
    let mut zpp = Zpp {
        huff_table: HashMap::new(),
        binary_data: Vec::new(),
        binary_len: 0,
    };

    let mut bit_stream = BitVec::<u8, Lsb0>::new();
    let mut real_length: usize = 0;

    for byte in source {
        if let Some(&(next_code, code_len)) = dict.get(&byte) {
            //push bits starting from the leftmost bit
            //to the rightmost
            for i in (0..code_len).rev() {
                bit_stream.push((next_code>>i)&0b1 == 1);
                real_length += 1;
            }

        //error when symol not found in dict
        } else { return Err(Error) }
    }

    Ok((Vec::<u8>::from(bit_stream), real_length))
}

///Takes a huffman encoded slice of bytes, a dictionary, and a length
///Returns a decoded stream of bytes
pub fn huff_decode(source: &[u8], dict: &CodeDict, bit_len: usize) -> Result<Vec<u8>, Error> {

    let mut bit_stream: BitVec<u8, Lsb0> = BitVec::from_vec(source.to_vec());

    let mut output = Vec::new();

    //converts the symbol to code map into a
    //code to symbol map
    //benched with flamegraph and this doesn't take up much time
    let inverted_code_dict: HashMap<(u16, u8), u8> = dict.iter()
                                            .map(|(&k, &v)| { (v, k) }).collect();

    let mut next_code: u16 = 0;
    let mut code_len: u8 = 0;
    for i in 0..bit_len {
        next_code <<= 1;
        next_code |= bit_stream[i] as u16;
        code_len += 1;

        if let Some(symbol) = inverted_code_dict.get(&(next_code, code_len)) {
            output.push(*symbol);
            next_code = 0;
            code_len = 0;

        //error when code not found in dict
        } else { return Err(Error) }
    }

    Ok(output)
}
