
use std::fs;

use webbrowser;

fn main() {
    let _ = webbrowser::open("https://youtu.be/q86g1aop6a8");
    println!("Zippopotamus: version {}", env!("CARGO_PKG_VERSION"));

    if let Ok(art) = fs::read_to_string("zipper.txt") {
        println!("{art}");
    }
}
