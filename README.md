# GIF Parser library

Simple library to parse GIF.

## Example

```rust
use std::fs::File;
use std::io::BufReader;

fn main() {
    let filename = "tests/data/nayn.gif"
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let parser = GifParser::new(reader);

    for token in parser {
        let token = token.unwrap();
        println!("TOKEN: {token:?}");
    }
}

```
