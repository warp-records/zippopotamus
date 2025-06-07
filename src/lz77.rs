
use std::collections::HashMap;
use std::cmp::min;
//try using a trie to see if there's a speedup
//use qp_trie::Trie;

//Note: cannot exceed 256 until mathching offset has more bits
const WINDOW_LEN: usize = 256;

struct Match {
    len: u8,
    offset: u8,
    next_symbol: u8,
}

pub fn lz77_encode(source: &[u8]) -> Vec<u8> {
    let mut search_buf: &[u8] = &[];
    let mut lookahead_buf = &source[0..min(WINDOW_LEN, source.len())];

    let mut output = Vec::new();

    let mut cursor: usize = 0;

    while lookahead_buf.len() > 0 {

        for lb_symbol in lookahead_buf {
            let mut best_match = Match::with_symbol(source[0]);

            let mut match_len = 0;

            //iterate over the search buffer, and find the longest match
            //starting from our current position in the lookahead buffer
            for (sb_pos, sb_symbol) in search_buf.iter().enumerate() {

                //if symbols match increment match counter
                if sb_symbol == lb_symbol { match_len += 1; } else {

                    //check if our current match is the longest one, and
                    //update it as our best match if so
                    if match_len > best_match.len {
                        best_match.len = match_len;
                        best_match.next_symbol = lookahead_buf[match_len as usize];
                        best_match.offset = (search_buf.len() - sb_pos) as u8;
                    }

                    match_len = 0;
                }
            }

            //output best match as a (length:offset:next_symbol) tuple
            output.push(best_match.len);
            output.push(best_match.offset);
            output.push(best_match.next_symbol);
        }

        cursor += 1;
        //expand search buffer until it reaches size WINDOW_LEN, then shift to right
        search_buf = &source[cursor.saturating_sub(WINDOW_LEN)..cursor];
        //vice versa
        lookahead_buf = &source[cursor..min(WINDOW_LEN+cursor, source.len())];
    }

    output
}


impl Match {
    pub fn new() -> Self {
        Match { len: 0, offset: 0, next_symbol: 0 }
    }
    pub fn with_symbol(symbol: u8) -> Self {
        Match { len: 0, offset: 0, next_symbol: symbol }
    }
}

pub fn trie_encode(source: &[u8]) {

}
