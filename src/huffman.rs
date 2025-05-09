

//use home baked implementation in the future
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
//use caffeine::q::PriorityQ;

pub struct HuffmanTree {
    //represent as an implicit data structure
    tree: Vec<Node>,
    //index of the root element
    root_idx: usize,
    //<u8, (huffman code, len)>
    dict: HashMap<u8, (u16, u8)>,
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

    //Vec<(symbol, frequency)>
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

    pub fn gen_dict(&mut self) -> HashMap<u8, (u16, u8)> {
        self.limit_length();
        self.recurse(self.tree.len()-1, &mut Vec::new());
        std::mem::take(&mut self.dict)
    }

    fn limit_length(&mut self) {
        let mut depth = 1;
        let mut node = self.tree.first().unwrap();

        while let Some(children) = node.children {
            node = &self.tree[children.1];
            depth += 1;
        }

        //Rotate the tree left until it's only depth 15
        while depth > Self::MAX_LEN {
            self.rot_left();
            depth -= 1;
        }
    }

    fn rot_left(&mut self) {
        //Reassign the root node's parent and children
        let mut new_left = self.tree[self.root_idx];
        let (old_left_idx, new_root_idx) = new_left.children.unwrap();
        new_left.parent = Some(new_root_idx);
        let new_children = (old_left_idx, self.tree[new_root_idx].children.unwrap().0);
        new_left.children = Some(new_children);
        new_left.weight = self.tree[new_children.0].weight + self.tree[new_children.1].weight;

        //Do the same for the right node
        let mut new_root = self.tree[new_root_idx];
        new_root.parent = None;
        let new_children = (self.root_idx, new_root.children.unwrap().1);
        new_root.weight = self.tree[new_children.0].weight + self.tree[new_children.1].weight;
        new_root.children = Some(new_children);

        //update tree with new nodes
        self.tree[new_root_idx] = new_root;
        self.tree[self.root_idx] = new_left;
        self.root_idx = new_root_idx;
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
