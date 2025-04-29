
//use home baked implementation in the future
use std::collections::HashMap;
use caffeine::q::PriorityQ;

pub struct HuffmanTree {
    //represent as an implicit data structure
    tree: Vec<Node>,
    //<huff code, char>
    dict: HashMap<u16, char>,
}

#[derive(Clone, Copy, Default)]
struct Node {
    symbol: Option<char>,
    weight: usize,
    //indices of children if internal node
    children: Option<(usize, usize)>,
    parent: Option<usize>,
    idx: usize,
}

impl HuffmanTree {

    //Vec<(symbol, frequency)>
    pub fn new(elems: Vec<(char, usize)>) -> Self {
        let mut nodes = Vec::with_capacity(elems.len());
        //insert into priority queue based on frequency
        let mut pq = PriorityQ::new();
        for (i, elem) in elems.iter().enumerate() {

            let node = Node {
                symbol: Some(elem.0),
                weight: elem.1,
                children: None,
                parent: None,
                idx: i,
            };

            nodes.push(node);
            pq.push(node, node.weight);
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
            pq.push(parent, parent.weight);
        }

        Self { tree: nodes, dict: HashMap::new() }
    }

    pub fn gen_dict(&mut self) -> HashMap<u16, char> {
        self.recurse(self.tree.len()-1, &mut Vec::new());
        std::mem::take(&mut self.dict)
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
            self.dict.insert(code, self.tree[idx].symbol.expect("Expected symbol in this node"));
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

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
