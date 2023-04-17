mod error;
mod parser;
mod read_ext;
mod types;
pub use parser::{GifParser, GifParts};

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use crate::*;

    #[test]
    pub fn simple() {
        let file = File::open("tests/data/nyan.gif").unwrap();
        let reader = BufReader::new(file);

        let parser = GifParser::new(reader);

        for token in parser {
            let token = token.unwrap();
            println!("TOKEN: {token:?}");
        }
    }
}
