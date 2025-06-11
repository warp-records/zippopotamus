
use std::collections::HashMap;
use std::cmp::min;
use std::fmt::Error;
//try using a trie to see if there's a speedup
//use qp_trie::Trie;

//Note: cannot exceed 256 until mathching offset has more bits
const WINDOW_LEN: usize = 256;

struct Match {
    len: u8,
    offset: u8,
    next_symbol: u8,
}

//TODO - support wrap around
pub fn lz77_encode(source: &[u8]) -> Vec<u8> {
    let mut search_buf: &[u8] = &[];
    let mut lookahead_buf = &source[0..min(WINDOW_LEN, source.len())];

    let mut output = Vec::new();

    //starting index of lookahead buffer
    let mut cursor: usize = 0;
    let mut sb_iter = search_buf.iter().enumerate().peekable();

    while lookahead_buf.len() > 0 {
        let mut best_match = Match::with_symbol(source[cursor]);

        while sb_iter.peek().is_some() {
            let mut inner_sb_iter = sb_iter.clone().cycle();
            let mut match_len = 0;

            //iterate over the lookahead buffer, and find the longest match
            //starting from our current position in the search buffer
            for (lb_pos, lb_symbol) in lookahead_buf.iter().enumerate() {
                let (sb_pos, sb_symbol) = inner_sb_iter.next().unwrap();

                //if symbols match and we're not on the last iteration increment match counter
                if sb_symbol == lb_symbol {
                    match_len += 1;
                }
                //if symbols are mismatched or the loop is ending,
                //check the match and restart counter
                if sb_symbol != lb_symbol || lb_pos == lookahead_buf.len()-1 {

                    //check if our current match is the longest one, and
                    //update it as our best match if so
                    if match_len > best_match.len {
                        best_match.len = match_len;
                        best_match.offset = (sb_pos - cursor.saturating_sub(WINDOW_LEN) + 1) as u8;
                        best_match.next_symbol = lookahead_buf[min(match_len as usize, lookahead_buf.len()-1)];
                    }

                    match_len = 0;
                }
            }

            sb_iter.next();
        }

        //output best match as a (length:offset:next_symbol) tuple
        output.push(best_match.len);
        output.push(best_match.offset);
        output.push(best_match.next_symbol);

        //advance by the length of the match we found
        cursor += best_match.len as usize + 1;

        if cursor >= source.len() { break; }
        //expand search buffer until it reaches size WINDOW_LEN, then shift to right
        search_buf = &source[cursor.saturating_sub(WINDOW_LEN)..cursor];
        //vice versa
        lookahead_buf = &source[cursor..min(WINDOW_LEN+cursor, source.len())];
        //update iterator to lookahead buffer
        sb_iter = search_buf.iter().enumerate().peekable();
    }

    output
}

/*
pub fn lz77_decode(source: &[u8]) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();

    let mut cursor: usize = 0;
    let mut iter = source.iter();

    //extract length, offset and next symbol
    while let Some(&len) = iter.next() {
        let len = len as usize;
        let offset = *iter.next().ok_or(Error)? as usize;
        let next_symbol = *iter.next().ok_or(Error)? as usize;


        let start = cursor - offset;
        let mut i = 0;
        for _ in start..start+len {
            output.push(source[i]);
            i += 1;

            //wraparound
            if i == cursor { i = 0; }
        }
    }

    Ok(output)
}
 */


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
